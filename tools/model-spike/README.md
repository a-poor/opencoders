# Model-generation experiment

Decision: use **Typify 0.8.0 as the wire-model generation backend**, with explicit
overrides and hand-written request/event adapters. Keep generated wire types
behind a reviewed public API. This private workspace package is a feasibility
experiment, not the implementation of the `opencoders` client.

The root crate exposes reviewed server-info, discovery, and session models and HTTP methods.
Workspace `default-members` keeps this tool out of ordinary `cargo build` and
`cargo test`; use `--workspace` or `-p` deliberately. Nothing runs generation at
build time. Generated probe and production model files are checked in separately.

## Reviewed server-information models

This tool also provides the first production generation slice:

```sh
cargo run --locked --offline -p opencoders-model-spike -- --server-info
cargo run --locked --offline -p opencoders-model-spike -- --server-info --check
```

The input is the pinned `ServerInfo` schema. Output goes to
`src/models/generated.rs` in the root crate. `src/server_info.rs` contains the
reviewable adaptations: preserve added fields in flattened maps at all object
levels, mark response structs non-exhaustive, and deserialize `pid` as a bounded
unsigned integer. A guard rejects changes to the upstream pid schema for review;
the runtime validator lives in the root crate's `src/models/mod.rs`.

These adaptations apply only to this response slice. The original raw experiment
and its characterization tests remain unchanged. Broader model promotion is still
a separate task.

## Reviewed discovery models

```sh
cargo run --locked --offline -p opencoders-model-spike -- --discovery
cargo run --locked --offline -p opencoders-model-spike -- --discovery --check
```

Output goes to the root `src/models/discovery.rs`. The inline `agent.list`
envelope, location responses, and transitive agent/provider models are selected
from the pinned schema. Adaptations in `src/discovery.rs` preserve additive
fields, make response objects/enums non-exhaustive, and enforce positive
safe-integer `steps` while preserving absence. A guarded simplification maps the
overlapping `"auto" | string` interval union to string. A guarded named replacement
uses the root `ProviderTimeout` adapter for numbers or exactly `false`; raw Typify
accepts both booleans. Numeric JSON retains integer precision. Provider requests
use a redacted handwritten `Debug` implementation; cache TTL regex validation
uses `regress`.

The public model tests cover these wire behaviors and the tool tests require
review when adapted schema constraints change. Unknown string enum variants are
decode errors. Most optional response fields accept null as absent; `steps`
rejects explicit null. These models are not update-request types.

## Reviewed session-list models

```sh
cargo run --locked --offline -p opencoders-model-spike -- --sessions
cargo run --locked --offline -p opencoders-model-spike -- --sessions --check
```

Output goes to the root `src/models/sessions.rs`, selecting `SessionsResponse`
and its transitive dependencies. `src/sessions.rs` records adaptations: shared
discovery types are reused through named replacements, arbitrary numeric fields
use `serde_json::Number`, and guarded adapters enforce file-diff count bounds and
the optional true-only `more` flag. Generated ID prefix checks remain intact.
Response objects and union variants preserve added fields; unknown enum values
fail decoding. Optional response fields generally accept null as absent, except
the strict `more` adapter. Cursor omission/null both become `None`; empty strings
stay present. This response model policy does not define update-request semantics.

Run all production checks (`--server-info --check`, `--discovery --check`, and
`--sessions --check`) after schema/tool changes, alongside the raw probe `--check`.
Normal library builds use the checked-in output without invoking the tool.

## Run the experiment

From the repository root, fetch the locked dependencies once if needed:

```sh
cargo fetch --locked
```

Then all of the following work offline:

```sh
# Regenerate the small, checked-in probe model set.
cargo run --locked --offline -p opencoders-model-spike
# Verify generation without writing files.
cargo run --locked --offline -p opencoders-model-spike -- --check
# Run behavioral, characterization, and reference-selection tests.
cargo test --locked --offline -p opencoders-model-spike
# Generate all upstream components and compile them in a disposable crate.
bash tools/model-spike/check-all.sh
```

`check-all.sh` writes only under `target/`. It copies the workspace lockfile into
the disposable crate; Cargo adjusts package membership offline while reusing the
locked dependency versions. The full component output is not checked in or used
by the root library. It does not cover every inline operation request/response.

To reproduce the unadapted generator limitation, run:

```sh
cargo run --locked --offline -p opencoders-model-spike -- --all --raw
```

This command is **expected to fail** with Typify's unimplemented non-null `not`
schema panic. It is a diagnostic, not a required passing check.

## Inputs and reproducibility

- The exact upstream bytes and provenance live in [`openapi/`](../../openapi/).
- `probes.json` selects roots, extracts the actual session-update request body,
  and defines clearly labeled synthetic probes. References are resolved locally
  and renamed from `#/components/schemas/` to `#/definitions/` for Typify's input.
- The probe selects 34 definitions including dependencies. Reduced message/error/
  event unions use actual upstream variants. The recursion probe is synthetic:
  the pinned component reference graph has no cycles.
- `src/lib.rs` contains the exact-match non-null schema override. The upstream
  snapshot is never rewritten. `src/compat.rs` contains its runtime validation.
- `src/generated.rs` is produced by Typify and prettyplease using `Cargo.lock`.
  Rustfmt skips this generated module; edit inputs or the generator, not output.
- Verified initially with Rust 1.99.1. This does not establish the library's MSRV.

Reference rewriting is not a general OpenAPI 3.1-to-draft-07 conversion. Successful
generation or compilation does not prove complete JSON Schema validation.

## Findings

| Probe | Result and consequence |
| --- | --- |
| All 490 component schemas | Generate and compile with the non-null override. This establishes mechanical feasibility, not endpoint or behavioral coverage. |
| `not: {type: "null"}` | Unmodified Typify panics. An exact conversion to `NonNullValue` preserves all non-null JSON kinds and rejects null. |
| Messages and structured errors | Reduced unions decode correctly. Typify emits untagged unions with constrained tag fields for these probes; use explicit dispatch for public event/error handling. |
| Optional nullable update fields | Raw generation collapses omission and null. The tested `Field<T>` adapter preserves omission, null, and a value; the `SessionTitlePatch` probe covers only the actual title field. Server-side meaning of null remains unverified. |
| Unknown events | Explicit dispatch preserves raw unknown events while rejecting malformed known variants. Dispatch is intentionally limited to two known variants in this experiment. |
| SSE `contentSchema` | Generation yields a string without validating its JSON content. The adapter parses SSE data text once and dispatches it; this is not an SSE framing implementation. |
| Additive response fields | Raw models enforce `additionalProperties: false`. A deliberate response-tolerance policy is still required before production use, including nested objects. |
| Numeric bounds | Raw `Event.Seq` accepts negative integers despite its minimum. Generated deserialization is not a complete schema validator; review constraints and public numeric types. |
| Recursion | The synthetic recursive model generates boxed indirection and round-trips. |
| Durable log marker | Optional sequence metadata survives decoding/encoding. Replay ordering and server behavior are outside this experiment. |

Tests named `raw_*` characterize generator limitations. They do not define the
desired public client contract. Keep these findings distinct from client bugs:
the root library uses separately adapted production output, not the probe
models.

## Alternatives and next steps

[Typify's TypeSpace API](https://docs.rs/typify/0.8.0/typify/struct.TypeSpace.html)
allows explicit model generation and conversion overrides.
[Progenitor 0.15.1](https://docs.rs/progenitor/0.15.1/progenitor/) documents an
OpenAPI 3.0.x client-generation interface. It was screened out on documented
scope, not executed against this schema; a full-client generator is not needed
for the chosen architecture.

Before broader production models are exposed:

1. Define response tolerance, unknown enum/error policy, and tag dispatch across
   the full event/error sets. Keep malformed known values as errors.
2. Apply three-state fields to requests where omission and null must be preserved;
   verify actual server semantics and other absent-versus-empty distinctions.
3. Review numeric ranges, ID validation, nested dynamic data, and inline endpoint
   schemas. Fail explicitly on unsupported schema constructs in the maintained
   generation pipeline.
4. Establish the public resource/request/response naming and module layout, then
   move the proven generation machinery into the production maintainer command.
5. Exercise real behavior separately using the [pinned server smoke test](../live/README.md).
   Expand live coverage as request and stream support grows.

These follow-ups belong to the foundation milestone in `TODO.md`; this experiment
does not commit the crate to Typify's generated public interface.
