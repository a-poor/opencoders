# API coverage inventory

`openapi/coverage.json` is the reviewed source for the generated root
[`API_COVERAGE.md`](../../API_COVERAGE.md). It classifies every pinned operation ID
as supported, planned or deferred. Status is deliberately independent of live
verification: a supported operation has a public method and protocol tests, but
its live fixture may exercise only part of its behavior.

Requires Python 3.10+, standard library only. From the repository root:

```sh
python3 -m unittest discover -s tools/coverage -p 'test_*.py'
python3 tools/coverage/report.py --write
python3 tools/coverage/report.py --check
```

Both modes work offline. `--check` never rewrites files. CI checks the report and
runs the regression tests; ordinary Cargo builds do not invoke this tool.

## Updating coverage

1. Review the operation's HTTP contract and public method. Update its explicit
   entry, retaining the exact method and path from the pinned schema. Planned
   entries need milestone 2, 3 or 4 and a reason; deferred entries need a reason.
2. Supported entries need a Rust reference, at least one representative protocol
   test and an evidence profile. References contain a repository-relative file,
   a unique literal anchor and a readable label. The tool derives line links;
   regenerate when source/test lines move. Review that linked assertions actually
   exercise the operation; existence of the anchor alone does not prove this.
3. Review shared evidence profiles for live scope, source-reviewed behavior and
   remaining gaps. Link live assertions and examples, and update compatibility
   notes together. Raw fixture setup does not establish public SDK support.
4. Run tests, regenerate and inspect the diff, then run `--check`.

For a schema update, review provenance, generation and compatibility before
updating the inventory digest, routes or statuses. Every new operation must be
classified explicitly; no prefix or fallback silently assigns future routes.
Changing the pinned server version/commit also requires reviewing the inventory's
baseline and evidence. Do not copy old verification claims to a new baseline.

## What checks establish

The checker rejects duplicate JSON keys/operation IDs, missing or stale entries,
route changes, schema-digest/provenance mismatches, stale baseline references,
invalid statuses, missing classification details, broken/ambiguous file anchors
and stale generated Markdown. A conservative scan of handwritten `src/**/*.rs`
(excluding `models/`) flags quoted pinned operation IDs without a supported entry.

That scan is a maintenance hint: comments can trigger it, dynamically constructed
IDs can evade it, and it does not parse Rust or prove method routing. Likewise,
test references do not run tests, quantify behavioral coverage or verify the
truth/freshness of a reported live run. These require code review and recorded
test results. The inventory is scoped to this pinned schema, not the latest API.

See [release readiness](../../RELEASE_READINESS.md) for milestone acceptance gaps.
