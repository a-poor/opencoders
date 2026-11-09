# Pinned OpenCode contract

`opencode-v2.json` is the unmodified published schema reviewed on 2026-11-09.
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

## 2026-11-09 drift review

The [scheduled check](https://github.com/a-poor/opencoders/actions/runs/42687061589)
correctly reported content drift. A fresh download matched the job's upstream
SHA-256, `e54ea93eac5278b072769616478e94638c3ea3974b36ce5998b834aff0e3265f`.
It replaces the 2026-11-02 pin,
`2fdcda59d6c37f5dac904ab7b7aff05fb98465e23eb0fe8d3b9767151ad391de`.
The inventory grows from 169 to 170 operations, 138 to 139 paths, and 490 to 497
component schemas; no operations or component schemas were removed.

The complete content review found:

- New `GET /api/experimental/job` (`experimental.job.list`) and seven `Job.*`
  schemas for shell, subagent and image jobs. The route is explicitly planned
  for milestone 4; this update adds no public client method.
- Optional configuration flags `all_flags`, `codemode_vm`, and `part_rows`, plus
  `websearch.use` in both policy action enums; optional `pool` on local/remote
  MCP configurations and optional `criteria` on predicate questions.
- Optional true-only `paste` on integration attempts and OAuth completion prose
  describing redirect-URL paste handling.
- Required `tokens` and `cost` in `SessionGenerateResponse.data`, and a narrower
  `credential.updated.data` object requiring `integrationID` and `credentialID`.
  These are contract changes, not merely additive optional fields. Generation
  endpoints are unimplemented; credential events remain raw, unselected events.
- Removal of the documented 503 response from `agent.list`. Its request and
  success response are unchanged. The shared typed `ServiceUnavailable` error
  handling remains useful for older servers and other operations.

All nine generation checks reproduce the checked-in Rust models unchanged; the
full-component experiment also generates and compiles all 497 schemas.
Supported non-streaming request/success contracts and selected typed event
payloads are unchanged.
The separately tested server baseline remains v2.2.5 with its existing evidence;
this schema review does not establish compatibility with a newer server release.

## Weekly upstream check

[`schema.yml`](../.github/workflows/schema.yml) runs on Linux each Monday at
**09:17 UTC** and supports manual dispatch. It downloads the published JSON into
runner temporary storage, then compares it with the reviewed pin. The workflow
has read-only repository permissions and never updates the pin, provenance,
generated code, issues or pull requests. It is separate from ordinary CI so an
upstream change or outage does not make the library's CI badge fail.

[`tools/schema/check.py`](../tools/schema/check.py) first verifies the pinned
file's provenance checksum, then compares parsed JSON with sorted object keys.
Whitespace and object-key order are ignored; array order, descriptions and other
content remain significant. `info.version` alone is insufficient: its value does
not identify a server release and may stay unchanged while the schema changes.

- Exit **0**: no content changes, including formatting-only differences.
- Exit **1**: schema drift. The job fails and its summary includes raw hashes,
  path/schema counts and a diff limited to 200 lines / 20,000 characters.
- Exit **2**: invalid/missing input, provenance mismatch or report-writing error.
  Download failures stop the separate download step before comparison.

Drift means review is needed, not that the SDK is broken or the change is newer
or backward-compatible. Inspect the diff and follow the explicit update process
above; do not replace the pin just to clear the check. The job will continue to
report drift weekly until the pin is updated or upstream matches again.

Run the same comparison locally (the Python checker itself performs no network I/O):

```sh
schema_candidate=$(mktemp)
curl --fail --silent --show-error --location --retry 2 --retry-delay 2 \
  --max-time 60 --max-filesize 16777216 \
  https://opencode.ai/v2/openapi.json --output "$schema_candidate"
python3 tools/schema/check.py "$schema_candidate"
python3 -m unittest discover -s tools/schema -p 'test_*.py'
```

The schedule starts once the workflow reaches the default branch. GitHub may
delay scheduled runs and disables them after 60 days of inactivity in public
repositories; manual dispatch remains useful for release checks. See
[GitHub's scheduling rules](https://docs.github.com/en/actions/reference/workflows-and-actions/events-that-trigger-workflows#schedule).
