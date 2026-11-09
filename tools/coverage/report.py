"""Offline inventory validation and deterministic API_COVERAGE.md rendering.

Python 3.10+, standard library only. References and operation literals are drift
checks, not a Rust parser or proof of behavioral coverage. Statuses are reviewed.
"""
import argparse
from collections import Counter
from datetime import date
import hashlib
import json
from pathlib import Path
import re
import sys
from urllib.parse import quote

ROOT = Path(__file__).resolve().parents[2]
METHODS = {"get", "put", "post", "delete", "options", "head", "patch", "trace"}
STATUSES = ("supported", "planned", "deferred")


class CoverageError(ValueError):
    """Inventory/schema/reference drift requiring explicit review."""


def require(condition, message):
    if not condition:
        raise CoverageError(message)


def parse_json(text):
    def unique(pairs):
        result = {}
        for key, value in pairs:
            require(key not in result, f"duplicate JSON key: {key}")
            result[key] = value
        return result
    try:
        return json.loads(text, object_pairs_hook=unique)
    except json.JSONDecodeError as error:
        raise CoverageError(str(error)) from error


def keys(value, expected, context):
    require(isinstance(value, dict), f"{context}: expected object")
    require(set(value) == set(expected), f"{context}: expected fields {sorted(expected)}")


def nonempty(value, context):
    require(isinstance(value, str) and bool(value.strip()), f"{context}: expected nonempty text")


def text_list(value, context):
    require(isinstance(value, list), f"{context}: expected list")
    for text in value:
        nonempty(text, context)


def local(root, name):
    nonempty(name, "file reference")
    path = Path(name)
    require(not path.is_absolute() and ".." not in path.parts, f"unsafe file reference: {name}")
    resolved = (root / path).resolve()
    require(resolved.is_relative_to(root.resolve()), f"reference leaves repository: {name}")
    require(resolved.is_file(), f"missing referenced file: {name}")
    return resolved


def reference(root, ref, prefix):
    keys(ref, {"file", "anchor", "label"}, "reference")
    nonempty(ref["file"], "reference file")
    require(ref["file"].startswith(prefix) and ref["file"].endswith(".rs"),
            f"reference must be Rust under {prefix}: {ref['file']}")
    nonempty(ref["anchor"], "anchor")
    nonempty(ref["label"], "label")
    text = local(root, ref["file"]).read_text()
    require(text.count(ref["anchor"]) == 1,
            f"missing or ambiguous anchor in {ref['file']}: {ref['anchor']}")
    return text.count("\n", 0, text.index(ref["anchor"])) + 1


def operations(spec):
    require(isinstance(spec.get("paths"), dict), "schema has no paths")
    found = {}
    for path, item in spec["paths"].items():
        require(isinstance(item, dict), f"invalid path item: {path}")
        for method, operation in item.items():
            if method not in METHODS:
                continue
            require(isinstance(operation, dict), f"invalid operation: {method} {path}")
            op = operation.get("operationId")
            nonempty(op, f"operationId at {method} {path}")
            require(op not in found, f"duplicate operationId: {op}")
            found[op] = (method.upper(), path)
    return found


def validate(root, spec, data):
    keys(data, {"format_version", "schema_sha256", "baseline", "operations", "evidence"}, "inventory")
    require(data["format_version"] == 1, "unsupported coverage format")
    require(isinstance(data["schema_sha256"], str) and
            re.fullmatch(r"[0-9a-f]{64}", data["schema_sha256"]), "invalid schema SHA-256")
    baseline = data["baseline"]
    keys(baseline, {"version", "commit", "verified_on", "platform"}, "baseline")
    for field in baseline:
        nonempty(baseline[field], f"baseline.{field}")
    require(re.fullmatch(r"[0-9a-f]{40}", baseline["commit"]), "invalid baseline commit")
    try:
        verified_on = date.fromisoformat(baseline["verified_on"])
    except ValueError as error:
        raise CoverageError("invalid baseline verification date") from error
    require(verified_on.isoformat() == baseline["verified_on"], "expected YYYY-MM-DD verification date")
    actual = operations(spec)
    declared = data["operations"]
    require(isinstance(declared, dict), "operations must be an object")
    require(set(actual) == set(declared),
            f"operation inventory mismatch; missing={sorted(set(actual)-set(declared))}, "
            f"stale={sorted(set(declared)-set(actual))}")
    profiles = data["evidence"]
    require(isinstance(profiles, dict), "evidence must be an object")
    for name, profile in profiles.items():
        require(re.fullmatch(r"[a-z][a-z0-9-]*", name), f"invalid evidence key: {name}")
        keys(profile, {"live_scope", "live_tests", "source_reviewed", "unverified", "examples"}, name)
        nonempty(profile["live_scope"], name)
        for field in ["source_reviewed", "unverified", "examples"]:
            text_list(profile[field], f"{name}.{field}")
        require(isinstance(profile["live_tests"], list), f"{name}: expected live test list")
        for ref in profile["live_tests"]:
            reference(root, ref, "tests/")
        for example in profile["examples"]:
            require(example.startswith("examples/") and example.endswith(".rs"), f"invalid example: {example}")
            local(root, example)
    used = set()
    for op, row in declared.items():
        require(isinstance(row, dict), f"invalid entry: {op}")
        status = row.get("status")
        require(status in STATUSES, f"{op}: invalid status {status!r}")
        fields = {"method", "path", "status"}
        fields |= {"rust", "protocol_tests", "evidence"} if status == "supported" else {"reason"}
        if status == "planned":
            fields.add("milestone")
        keys(row, fields, op)
        require((row["method"], row["path"]) == actual[op], f"{op}: method/path changed; review route")
        if status != "supported":
            nonempty(row["reason"], op)
            if status == "planned":
                require(type(row["milestone"]) is int and row["milestone"] in (2, 3, 4),
                        f"{op}: invalid planned milestone")
            continue
        reference(root, row["rust"], "src/")
        require('"' + op + '"' in local(root, row["rust"]["file"]).read_text(),
                f"{op}: source reference lacks operation literal")
        require(isinstance(row["protocol_tests"], list) and row["protocol_tests"],
                f"{op}: supported operation needs protocol tests")
        for ref in row["protocol_tests"]:
            reference(root, ref, "tests/")
        require(isinstance(row["evidence"], str) and row["evidence"] in profiles,
                f"{op}: unknown evidence profile")
        used.add(row["evidence"])
    require(used == set(profiles), f"unused evidence profiles: {sorted(set(profiles)-used)}")

    # Conservative hints only: a new quoted schema operation in handwritten Rust
    # requires review. Comments may trigger this; it does not prove call wiring.
    for source in sorted((root / "src").rglob("*.rs")):
        if "models" in source.relative_to(root / "src").parts:
            continue
        text = source.read_text()
        for op in actual:
            if '"' + op + '"' in text:
                require(declared[op]["status"] == "supported",
                        f"{op}: unreviewed operation literal in {source.relative_to(root)}")
    return actual


def cell(value):
    return str(value).replace("|", r"\|").replace("\n", " ")


def link(root, ref):
    line = reference(root, ref, "src/" if ref["file"].startswith("src/") else "tests/")
    return f"[{cell(ref['label'])}]({quote(ref['file'])}#L{line})"


def render(root, data):
    rows = data["operations"]
    baseline = data["baseline"]
    counts = Counter(row["status"] for row in rows.values())
    out = ["# OpenCode v2 API coverage", "",
        "<!-- Generated by tools/coverage/report.py --write; edit openapi/coverage.json. -->", "",
        f"**{len(rows)} operations: {counts['supported']} supported, {counts['planned']} planned, {counts['deferred']} deferred.**", "",
        "This counts pinned HTTP operation IDs, including SSE/WebSocket upgrade routes,",
        "not behavioral test coverage or a claim of complete UI support. No operation",
        "is inferred to work solely because models exist or a raw fixture calls it.", "",
        f"Schema SHA-256: `{data['schema_sha256']}`. The schema's producing server release",
        "is unknown; see [provenance](openapi/README.md). Reviewed live evidence uses",
        f"OpenCode **{baseline['version']}** on **{baseline['platform']}**, shared baseline verified **{baseline['verified_on']}**;",
        f"commit `{baseline['commit']}`. [Compatibility notes](tools/live/README.md)",
        "record the verified scope, hosted runs and remaining limitations. Newer or",
        "narrower verification is dated separately in each evidence profile below.", "",
        "- **Supported:** a public resource method and protocol tests exist. Live scope and limitations are separate below.",
        "- **Planned:** assigned to a design milestone; no public method yet and no release/date commitment.",
        "- **Deferred:** no current implementation commitment; each entry records why.", "",
        "Regenerate with `python3 tools/coverage/report.py --write`; check with `--check`.",
        "Both are offline. Checks validate schema/route completeness, file/anchor references,",
        "and conservative operation-literal hints. They do not run tests, parse Rust call",
        "graphs, or establish that assertions cover every parameter or server behavior.",
        "See [tool maintenance](tools/coverage/README.md) and [release readiness](RELEASE_READINESS.md).", "",
        "## Supported operations", "",
        "Method references include convenience location overrides where available.",
        "Protocol links identify representative tests; their files also contain error/boundary cases.", "",
        "| Operation ID | Route | Rust API | Protocol evidence | Live scope and limits |",
        "| --- | --- | --- | --- | --- |"]
    for op, row in sorted(rows.items()):
        if row["status"] != "supported":
            continue
        tests = ", ".join(link(root, ref) for ref in row["protocol_tests"])
        out.append(f"| `{op}` | `{row['method']} {cell(row['path'])}` | {link(root,row['rust'])} | {tests} | [{row['evidence']}](#evidence-{row['evidence']}) |")
    for status in ["planned", "deferred"]:
        out += ["", f"## {status.capitalize()} operations", "",
                "| Operation ID | Route | Milestone / reason |", "| --- | --- | --- |"]
        for op, row in sorted(rows.items()):
            if row["status"] == status:
                reason = (f"M{row['milestone']}: " if status == "planned" else "") + row["reason"]
                out.append(f"| `{op}` | `{row['method']} {cell(row['path'])}` | {cell(reason)} |")
    out += ["", "## Verification evidence", "",
            "These profiles summarize reviewed tests and prior local compatibility runs.",
            "The checker verifies references, not the truth or freshness of a reported run.",
            "Source-reviewed behavior is not promoted to live verification."]
    for name, profile in sorted(data["evidence"].items()):
        out += ["", f"### Evidence: {name}", "", profile["live_scope"], ""]
        out.append("- **Live assertions:** " + (", ".join(link(root, r) for r in profile["live_tests"]) or "none") + ".")
        if profile["source_reviewed"]:
            out.append("- **Source-reviewed only:** " + " ".join(profile["source_reviewed"]))
        if profile["unverified"]:
            out.append("- **Still unverified live:** " + " ".join(profile["unverified"]))
        if profile["examples"]:
            out.append("- **Examples:** " + ", ".join(f"[{Path(p).stem}]({quote(p)})" for p in profile["examples"]) + ".")
    return "\n".join(out) + "\n"


def build(root):
    spec_bytes = (root / "openapi/opencode-v2.json").read_bytes()
    spec = parse_json(spec_bytes)
    data = parse_json((root / "openapi/coverage.json").read_text())
    provenance = parse_json((root / "openapi/provenance.json").read_text())
    digest = hashlib.sha256(spec_bytes).hexdigest()
    require(data.get("schema_sha256") == provenance.get("sha256") == digest,
            "schema digest changed; explicitly review provenance and coverage")
    actual = validate(root, spec, data)
    require(provenance.get("operations") == len(actual), "provenance operation count changed")
    baseline = parse_json((root / "tools/live/server.json").read_text())
    require(all(data["baseline"][field] == baseline.get(field) for field in ("version", "commit")),
            "live baseline changed; explicitly review verification evidence")
    return render(root, data)


def run(root, write):
    output = build(root)
    target = root / "API_COVERAGE.md"
    if write:
        target.write_text(output)
    else:
        require(target.is_file() and target.read_text() == output,
                "API_COVERAGE.md is stale; review inputs and run --write")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--check", action="store_true")
    mode.add_argument("--write", action="store_true")
    args = parser.parse_args()
    try:
        run(ROOT, write=args.write)
    except (CoverageError, OSError) as error:
        print(f"Coverage check failed: {error}", file=sys.stderr)
        return 1
    print("API coverage written" if args.write else "API coverage matches reviewed inputs")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
