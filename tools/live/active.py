#!/usr/bin/env python3
"""Opt-in active execution against a pinned server and scripted localhost provider."""

from contextlib import contextmanager
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
import json
import os
from pathlib import Path
import secrets
import select
import signal
import subprocess
import tempfile
import threading
import time

import run as smoke

SCENARIOS = ("output", "interrupt", "failure", "permission", "question", "cancel", "recovery", "recovery_question")
FILE_TEXT = "fixture-file-content"


def environment(root, password, origin, token):
    # Reuse the smoke runner's allowlist; never inherit credentials/config/plugins.
    if smoke.startup_url(json.dumps({"url": origin})) != origin:
        raise ValueError("invalid fixture origin")
    env = smoke.server_environment(root, password)
    env["OPENCODE_CONFIG_CONTENT"] = json.dumps({
        "model": "fixture/output", "default_agent": "fixture",
        "update": "disable", "share": "disabled", "snapshots": False,
        "warming": False, "compaction": {"auto": False},
        "permissions": [
            {"action": "*", "resource": "*", "effect": "deny"},
            {"action": "read", "resource": "fixture.txt", "effect": "ask"},
            {"action": "question", "resource": "*", "effect": "allow"},
        ],
        "policies": [
            {"action": "provider.use", "resource": "*", "effect": "deny"},
            {"action": "provider.use", "resource": "fixture", "effect": "allow"},
        ],
        "agents": {
            "fixture": {"mode": "primary", "steps": 3, "system": "Local compatibility fixture."},
            "title": {"disabled": True},
        },
        "providers": {"fixture": {
            "name": "Local compatibility fixture", "env": [],
            "package": "@opencode/ai/providers/openai-compatible",
            "settings": {"baseURL": origin + "/v1", "apiKey": token, "timeout": 60000},
            "models": {name: {
                "name": name, "type": "chat", "limit": {"context": 32000, "output": 512},
                "capabilities": {"input": ["text"], "output": ["text"], "tools": True},
            } for name in SCENARIOS},
        }},
    })
    return env


def plan(body, count):
    """Reject unexpected/repeated calls; never improvise a model or tool response."""
    scenario = body.get("model")
    if scenario not in SCENARIOS or body.get("stream") is not True:
        raise ValueError("unexpected model or non-streaming request")
    messages = body.get("messages")
    if not isinstance(messages, list) or not messages:
        raise ValueError("missing conversation")
    if count == 2 and scenario in {"permission", "question", "cancel", "recovery_question"}:
        results = [item for item in messages if item.get("role") == "tool"]
        marker = {"permission": FILE_TEXT, "question": "fixture-answer", "cancel": "fixture-feedback",
                  "recovery_question": "fixture-answer"}[scenario]
        if (len(results) != 1 or results[0].get("tool_call_id") != "call_" + scenario
                or marker not in json.dumps(results[0].get("content"))):
            raise ValueError("missing expected tool result")
        return {"last": scenario + " resumed"}
    if count != 1 or any(item.get("role") == "tool" for item in messages):
        raise ValueError("unexpected extra provider request")
    if scenario == "failure":
        return {"error": "fixture provider rejected request"}
    if scenario == "recovery":
        return {"segments": [
            {"text": "before ", "gate": "recovery/missed"},
            {"text": "missed ", "gate": "recovery/after"},
            {"text": "after", "gate": "recovery/end"},
        ], "last": ""}
    if scenario in {"output", "interrupt"}:
        return {"first": "first ", "last": "second", "hold": True}
    name = "read" if scenario == "permission" else "question"
    tools = {tool.get("function", {}).get("name") for tool in body.get("tools", [])}
    if name not in tools:
        raise ValueError("fixture tool not advertised")
    arguments = {"path": "fixture.txt"} if name == "read" else {"questions": [{
        "header": "Fixture", "question": "Choose the fixture answer.",
        "options": [{"label": "fixture-answer", "description": "Deterministic test answer"}],
    }]}
    return {"tool": name, "arguments": arguments}


def chunk(delta, finish=None):
    return ("data: " + json.dumps({
        "id": "fixture-completion", "object": "chat.completion.chunk", "created": 0,
        "choices": [{"index": 0, "delta": delta, "finish_reason": finish}],
    }) + "\n\n").encode()


class Fixture(ThreadingHTTPServer):
    # Every streaming handler has a deadline and is released at teardown.
    daemon_threads = True

    def __init__(self, token):
        super().__init__(("127.0.0.1", 0), Handler)
        self.token = token
        self.lock = threading.Lock()
        self.counts = {}
        self.aborted = []
        self.errors = []
        self.gates = {name: threading.Event() for name in [
            "output", "interrupt", "recovery/missed", "recovery/after", "recovery/end",
        ]}

    def snapshot(self):
        with self.lock:
            return {"requests": dict(self.counts), "aborted": list(self.aborted), "errors": list(self.errors)}


class Handler(BaseHTTPRequestHandler):
    def log_message(self, *_):
        pass  # Never log authorization headers or conversation bodies.

    def setup(self):
        super().setup()
        self.connection.settimeout(5)

    def authorized(self):
        if self.headers.get("Authorization") != "Bearer " + self.server.token:
            self.reply(401, {"error": "unauthorized"})
            return False
        return True

    def reply(self, status, body):
        data = json.dumps(body).encode()
        self.send_response(status)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(data)))
        self.end_headers()
        self.wfile.write(data)

    def do_GET(self):
        if not self.authorized():
            return
        if self.path == "/state":
            self.reply(200, self.server.snapshot())
        else:
            self.reply(404, {"error": "unknown fixture route"})

    def do_POST(self):
        if not self.authorized():
            return
        gate = self.path.removeprefix("/release/")
        if self.path.startswith("/release/") and gate in self.server.gates and gate != "interrupt":
            self.server.gates[gate].set()
            self.reply(200, {})
            return
        if self.path != "/v1/chat/completions":
            self.reply(404, {"error": "unknown fixture route"})
            return
        try:
            length = int(self.headers.get("Content-Length", "0"))
            if not 0 < length <= 1024 * 1024:
                raise ValueError("invalid provider body length")
            body = json.loads(self.rfile.read(length))
            scenario = body.get("model")
            with self.server.lock:
                count = self.server.counts.get(scenario, 0) + 1
                self.server.counts[scenario] = count
            response = plan(body, count)
        except (ValueError, TypeError, AttributeError) as error:
            # Error messages are fixture-controlled and do not include request bodies.
            with self.server.lock:
                self.server.errors.append(type(error).__name__)
            self.reply(400, {"error": {"message": "unexpected fixture request"}})
            return
        if "error" in response:
            self.reply(400, {"error": {"message": response["error"], "type": "invalid_request_error"}})
            return
        self.send_response(200)
        self.send_header("Content-Type", "text/event-stream")
        self.send_header("Connection", "close")
        self.end_headers()
        try:
            segments = response.get("segments", [])
            if response.get("hold"):
                segments = [{"text": response["first"], "gate": scenario}]
            for segment in segments:
                self.wfile.write(chunk({"role": "assistant", "content": segment["text"]}))
                self.wfile.flush()
                deadline = time.monotonic() + 45
                while not self.server.gates[segment["gate"]].wait(0.02):
                    readable, _, _ = select.select([self.connection], [], [], 0)
                    if readable and not self.connection.recv(1):
                        raise ConnectionResetError
                    if time.monotonic() > deadline:
                        with self.server.lock:
                            self.server.errors.append("stream gate timed out")
                        return
            if "tool" in response:
                self.wfile.write(chunk({"role": "assistant", "tool_calls": [{
                    "index": 0, "id": "call_" + scenario, "type": "function",
                    "function": {"name": response["tool"], "arguments": json.dumps(response["arguments"])},
                }]}))
                self.wfile.write(chunk({}, "tool_calls"))
            else:
                if response["last"]:
                    self.wfile.write(chunk({"content": response["last"]}))
                self.wfile.write(chunk({}, "stop"))
            self.wfile.write(b'data: {"choices":[],"usage":{"prompt_tokens":10,"completion_tokens":2,"total_tokens":12}}\n\n')
            self.wfile.write(b"data: [DONE]\n\n")
            self.wfile.flush()
        except (BrokenPipeError, ConnectionResetError):
            with self.server.lock:
                self.server.aborted.append(scenario)


@contextmanager
def provider(token):
    server = Fixture(token)
    thread = threading.Thread(target=server.serve_forever, daemon=True)
    thread.start()
    try:
        yield server, f"http://127.0.0.1:{server.server_port}"
    finally:
        for gate in server.gates.values():
            gate.set()
        server.shutdown()
        server.server_close()
        thread.join(timeout=5)


def run(manifest, archive):
    with tempfile.TemporaryDirectory(prefix="opencoders-active-") as temp, provider(secrets.token_urlsafe(32)) as (fixture, origin):
        root = Path(temp).resolve()
        executable = root / "bin" / "opencode"
        smoke.extract_binary(archive, executable)
        workspace = root / "workspace"
        workspace.mkdir()
        (workspace / "fixture.txt").write_text(FILE_TEXT + "\n")
        password = secrets.token_urlsafe(32)
        env = environment(root, password, origin, fixture.token)
        with (root / "server.log").open("w") as log:
            process = subprocess.Popen(
                [str(executable), "serve", "--stdio", "--hostname", "127.0.0.1", "--port", "0"],
                cwd=workspace, env=env, stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                stderr=log, text=True, start_new_session=True,
            )
            try:
                url = smoke.await_startup(process)
                print(f"Active fixture: OpenCode {manifest['version']} at {url}; scripted localhost provider", flush=True)
                test_env = os.environ.copy()
                test_env.update({
                    "OPENCODERS_LIVE_URL": url, "OPENCODERS_LIVE_PASSWORD": password,
                    "OPENCODERS_LIVE_ROOT": str(root), "OPENCODERS_LIVE_PID": str(process.pid),
                    "OPENCODERS_LIVE_VERSION": manifest["version"],
                    "OPENCODERS_FIXTURE_URL": origin, "OPENCODERS_FIXTURE_TOKEN": fixture.token,
                })
                result = subprocess.run(
                    ["cargo", "test", "--locked", "--test", "live_active", "--", "--ignored", "--test-threads=1", "--nocapture"],
                    cwd=smoke.REPO, env=test_env, timeout=180,
                )
                if fixture.snapshot()["errors"]:
                    raise RuntimeError("provider fixture rejected a request or timed out")
                return result.returncode
            finally:
                smoke.stop(process)


def main():
    signal.signal(signal.SIGTERM, smoke.interrupted)
    manifest = json.loads(smoke.MANIFEST.read_text())
    return run(manifest, smoke.download(manifest))


if __name__ == "__main__":
    raise SystemExit(main())
