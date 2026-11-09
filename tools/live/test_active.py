"""Pure fixture tests: no server, socket, provider, or model calls."""

import json
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

import active


class ActiveFixtureTests(unittest.TestCase):
    def request(self, scenario, messages=None):
        return {
            "model": scenario, "stream": True,
            "messages": messages or [{"role": "user", "content": "fixture"}],
            "tools": [{"type": "function", "function": {"name": name}}
                      for name in ["read", "question"]],
        }

    def test_config_uses_only_local_provider_and_restricts_tools(self):
        with tempfile.TemporaryDirectory() as temp, patch.dict(
            "os.environ", {"OPENAI_API_KEY": "must-not-leak", "OPENCODE_CONFIG_CONTENT": "host-config"}
        ):
            env = active.environment(Path(temp), "server-password", "http://127.0.0.1:1234", "fixture-token")
        config = json.loads(env["OPENCODE_CONFIG_CONTENT"])
        self.assertNotIn("OPENAI_API_KEY", env)
        self.assertNotIn("must-not-leak", json.dumps(env))
        self.assertEqual(list(config["providers"]), ["fixture"])
        settings = config["providers"]["fixture"]["settings"]
        self.assertEqual(settings["baseURL"], "http://127.0.0.1:1234/v1")
        self.assertEqual(settings["apiKey"], "fixture-token")
        self.assertFalse(config["warming"])
        self.assertFalse(config["compaction"]["auto"])
        self.assertEqual(config["permissions"], [
            {"action": "*", "resource": "*", "effect": "deny"},
            {"action": "read", "resource": "fixture.txt", "effect": "ask"},
            {"action": "question", "resource": "*", "effect": "allow"},
        ])

    def test_config_rejects_nonlocal_or_decorated_origins(self):
        with tempfile.TemporaryDirectory() as temp:
            for origin in ["https://api.example.com", "http://localhost:1234", "http://127.0.0.1:1234/path",
                           "http://user@127.0.0.1:1234", "http://127.0.0.1"]:
                with self.subTest(origin=origin), self.assertRaises(ValueError):
                    active.environment(Path(temp), "password", origin, "token")

    def test_output_is_split_and_gated(self):
        plan = active.plan(self.request("output"), 1)
        self.assertEqual(plan["first"], "first ")
        self.assertEqual(plan["last"], "second")
        self.assertTrue(plan["hold"])

    def test_recovery_has_separately_gated_missed_and_reconnected_fragments(self):
        plan = active.plan(self.request("recovery"), 1)
        self.assertEqual(plan["segments"], [
            {"text": "before ", "gate": "recovery/missed"},
            {"text": "missed ", "gate": "recovery/after"},
            {"text": "after", "gate": "recovery/end"},
        ])
        with self.assertRaises(ValueError):
            active.plan(self.request("recovery"), 2)

    def test_permission_call_reads_only_fixture_file(self):
        plan = active.plan(self.request("permission"), 1)
        self.assertEqual(plan["tool"], "read")
        self.assertEqual(plan["arguments"], {"path": "fixture.txt"})

    def test_tool_continuation_requires_matching_result(self):
        for scenario, marker in [("permission", active.FILE_TEXT), ("question", "fixture-answer"),
                                 ("cancel", "fixture-feedback"), ("recovery_question", "fixture-answer")]:
            messages = [
                {"role": "assistant", "content": None, "tool_calls": [{
                    "id": "call_" + scenario, "type": "function", "function": {"name": "read"}}]},
                {"role": "tool", "tool_call_id": "call_" + scenario, "content": marker},
            ]
            self.assertEqual(active.plan(self.request(scenario, messages), 2)["last"], scenario + " resumed")
            messages[1]["content"] = "wrong result"
            with self.assertRaises(ValueError):
                active.plan(self.request(scenario, messages), 2)

    def test_unexpected_requests_fail_instead_of_driving_more_execution(self):
        for request, count in [(self.request("unknown"), 1), (self.request("output"), 2),
                               (self.request("permission"), 2), (self.request("question"), 3),
                               ({**self.request("output"), "stream": False}, 1),
                               ({**self.request("permission"), "tools": []}, 1)]:
            with self.subTest(request=request, count=count), self.assertRaises(ValueError):
                active.plan(request, count)

    def test_workflow_summary_requires_instructions_and_never_repeats(self):
        request = self.request("compact", [{"role": "user", "content": "## Objective"}])
        self.assertIn("## Objective", active.plan(request, 1)["last"])
        with self.assertRaises(ValueError):
            active.plan(request, 2)
        with self.assertRaises(ValueError):
            active.plan(self.request("compact"), 1)
        self.assertTrue(active.plan(self.request("command"), 1)["hold"])
        self.assertEqual(active.plan(self.request("clear"), 1)["last"], "cleared revert resumed")

    def test_chunks_encode_sse_and_tool_arguments(self):
        chunk = active.chunk({"tool_calls": [{"index": 0, "function": {"arguments": "雪"}}]})
        self.assertTrue(chunk.startswith(b"data: "))
        self.assertTrue(chunk.endswith(b"\n\n"))
        data = json.loads(chunk[6:])
        self.assertEqual(data["choices"][0]["delta"]["tool_calls"][0]["function"]["arguments"], "雪")


if __name__ == "__main__":
    unittest.main()
