#!/usr/bin/env python3
"""Run opt-in Rust tests against a checksum-pinned, disposable OpenCode server."""

import argparse
import base64
import hashlib
import json
import os
from pathlib import Path
import platform
import queue
import secrets
import shutil
import signal
import subprocess
import tarfile
import tempfile
import threading
from urllib.parse import urlsplit
from urllib.request import urlopen
import zipfile

REPO = Path(__file__).resolve().parents[2]
MANIFEST = Path(__file__).with_name("server.json")


def server_environment(root, password):
    # Deliberately do not inherit provider keys, proxies, plugins, or OpenCode config.
    env = {"PATH": os.defpath, "LANG": "en_US.UTF-8"}
    for key, name in {
        "OPENCODE_TEST_HOME": "home",
        "XDG_DATA_HOME": "data",
        "XDG_CONFIG_HOME": "config",
        "XDG_CACHE_HOME": "cache",
        "XDG_STATE_HOME": "state",
        "TMPDIR": "tmp",
    }.items():
        directory = root / name
        directory.mkdir(exist_ok=True)
        env[key] = str(directory)
    env.update({
        "TMP": env["TMPDIR"],
        "TEMP": env["TMPDIR"],
        "OPENCODE_CONFIG_DIR": env["XDG_CONFIG_HOME"],
        "OPENCODE_DB": str(root / "server.sqlite"),
        "OPENCODE_SERVER_PASSWORD": password,
        "OPENCODE_DISABLE_MODELS_FETCH": "1",
        "OPENCODE_DISABLE_PROJECT_CONFIG": "1",
        "OPENCODE_DISABLE_CLAUDE_CODE": "1",
        "OPENCODE_DISABLE_FILEWATCHER": "1",
        "OPENCODE_DISABLE_FFF": "1",
        "GIT_CONFIG_NOSYSTEM": "1",
        "GIT_CONFIG_GLOBAL": os.devnull,
    })
    return env


def verify_archive(archive, expected):
    digest = hashlib.sha256()
    with archive.open("rb") as source:
        for chunk in iter(lambda: source.read(1024 * 1024), b""):
            digest.update(chunk)
    if digest.hexdigest() != expected:
        raise RuntimeError(f"SHA-256 mismatch for {archive}; remove it and retry")


def extract_binary(archive, executable):
    # Never extract archive paths, links, permissions, or unrelated files.
    executable.parent.mkdir(parents=True, exist_ok=True)
    if archive.suffix == ".zip":
        with zipfile.ZipFile(archive) as bundle:
            with bundle.open("opencode") as source, executable.open("wb") as dest:
                shutil.copyfileobj(source, dest)
    else:
        with tarfile.open(archive, "r:gz") as bundle:
            member = bundle.getmember("opencode")
            if not member.isfile():
                raise RuntimeError("release executable is not a regular file")
            with bundle.extractfile(member) as source, executable.open("wb") as dest:
                shutil.copyfileobj(source, dest)
    executable.chmod(0o700)


def download(manifest):
    key = f"{platform.system()}-{platform.machine()}"
    if key not in manifest["assets"]:
        raise RuntimeError(f"no pinned asset for {key}; supported: {', '.join(manifest['assets'])}")
    asset = manifest["assets"][key]
    cache = REPO / "target" / "live" / manifest["tag"] / key
    cache.mkdir(parents=True, exist_ok=True)
    archive = cache / asset["url"].rsplit("/", 1)[1]
    if not archive.exists():
        print(f"Downloading OpenCode {manifest['tag']} for {key}...", flush=True)
        with tempfile.NamedTemporaryFile(dir=cache, delete=False) as dest:
            partial = Path(dest.name)
            try:
                with urlopen(asset["url"], timeout=120) as source:
                    shutil.copyfileobj(source, dest)
                dest.close()
                verify_archive(partial, asset["sha256"])
                partial.replace(archive)
            finally:
                partial.unlink(missing_ok=True)
    verify_archive(archive, asset["sha256"])
    return archive


def startup_url(line):
    try:
        value = json.loads(line)
    except json.JSONDecodeError:
        return None
    if not isinstance(value, dict) or "url" not in value:
        return None
    parsed = urlsplit(value["url"])
    if (parsed.scheme != "http" or parsed.hostname != "127.0.0.1"
            or not parsed.port or parsed.username is not None or parsed.password is not None
            or parsed.path or parsed.query or parsed.fragment):
        raise ValueError("server reported an unexpected origin")
    return value["url"]


def await_startup(process):
    lines = queue.Queue()

    def read():
        for line in process.stdout:
            # Startup output is consumed, never forwarded (it might contain secrets).
            if line.startswith('{"url":'):
                lines.put(line)
        lines.put(None)

    threading.Thread(target=read, daemon=True).start()
    try:
        line = lines.get(timeout=90)
    except queue.Empty as error:
        raise RuntimeError("server startup timed out") from error
    if line is None:
        raise RuntimeError(f"server exited during startup (status {process.poll()})")
    return startup_url(line)


def stop(process):
    process.stdin.close()  # serve --stdio exits when its owner closes stdin.
    try:
        process.wait(timeout=10)
    except subprocess.TimeoutExpired:
        process.terminate()
        try:
            process.wait(timeout=5)
        except subprocess.TimeoutExpired:
            process.kill()
            process.wait()
    process.stdout.close()


def interrupted(*_):
    raise KeyboardInterrupt


def run(manifest, archive, websocket=False):
    # Canonicalize macOS /var -> /private/var before comparing server paths.
    with tempfile.TemporaryDirectory(prefix="opencoders-live-") as temp:
        root = Path(temp).resolve()
        executable = root / "bin" / "opencode"
        extract_binary(archive, executable)
        workspace = root / "workspace + unicode 雪"
        workspace.mkdir()
        (root / "other-workspace").mkdir()
        password = secrets.token_urlsafe(32)
        env = server_environment(root, password)
        with (root / "server.log").open("w") as log:
            process = subprocess.Popen(
                [str(executable), "serve", "--stdio", "--hostname", "127.0.0.1", "--port", "0"],
                cwd=workspace, env=env, stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                stderr=log, text=True, start_new_session=True,
            )
            try:
                (root / "fixture-owner.json").write_text(json.dumps({"pid": process.pid}))
                url = await_startup(process)
                print(f"Testing OpenCode {manifest['version']} at {url} (isolated data)", flush=True)
                test_env = os.environ.copy()
                test_env.update({
                    "OPENCODERS_LIVE_URL": url,
                    "OPENCODERS_LIVE_AUTH": "Basic " + base64.b64encode(f"opencode:{password}".encode()).decode(),
                    "OPENCODERS_LIVE_PASSWORD": password,
                    "OPENCODERS_LIVE_ROOT": str(root),
                    "OPENCODERS_LIVE_PID": str(process.pid),
                    "OPENCODERS_LIVE_VERSION": manifest["version"],
                })
                command = ["cargo", "test", "--locked", "--test", "live"]
                if websocket:
                    command += ["--features", "websocket"]
                result = subprocess.run(
                    command + ["--", "--ignored", "--test-threads=1"],
                    cwd=REPO, env=test_env, timeout=180,
                )
                return result.returncode
            finally:
                stop(process)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--download-only", action="store_true")
    parser.add_argument("--websocket", action="store_true", help="also verify ordinary PTY WebSocket I/O")
    args = parser.parse_args()
    # Ensure normal termination (including CI cancellation) enters cleanup.
    signal.signal(signal.SIGTERM, interrupted)
    manifest = json.loads(MANIFEST.read_text())
    archive = download(manifest)
    return 0 if args.download_only else run(manifest, archive, args.websocket)


if __name__ == "__main__":
    raise SystemExit(main())
