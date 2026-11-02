"""Deterministic checks for the disposable live-test runner (no server/download)."""

import hashlib
import io
from pathlib import Path
import tempfile
import tarfile
import unittest
from unittest.mock import Mock, patch
import zipfile

import run


class RunnerTests(unittest.TestCase):
    def test_environment_excludes_user_credentials_and_redirects_state(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp).resolve()
            with patch.dict(run.os.environ, {"ANTHROPIC_API_KEY": "secret", "OPENCODE_CONFIG": "user.json"}):
                env = run.server_environment(root, "test-password")
            self.assertNotIn("ANTHROPIC_API_KEY", env)
            self.assertNotIn("OPENCODE_CONFIG", env)
            self.assertNotIn("HOME", env)
            for key in ("XDG_DATA_HOME", "XDG_CONFIG_HOME", "XDG_CACHE_HOME", "XDG_STATE_HOME", "OPENCODE_TEST_HOME", "TMPDIR"):
                self.assertTrue(Path(env[key]).is_relative_to(root))
            self.assertEqual(env["OPENCODE_SERVER_PASSWORD"], "test-password")
            self.assertEqual(env["OPENCODE_DISABLE_MODELS_FETCH"], "1")

    def test_corrupt_archive_is_rejected_before_extraction(self):
        with tempfile.TemporaryDirectory() as tmp:
            archive = Path(tmp) / "server.zip"
            archive.write_bytes(b"corrupt")
            with self.assertRaisesRegex(RuntimeError, "SHA-256"):
                run.verify_archive(archive, "0" * 64)

    def test_extracts_only_executable_without_following_archive_paths(self):
        with tempfile.TemporaryDirectory() as tmp:
            archive = Path(tmp) / "server.zip"
            with zipfile.ZipFile(archive, "w") as z:
                z.writestr("../../escape", b"bad")
                z.writestr("opencode", b"binary")
            run.verify_archive(archive, hashlib.sha256(archive.read_bytes()).hexdigest())
            executable = Path(tmp) / "bin" / "opencode"
            run.extract_binary(archive, executable)
            self.assertEqual(executable.read_bytes(), b"binary")
            self.assertEqual(list(executable.parent.iterdir()), [executable])

    def test_startup_accepts_only_an_ephemeral_ipv4_loopback_origin(self):
        self.assertEqual(run.startup_url('{"url":"http://127.0.0.1:12345"}'), "http://127.0.0.1:12345")
        for url in ("http://example.com:12345", "http://127.0.0.1", "http://127.0.0.1:0", "http://127.0.0.1:12345/api", "http://user@127.0.0.1:12345"):
            with self.subTest(url=url), self.assertRaises(ValueError):
                run.startup_url('{"url":"' + url + '"}')
        self.assertIsNone(run.startup_url("a startup log line"))

    def test_tar_binary_extraction_rejects_links(self):
        with tempfile.TemporaryDirectory() as tmp:
            archive = Path(tmp) / "server.tar.gz"
            executable = Path(tmp) / "bin" / "opencode"
            with tarfile.open(archive, "w:gz") as bundle:
                member = tarfile.TarInfo("opencode")
                member.size = 6
                bundle.addfile(member, io.BytesIO(b"binary"))
            run.extract_binary(archive, executable)
            self.assertEqual(executable.read_bytes(), b"binary")
            with tarfile.open(archive, "w:gz") as bundle:
                member = tarfile.TarInfo("opencode")
                member.type = tarfile.SYMTYPE
                member.linkname = "../../outside"
                bundle.addfile(member)
            with self.assertRaisesRegex(RuntimeError, "regular file"):
                run.extract_binary(archive, executable)

    def test_shutdown_closes_stdin_and_escalates_only_after_timeouts(self):
        process = Mock()
        process.wait.side_effect = [run.subprocess.TimeoutExpired("server", 10),
                                    run.subprocess.TimeoutExpired("server", 5), 0]
        run.stop(process)
        process.stdin.close.assert_called_once()
        process.terminate.assert_called_once()
        process.kill.assert_called_once()
        self.assertEqual(process.wait.call_count, 3)
        process.stdout.close.assert_called_once()


if __name__ == "__main__":
    unittest.main()
