"""Compare a downloaded OpenAPI document with the reviewed pin, without network I/O."""
import argparse
import difflib
import hashlib
import json
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[2]
MAX_BYTES = 16 * 1024 * 1024
MAX_DIFF_LINES = 200
MAX_DIFF_CHARS = 20000


def read_json(path):
    def unique(pairs):
        result = {}
        for key, value in pairs:
            if key in result:
                raise ValueError(f"duplicate JSON key: {key!r}")
            result[key] = value
        return result

    def invalid_constant(value):
        raise ValueError(f"invalid JSON constant: {value}")

    with path.open("rb") as source:
        raw = source.read(MAX_BYTES + 1)
    if len(raw) > MAX_BYTES:
        raise ValueError(f"{path.name} exceeds the {MAX_BYTES}-byte input limit")
    return raw, json.loads(raw, object_pairs_hook=unique, parse_constant=invalid_constant)


def schema_text(schema):
    # Basic document-shape checks distinguish error responses from schema drift.
    # This is not a complete OpenAPI validator or a breaking-change detector.
    if not isinstance(schema, dict):
        raise ValueError("expected an OpenAPI document object")
    info = schema.get("info")
    components = schema.get("components", {})
    if (not isinstance(schema.get("openapi"), str)
            or not isinstance(info, dict)
            or not isinstance(info.get("title"), str)
            or not isinstance(info.get("version"), str)
            or not isinstance(schema.get("paths"), dict)
            or not isinstance(components, dict)
            or not isinstance(components.get("schemas", {}), dict)):
        raise ValueError("expected OpenAPI version, info, paths and optional component schemas")
    return json.dumps(schema, sort_keys=True, indent=2, ensure_ascii=True, allow_nan=False)


def diff_excerpt(pinned, candidate):
    lines = []
    size = 0
    truncated = False
    for line in difflib.unified_diff(pinned.splitlines(), candidate.splitlines(),
                                     fromfile="pinned", tofile="upstream", lineterm=""):
        if len(lines) >= MAX_DIFF_LINES or size + len(line) + 1 > MAX_DIFF_CHARS:
            truncated = True
            break
        lines.append(line)
        size += len(line) + 1
    excerpt = "```diff\n" + "\n".join(lines) + "\n```\n"
    if truncated:
        excerpt += "\nDiff truncated; download the candidate for a complete review.\n"
    return excerpt


def compare(pinned_path, provenance_path, candidate_path):
    pinned_raw, pinned = read_json(pinned_path)
    _, provenance = read_json(provenance_path)
    pinned_hash = hashlib.sha256(pinned_raw).hexdigest()
    if not isinstance(provenance, dict) or provenance.get("sha256") != pinned_hash:
        raise ValueError("pinned schema does not match provenance SHA-256")
    candidate_raw, candidate = read_json(candidate_path)
    pinned_text = schema_text(pinned)
    candidate_text = schema_text(candidate)
    changed = pinned_text != candidate_text
    status = "Schema drift detected" if changed else "No schema content changes"
    report = (
        f"# OpenAPI schema check\n\n**{status}.**\n\n"
        "Comparison ignores JSON whitespace and object-key order. Array order, "
        "descriptions and other content remain significant.\n\n"
        f"Pinned raw SHA-256: `{pinned_hash}`\n\n"
        f"Upstream raw SHA-256: `{hashlib.sha256(candidate_raw).hexdigest()}`\n\n"
        f"Paths: {len(pinned['paths'])} pinned / {len(candidate['paths'])} upstream. "
        f"Component schemas: {len(pinned.get('components', {}).get('schemas', {}))} pinned / "
        f"{len(candidate.get('components', {}).get('schemas', {}))} upstream.\n"
    )
    if changed:
        report += (
            "\nThis is a content change, not necessarily a breaking change or a newer "
            "server release. Review the candidate using openapi/README.md; the pin "
            "and generated models have not been modified.\n\n"
            + diff_excerpt(pinned_text, candidate_text)
        )
    return int(changed), report


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("candidate", type=Path, help="downloaded upstream JSON file")
    parser.add_argument("--pinned", type=Path, default=ROOT / "openapi/opencode-v2.json")
    parser.add_argument("--provenance", type=Path, default=ROOT / "openapi/provenance.json")
    parser.add_argument("--summary", type=Path, help="also write a Markdown report")
    args = parser.parse_args(argv)
    try:
        code, report = compare(args.pinned, args.provenance, args.candidate)
    except (OSError, ValueError, RecursionError) as error:
        code = 2
        report = f"# OpenAPI schema check\n\n**Schema check failed.**\n\n{error}\n"
    print(report, end="")
    if args.summary:
        try:
            if args.summary.resolve() in {
                path.resolve() for path in (args.pinned, args.provenance, args.candidate)
            }:
                raise ValueError("summary must not overwrite an input file")
            args.summary.write_text(report, encoding="utf-8")
        except (OSError, ValueError) as error:
            print(f"Cannot write schema summary: {error}", file=sys.stderr)
            return 2
    return code


if __name__ == "__main__":
    raise SystemExit(main())
