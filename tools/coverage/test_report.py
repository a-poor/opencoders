"""Regression tests for coverage drift and misleading inventory evidence."""
import copy
import hashlib
import json
from pathlib import Path
import tempfile
import unittest

import report


class CoverageTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.spec = {"paths": {"/api/ping": {"get": {"operationId": "ping.get"}},
                               "/api/pong": {"post": {"operationId": "pong.post"}}}}
        self.data = {
            "format_version": 1, "schema_sha256": "",
            "baseline": {"version": "2.2.5", "commit": "a" * 40,
                         "verified_on": "2026-11-02", "platform": "macOS arm64"},
            "evidence": {"fixture": {
                "live_scope": "A disposable fixture returned a ping.",
                "source_reviewed": ["Server behavior outside the fixture."],
                "unverified": ["Concurrent requests."],
                "live_tests": [{"file": "tests/live.rs", "anchor": "fn verify(", "label": "verify"}],
                "examples": ["examples/ping.rs"],
            }},
            "operations": {
                "ping.get": {"method": "GET", "path": "/api/ping", "status": "supported",
                    "rust": {"file": "src/ping.rs", "anchor": '"ping.get"', "label": "Ping::get"},
                    "protocol_tests": [{"file": "tests/ping.rs", "anchor": "fn ping_works(", "label": "ping_works"}],
                    "evidence": "fixture"},
                "pong.post": {"method": "POST", "path": "/api/pong", "status": "planned",
                    "milestone": 3, "reason": "Terminal operations."},
            },
        }
        for name, text in {
            "src/ping.rs": 'pub async fn get() { send("ping.get"); }\n',
            "tests/ping.rs": '#[test]\nfn ping_works() {}\n',
            "tests/live.rs": 'fn verify() {}\n',
            "examples/ping.rs": 'fn main() {}\n',
            "tools/live/README.md": '# Live notes\n',
        }.items():
            path = self.root / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(text)
        self.save()

    def save(self):
        spec_bytes = json.dumps(self.spec).encode()
        digest = hashlib.sha256(spec_bytes).hexdigest()
        self.data["schema_sha256"] = digest
        files = {
            "openapi/opencode-v2.json": spec_bytes.decode(),
            "openapi/coverage.json": json.dumps(self.data),
            "openapi/provenance.json": json.dumps({"sha256": digest, "operations": 2}),
            "tools/live/server.json": json.dumps({"version": "2.2.5", "commit": "a" * 40}),
        }
        for name, text in files.items():
            path = self.root / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(text)

    def invalid(self, data=None):
        with self.assertRaises(report.CoverageError):
            report.validate(self.root, self.spec, data or self.data)

    def test_new_removed_duplicate_and_missing_operation_ids_require_review(self):
        original = copy.deepcopy(self.spec)
        for operation in [{"operationId": "new.get"}, {"operationId": "ping.get"}, {}]:
            self.spec = copy.deepcopy(original)
            self.spec["paths"]["/new"] = {"get": operation}
            self.invalid()
        self.spec = copy.deepcopy(original)
        del self.spec["paths"]["/api/pong"]
        self.invalid()

    def test_route_status_and_plan_drift_cannot_silently_pass(self):
        for key, value in [("path", "/different"), ("method", "DELETE"),
                           ("status", "almost"), ("milestone", 99), ("reason", "")]:
            data = copy.deepcopy(self.data)
            data["operations"]["pong.post"][key] = value
            self.invalid(data)
        data = copy.deepcopy(self.data)
        data["operations"]["pong.post"] = {"method": "POST", "path": "/api/pong", "status": "deferred"}
        self.invalid(data)

    def test_support_requires_method_tests_and_verification_scope(self):
        for key in ["rust", "protocol_tests", "evidence"]:
            data = copy.deepcopy(self.data)
            del data["operations"]["ping.get"][key]
            self.invalid(data)
        for key in ["live_scope", "source_reviewed", "unverified", "live_tests", "examples"]:
            data = copy.deepcopy(self.data)
            del data["evidence"]["fixture"][key]
            self.invalid(data)
        data = copy.deepcopy(self.data)
        data["operations"]["ping.get"]["protocol_tests"] = []
        self.invalid(data)

    def test_deleted_ambiguous_and_escaping_references_are_rejected(self):
        for file, anchor in [("tests/missing.rs", "fn ping_works("),
                             ("tests/ping.rs", "fn nonexistent("),
                             ("../outside.rs", "fn hidden(")]:
            data = copy.deepcopy(self.data)
            data["operations"]["ping.get"]["protocol_tests"][0].update(file=file, anchor=anchor)
            self.invalid(data)
        (self.root / "tests/ping.rs").write_text("fn ping_works() {}\nfn ping_works() {}\n")
        self.invalid()

    def test_method_operation_literal_must_exist_and_new_candidates_require_classification(self):
        (self.root / "src/ping.rs").write_text('pub async fn get() { send("other.get"); }\n')
        self.invalid()
        (self.root / "src/ping.rs").write_text('send("ping.get"); send("pong.post");\n')
        self.invalid()

    def test_duplicate_json_keys_and_changed_schema_digest_are_rejected(self):
        with self.assertRaises(report.CoverageError):
            report.parse_json('{"status":"supported","status":"deferred"}')
        self.data["schema_sha256"] = "0" * 64
        (self.root / "openapi/coverage.json").write_text(json.dumps(self.data))
        with self.assertRaises(report.CoverageError):
            report.build(self.root)

    def test_generation_is_deterministic_and_check_never_repairs_stale_output(self):
        generated = report.build(self.root)
        self.assertIn("1 supported", generated)
        self.assertIn("1 planned", generated)
        self.assertIn("src/ping.rs#L1", generated)
        self.assertIn("Concurrent requests.", generated)
        self.data["operations"] = dict(reversed(list(self.data["operations"].items())))
        self.save()
        self.assertEqual(report.build(self.root), generated)
        report.run(self.root, write=True)
        report.run(self.root, write=False)
        target = self.root / "API_COVERAGE.md"
        target.write_text("stale\n")
        with self.assertRaises(report.CoverageError):
            report.run(self.root, write=False)
        self.assertEqual(target.read_text(), "stale\n")

    def test_new_live_baseline_does_not_inherit_previous_verification(self):
        path = self.root / "tools/live/server.json"
        original = json.loads(path.read_text())
        for field, value in [("version", "3.0.0"), ("commit", "b" * 40)]:
            with self.subTest(field=field):
                changed = dict(original, **{field: value})
                path.write_text(json.dumps(changed))
                with self.assertRaises(report.CoverageError):
                    report.build(self.root)


if __name__ == "__main__":
    unittest.main()
