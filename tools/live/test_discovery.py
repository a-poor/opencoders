"""Protocol/ownership checks for local discovery fixtures; no server required."""
import json
from pathlib import Path
import tempfile
import unittest

import discovery


class DiscoveryTests(unittest.TestCase):
    def test_fixture_installs_only_owned_local_plugins_and_stdio_commands(self):
        with tempfile.TemporaryDirectory() as temp:
            root=Path(temp).resolve()
            env={}
            config={}
            discovery.install(root,env,config)
            self.assertEqual(env["OPENCODERS_DISCOVERY_PRESENT"],"fixture-marker")
            self.assertFalse(config["websearch"])
            self.assertEqual(len(config["plugins"]),2)
            for plugin in config["plugins"]:
                self.assertTrue(Path(plugin).is_relative_to(root))
                self.assertTrue(Path(plugin).is_dir())
                self.assertTrue((Path(plugin) / "server.js").is_file())
            self.assertEqual(len(config["mcp"]["servers"]),4)
            for server in config["mcp"]["servers"].values():
                self.assertEqual(server["type"],"local")
                self.assertTrue(Path(server["command"][1]).is_relative_to(root))
                self.assertEqual(server["cwd"],str(root))
            self.assertTrue(config["mcp"]["servers"]["fixture-disabled"]["disabled"])

    def test_mcp_fixture_preserves_ids_capabilities_and_populated_catalogs(self):
        reply=discovery.response({"jsonrpc":"2.0","id":"init","method":"initialize","params":{"protocolVersion":"2025-03-26"}},"complete")
        self.assertEqual(reply["id"],"init")
        self.assertEqual(reply["result"]["protocolVersion"],"2025-03-26")
        self.assertIn("resources",reply["result"]["capabilities"])
        for method,key in [("resources/list","resources"),("resources/templates/list","resourceTemplates")]:
            reply=discovery.response({"id":7,"method":method},"complete")
            self.assertEqual(len(reply["result"][key]),2)
        self.assertIsNone(discovery.response({"method":"notifications/initialized"},"complete"))
        self.assertEqual(discovery.response({"id":0,"method":"tools/call"},"complete")["error"]["code"],-32601)

    def test_missing_template_method_is_explicit_while_resources_remain_available(self):
        reply=discovery.response({"id":3,"method":"resources/templates/list"},"no-templates")
        self.assertEqual(reply["error"]["code"],-32601)
        self.assertEqual(len(discovery.response({"id":4,"method":"resources/list"},"no-templates")["result"]["resources"]),2)


if __name__=="__main__":
    unittest.main()
