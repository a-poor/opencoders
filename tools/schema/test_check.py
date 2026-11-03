"""Offline behavior checks for the upstream OpenAPI drift report."""
import contextlib
import hashlib
import io
import json
from pathlib import Path
import tempfile
import unittest
from unittest import mock

import check


class SchemaCheckTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.pinned = self.root / "pinned.json"
        self.provenance = self.root / "provenance.json"
        self.candidate = self.root / "candidate.json"
        self.summary = self.root / "summary.md"
        self.schema = {
            "openapi": "3.1.0",
            "info": {"title": "Test API", "version": "0.0.1"},
            "paths": {"/ping": {"get": {"operationId": "ping.get"}}},
            "components": {"schemas": {"Ping": {"type": "string"}}},
        }
        self.pinned.write_text(json.dumps(self.schema))
        self.provenance.write_text(json.dumps({
            "sha256": hashlib.sha256(self.pinned.read_bytes()).hexdigest(),
        }))
        self.candidate.write_bytes(self.pinned.read_bytes())

    def run_check(self):
        before = [p.read_bytes() for p in (self.pinned, self.provenance, self.candidate)]
        output = io.StringIO()
        with contextlib.redirect_stdout(output):
            result = check.main([
                str(self.candidate), "--pinned", str(self.pinned),
                "--provenance", str(self.provenance), "--summary", str(self.summary),
            ])
        self.assertEqual(before, [p.read_bytes() for p in (
            self.pinned, self.provenance, self.candidate)])
        self.assertEqual(output.getvalue().rstrip(), self.summary.read_text().rstrip())
        return result, output.getvalue()

    def test_identical_and_reformatted_schema_are_current(self):
        for candidate in (self.pinned.read_text(),
                          json.dumps(self.schema, sort_keys=True, indent=4)):
            with self.subTest(candidate=candidate[:20]):
                self.candidate.write_text(candidate)
                result, output = self.run_check()
                self.assertEqual(result, 0)
                self.assertIn("No schema content changes", output)

    def test_content_change_is_drift_even_when_info_version_is_unchanged(self):
        self.schema["components"]["schemas"]["Ping"]["type"] = "integer"
        self.candidate.write_text(json.dumps(self.schema))
        result, output = self.run_check()
        self.assertEqual(result, 1)
        self.assertIn("Schema drift detected", output)
        self.assertIn('"type": "integer"', output)
        self.assertIn("not necessarily a breaking change", output)

    def test_array_order_and_json_scalar_types_remain_visible(self):
        for old, new in ((["a", "b"], ["b", "a"]), (True, 1)):
            with self.subTest(old=old, new=new):
                self.schema["x-test"] = old
                self.pinned.write_text(json.dumps(self.schema))
                self.provenance.write_text(json.dumps({
                    "sha256": hashlib.sha256(self.pinned.read_bytes()).hexdigest(),
                }))
                self.schema["x-test"] = new
                self.candidate.write_text(json.dumps(self.schema))
                self.assertEqual(self.run_check()[0], 1)

    def test_bad_json_and_non_schema_responses_are_errors_not_drift(self):
        invalid = ("<html>unavailable</html>", "null", "[]", "{}",
                   '{"error": "temporarily unavailable"}',
                   '{"openapi": "3.1.0", "openapi": "3.2.0"}',
                   json.dumps(self.schema)[:-1] + ', "x-bad": NaN}')
        for candidate in invalid:
            with self.subTest(candidate=candidate):
                self.candidate.write_text(candidate)
                result, output = self.run_check()
                self.assertEqual(result, 2)
                self.assertIn("Schema check failed", output)
                self.assertNotIn("Schema drift detected", output)

    def test_invalid_pinned_checksum_is_an_error(self):
        self.provenance.write_text(json.dumps({"sha256": "0" * 64}))
        result, output = self.run_check()
        self.assertEqual(result, 2)
        self.assertIn("provenance", output)

    def test_missing_candidate_is_an_error(self):
        self.candidate.unlink()
        with contextlib.redirect_stdout(io.StringIO()) as output:
            result = check.main([str(self.candidate), "--pinned", str(self.pinned),
                                 "--provenance", str(self.provenance)])
        self.assertEqual(result, 2)
        self.assertIn("Schema check failed", output.getvalue())

    def test_oversized_candidate_is_an_error(self):
        self.candidate.write_bytes(b" " * 1025)
        with mock.patch.object(check, "MAX_BYTES", 1024):
            result, output = self.run_check()
        self.assertEqual(result, 2)
        self.assertIn("input limit", output)

    def test_summary_cannot_overwrite_any_input(self):
        for path in (self.pinned, self.provenance, self.candidate):
            with self.subTest(path=path):
                before = path.read_bytes()
                with contextlib.redirect_stdout(io.StringIO()), \
                        contextlib.redirect_stderr(io.StringIO()):
                    result = check.main([
                        str(self.candidate), "--pinned", str(self.pinned),
                        "--provenance", str(self.provenance), "--summary", str(path),
                    ])
                self.assertEqual(result, 2)
                self.assertEqual(path.read_bytes(), before)

    def test_large_diffs_are_bounded_in_the_summary(self):
        self.schema["components"]["schemas"].update({
            f"New{i}": {"description": "x" * 500, "type": "string"}
            for i in range(300)
        })
        self.candidate.write_text(json.dumps(self.schema))
        result, output = self.run_check()
        self.assertEqual(result, 1)
        self.assertIn("Diff truncated", output)
        self.assertLess(len(output.encode()), 25000)


if __name__ == "__main__":
    unittest.main()
