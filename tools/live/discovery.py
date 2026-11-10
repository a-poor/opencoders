"""Owned plugin and stdio MCP catalogs for the disposable smoke runner."""
import json
from pathlib import Path
import shutil
import sys


def install(root, env, document):
    directory = root / "discovery-fixtures"
    directory.mkdir()
    script = directory / "mcp.py"
    shutil.copyfile(__file__, script)
    # Configured local plugins must be directories in v2.2.5.
    plugin = directory / "plugin"
    failed = directory / "failed"
    for target in (plugin, failed):
        target.mkdir()
        (target / "package.json").write_text(json.dumps({"type":"module"}))
    shutil.copyfile(Path(__file__).with_name("discovery-plugin.mjs"), plugin / "server.js")
    (failed / "server.js").write_text('export default {id:"opencoders.discovery.failed",async setup(){throw new Error("owned fixture setup failure")}};\n')
    document["plugins"] = [str(plugin), str(failed)]
    document["websearch"] = False
    document["policies"] = [{"action":"integration.connect","resource":"opencoders-discovery-denied","effect":"deny"}]
    env["OPENCODERS_DISCOVERY_PRESENT"] = "fixture-marker"
    servers = {}
    for name, mode in [("fixture-catalog","complete"),("fixture-no-templates","no-templates"),("fixture-disabled","complete")]:
        servers[name] = {
            "type":"local", "command":[sys.executable,str(script),"--mcp",mode,str(directory / (name + ".jsonl"))],
            "cwd":str(root),"timeout":{"startup":5000,"catalog":1000},
            "disabled":name == "fixture-disabled",
        }
    servers["fixture-failed"] = {"type":"local","command":[sys.executable,str(directory / "missing.py")],"cwd":str(root),"timeout":{"startup":1000}}
    document["mcp"] = {"servers":servers}


def response(request, mode):
    if "id" not in request:
        return None
    method = request.get("method")
    params = request.get("params", {})
    result = None
    if method == "initialize":
        result = {"protocolVersion":params["protocolVersion"],"serverInfo":{"name":"opencoders-fixture","version":"1","title":"Owned MCP fixture"},"capabilities":{"resources":{},"tools":{},"prompts":{}}}
    elif method == "ping":
        result = {}
    elif method == "tools/list":
        result = {"tools":[]}
    elif method == "prompts/list":
        result = {"prompts":[]}
    elif method == "resources/list":
        result = {"resources":[
            {"name":"Zulu","uri":"fixture://z","description":"Owned catalog entry","mimeType":"text/plain"},
            {"name":"Alpha","uri":"fixture://a","mimeType":"application/json"},
        ]}
    elif method == "resources/templates/list" and mode == "complete":
        result = {"resourceTemplates":[
            {"name":"Zulu","uriTemplate":"fixture://z/{name}","mimeType":"text/plain"},
            {"name":"Alpha","uriTemplate":"fixture://a/{name}","description":"Owned template"},
        ]}
    envelope = {"jsonrpc":"2.0","id":request["id"]}
    if result is None:
        envelope["error"] = {"code":-32601,"message":"Method not implemented by owned fixture"}
    else:
        envelope["result"] = result
    return envelope


def serve(mode, audit):
    for line in sys.stdin:
        request = json.loads(line)
        # Record only method names, never raw requests, authentication or content.
        with audit.open("a") as output:
            output.write(json.dumps({"method":request.get("method")}) + "\n")
        reply = response(request, mode)
        if reply is not None:
            print(json.dumps(reply), flush=True)


if __name__ == "__main__":
    if len(sys.argv) != 4 or sys.argv[1] != "--mcp" or sys.argv[2] not in ("complete","no-templates"):
        raise SystemExit("This fixture is launched only by the disposable smoke server")
    serve(sys.argv[2], Path(sys.argv[3]))
