# Pinned OpenCode contract

`opencode-v2.json` is the unmodified schema reviewed during the initial design.
`provenance.json` records its source, retrieval date, SHA-256, and inventory counts.

The schema's `info.version` is **not** a server compatibility baseline. The
published artifact does not identify an associated server release, so provenance
records that version as `null`. Selecting a pinned server remains a separate task.

Schema updates must be explicit: download a candidate to a temporary file,
compare it with this snapshot, update provenance, regenerate models, and review
the behavioral and compatibility changes. Normal builds do not fetch a schema.

See [the generation experiment](../tools/model-spike/README.md) for commands,
findings, and known limitations.
