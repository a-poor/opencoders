# Pinned OpenCode contract

`opencode-v2.json` is the unmodified schema reviewed during the initial design.
`provenance.json` records its source, retrieval date, SHA-256, and inventory counts.

The schema's `info.version` is **not** a server compatibility baseline. The
published artifact does not identify an associated server release, so provenance
records that version as `null`. A separate [live baseline](../tools/live/README.md)
now tests OpenCode v2.2.5; it does not establish which release produced this schema.

Schema updates must be explicit: download a candidate to a temporary file,
compare it with this snapshot, update provenance, regenerate models, and review
the behavioral and compatibility changes. Normal builds do not fetch a schema.

See [the generation experiment](../tools/model-spike/README.md) for commands,
findings, and known limitations.

[`coverage.json`](coverage.json) explicitly classifies all pinned operation IDs
and records method/test references and verification limits. The generated
[coverage report](../API_COVERAGE.md) is checked offline in CI. Schema and
server-baseline updates require reviewing this inventory too; see
[maintenance instructions](../tools/coverage/README.md).
