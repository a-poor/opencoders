# Model-generation experiment

Decision: use **Typify 0.8.0 as the wire-model generation backend**, with explicit
overrides and hand-written request/event adapters. Keep generated wire types
behind a reviewed public API. This private workspace package is a feasibility
experiment, not the implementation of the `opencoders` client.

The root crate exposes reviewed server-info, discovery, session, message, inbox,
event, permission, form, workspace and PTY models through hand-written HTTP/stream methods.
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

Output goes to the root `src/models/discovery.rs`. Inline agent list/get and model
list/default envelopes, location responses, and transitive agent/provider/model types are selected
from the pinned schema. Adaptations in `src/discovery.rs` preserve additive
fields, make response objects/enums non-exhaustive, and enforce positive
safe-integer `steps` while preserving absence. A guarded simplification maps the
overlapping `"auto" | string` interval union to string. A guarded named replacement
uses the root `ProviderTimeout` adapter for numbers or exactly `false`; raw Typify
accepts both booleans. Numeric JSON retains integer precision. Provider requests
use a redacted handwritten `Debug` implementation; cache TTL regex validation
uses `regress`.

Guarded `Model.Type` and `Model.ReasoningField` unions simplify to open strings.
Model limits and cost-tier sizes enforce signed safe-integer bounds (including
negative values allowed by the schema); optional input limits reject explicit null.
Default-model `data` must be present but can be null; its adapter preserves null on
serialization. `ModelInfo` and `ModelVariant` use redacted debug output because
their headers/body/settings can contain credentials. Adapters live in the root
`src/models/catalog_adapters.rs`; schema guards require review when shapes change.

The public model tests cover these wire behaviors and the tool tests require
review when adapted schema constraints change. Unknown closed string enum variants are
decode errors. Most optional response fields accept null as absent; `steps`
rejects explicit null. These models are not update-request types.

## Reviewed session models

```sh
cargo run --locked --offline -p opencoders-model-spike -- --sessions
cargo run --locked --offline -p opencoders-model-spike -- --sessions --check
```

Output goes to the root `src/models/sessions.rs`, selecting `SessionsResponse`,
the inline create/get/fork envelopes (`SessionResponse`), the inline creation,
update, and fork requests (`SessionCreateRequest`, `SessionUpdateRequest`, and
`SessionForkRequest`), selection requests
(`SessionSwitchAgentRequest` and `SessionSwitchModelRequest`),
`SessionInterruptResponse`, and their dependencies. The interrupt response keeps
its required boolean and additive response fields without a custom adapter.
`src/sessions.rs` records adaptations: shared
discovery types are reused through named replacements, arbitrary numeric fields
use `serde_json::Number`, and guarded adapters enforce file-diff count bounds and
the optional true-only `more` flag. Generated ID prefix checks remain intact.
Response objects and union variants preserve added fields; unknown enum values
fail decoding. Optional response fields generally accept null as absent, except
the strict `more` adapter. Cursor omission/null both become `None`; empty strings
stay present. This response model policy does not define update-request semantics.

Input definitions retain `additionalProperties: false` and use separate strict
location/model/permission types. All eight create fields, four update fields, and
fork `before` use the public `RequestField<T>` adapter, preserving omission, null,
and values. The nested model variant rejects explicit null. Guards check nullable/optional field shapes,
the matching create/get/fork envelopes, the exact fork message-ID reference, and
variant constraints before generation.
Hand-written constructors/setters live in the root `src/models/session_inputs.rs`.
Selection bodies are guarded strict objects with required non-null fields; the
model body reuses `SessionModelInput` rather than tolerating response extensions.

Run [all fifteen generation checks](#verify-checked-in-models) after schema/tool changes.
Normal library builds use the checked-in output without invoking the tool.

## Reviewed message models

```sh
cargo run --locked --offline -p opencoders-model-spike -- --messages
cargo run --locked --offline -p opencoders-model-spike -- --messages --check
```

Output goes to the root `src/models/messages.rs`, selecting message pages, the
inline get envelope and type filter, and their dependencies. Shared session IDs,
locations/models, costs, and token counts reuse existing types. Responses preserve
additive fields, non-exhaustive types, and numeric precision.

Raw Typify flattens `Session.Message.Info` into optional variant fields. A named
replacement supplies the handwritten eleven-kind `SessionMessageInfo` enum.
Explicit tag dispatch also covers assistant content, tool states, compaction,
and tool results; unknown tags and malformed known values are errors. Guards
check union members and tag literals before applying these adapters. Boxed message
payloads and the larger assistant tool variant keep enum sizes bounded.

Guarded field adapters enforce shell-output safe integers, positive retry attempts,
retry delays, HTTP status bounds, and nonempty tool content when supplied. The
runtime adapters live in `src/models/message_adapters.rs` in the root crate.
Optional constrained fields reject null; absent optional tool-error content is
allowed. Message attachments and native/provider data are response models, not
reviewed request inputs. The live test imports fixed history without model calls.

## Reviewed prompt and inbox models

```sh
cargo run --locked --offline -p opencoders-model-spike -- --inbox
cargo run --locked --offline -p opencoders-model-spike -- --inbox --check
```

Output goes to `src/models/inbox.rs`. The tool extracts inline prompt submission,
inbox listing, and delivery-update contracts. Shared IDs, locations, and prepared
attachment response models reuse existing types. Responses preserve extensions
and number precision. `SessionInboxInfo` dispatches all four kinds explicitly;
`NonNullValue` enforces the opaque compaction payload's non-null constraint.

Prompt inputs are strict and separate from prepared response objects. Three
nullable optional fields use `RequestField<T>`; non-null optional collections/maps
use `Option<T>` so empty values remain distinct from omission. Nested optional
input fields reject explicit null. A handwritten decoder rejects extra fields
that Typify's adjacent-tag file-source decoder otherwise accepts, including an
unexpected null `uri` on an inline source. Guards check request shapes, optionality,
source unions, inbox tags, and opaque payload constraints before adaptation.
Adapters/constructors live in the root `src/models/inbox_adapters.rs` and
`src/models/prompt_inputs.rs`. CI verifies all fifteen generation targets, including the slices below.

## Reviewed live-event models

```sh
cargo run --locked --offline -p opencoders-model-spike -- --events
cargo run --locked --offline -p opencoders-model-spike -- --events --check
```

`event-roots.json` records the 62 selected live/log wire kinds, component schemas, and
public variants. `src/event_models.rs` emits root `src/models/events.rs`, including
explicit outer and nested inbox/status/form tag dispatch. Other kinds remain raw
JSON; malformed selected kinds fail decoding. This production path is separate
from the original two-kind experiment in `src/events.rs`.

Shared response types are reused. `Location.Ref` stays distinct from the public
location reference. Guarded scalar conversions enforce safe integers and exact
durable versions; nonempty array wrappers avoid Typify's tuple representation.
Optional constrained values reject null. Event fixtures exercise every selected
kind; nested union tests verify required fields and extension retention. The
generator rejects changed root tags, RPC patterns, unreviewed numeric constraints,
and nonempty-array/tagged-union shapes.

Instruction-update deltas normalize the reviewed hash-or-`"removed"` `anyOf`
to equivalent disjoint `oneOf` branches before generation. This avoids Typify's
flattened-object representation for scalar values. Guards reject changes to the
hash pattern or sentinel before assuming disjointness. Regression tests reject
malformed hashes, nulls and objects; the unfiltered active recovery fixture covers
real hash-bearing instruction events.

The same path generates `EventLogSynced` and guarded durable-event accessors for
all 47 members of `Session.Event.Durable`. Two of these kinds are only in the
pinned durable union, so root membership accepts either reviewed event union.
`SessionLogItem` explicitly dispatches markers, reviewed durable events, and raw
unknowns; known transient events are errors. Marker sequences reject explicit
null and preserve absence. Shell PID/output constraints use checked wrappers;
message content reuses the existing strict assistant-content union. Guards check
watermark optionality, durable envelopes, and the shared content replacement.
The generated marker description qualifies the upstream wording with the verified
retention limitation. Generation remains offline and explicit.


## Reviewed permission models

```sh
cargo run --locked --offline -p opencoders-model-spike -- --permissions
cargo run --locked --offline -p opencoders-model-spike -- --permissions --check
```

The output `src/models/permissions.rs` includes pending request/list/get envelopes,
immediate create results, and strict create/reply inputs. Shared permission IDs,
reply enums, and response sources reuse existing event types; effect, agent,
session, and location types also remain shared. Responses preserve extensions.

Guards check create/reply field sets, optionality, and the strict tool-source
shape. Adapters use `RequestField<T>` for create id/agent and reply message, and
`Option<T>` with non-null decoding for save/metadata/source. Explicit empty input
collections remain present. Response collections default to empty and omit empty
values on serialization, consistent with event payloads. Constructors live in
`src/models/permission_inputs.rs`; no local approval policy is generated.

## Reviewed form models

```sh
cargo run --locked --offline -p opencoders-model-spike -- --forms
cargo run --locked --offline -p opencoders-model-spike -- --forms --check
```

`src/models/forms.rs` contains discovery/lifecycle envelopes, strict inputs for
all six field kinds, and explicit pending/answered/cancelled dispatch. Constructors
and the nonempty field-list wrapper live in `src/models/form_inputs.rs`. Responses
reuse event IDs/fields/answers/metadata only after structural-equivalence checks.
Strict inputs preserve omitted/empty/false, nullable create ID uses `RequestField`,
and all other optional inputs reject null. Guards check nonempty arrays, numeric
constraints, request/state shapes, and union tags.

The shared event generator now normalizes form answer/condition unions by removing
a redundant enum branch already covered by arbitrary strings. Without that step,
Typify emitted flattened structs that could not decode actual scalar answers.
Regression tests cover strings (including special-number spellings), precise JSON
numbers, booleans, and answer string arrays. Input numeric limits/defaults retain
the separate number/special-string union. Server-owned semantic validation is not
generated into the client. Run both `--events --check` and `--forms --check`.

## Reviewed workspace and PTY models

`--workspace` selects filesystem list/find envelopes, VCS info/base/status/branches/
diff, project worktree lists/create responses and strict create/refresh/remove
inputs into `src/models/workspace.rs`. `--ptys` selects
ordinary lifecycle/ticket responses and strict create/update inputs into
`src/models/ptys.rs`. Each accepts `--check`. Adaptations live in `src/workspace.rs`.
Shared paths, project IDs, location metadata and file diffs reuse existing types.
Worktree guards require review when request shapes, explicit removal force or
create response contracts change. Optional create fields reject null while
preserving omission and empty strings. Responses retain additive fields and
non-exhaustive types; unknown closed enums
remain decode errors. VCS base data must be present but may be null.

Guards check matching envelopes, request shapes, nullable-base semantics and
numeric constraints. Runtime adapters enforce safe-integer PID/exit/count values
and positive ticket lifetime/terminal dimensions. Optional request fields reject
null while preserving empty collections/maps separately from omission. Explicit
null exit codes are rejected. Request/ticket Debug redacts sensitive values.
The root `workspace_adapters.rs` supplies constructors and validators. File bytes
and WebSocket messages are hand-written transports, not generated JSON responses.

## Reviewed session workflows

`--workflows` generates command/skill catalogs, process activity, strict
command/compaction/revert/view/synthetic inputs and their response envelopes.
Existing generated/adapted message, inbox,
attachment, diff and revert types are external leaves rather than duplicates.
The inbox guard owns attachment validation; the workflow guard verifies strict
inputs, nullability and shared response references. Run all targets together to
check the owners of reused types. Eight nullable input fields use `RequestField`;
three optional attachment arrays and synthetic metadata distinguish absence from
empty and reject null. View timestamps use `serde_json::Number`. The active map
adapter guards the exact `^ses` pattern and shared Session ID definition, then
uses validated `SessionId` keys with generated tolerant state values. Unknown
state enum variants fail decoding; additive fields remain available in `extra`.

## Reviewed stable shell APIs

`--shells` generates strict standalone/session shell inputs and location-bearing
envelopes. The event target owns the shared `ShellInfo` and output payloads, so
polling, events and numeric validation stay consistent. The guard requires equal
output shapes and reviewed request, status and query contracts. Safe-integer
timeout validation applies on serialization and decoding; optional fields preserve
omission/empty values and the session message ID uses `RequestField`.
New inputs/envelopes and shared shell record/output Debug implementations redact
payloads. Run `--events` and all other checks alongside `--shells`.

## Reviewed app-setup discovery

`--setup` generates configuration entries and nested settings, shell options,
projects, providers and references. Array/envelope guards retain the six route
shapes; existing IDs/paths/settings/permissions/model contracts are shared leaves.
Config-specific settings retain null-capable arbitrary overlays. A guarded
normalization of `allOf` with unrestricted values prevents Typify from losing
known property types. Timeout unions reuse `ProviderTimeout`; bounded integers
and literal booleans use checked `SchemaInteger`/`SchemaBoolean` adapters.
Known string/tagged unions remain validated; response extensions are retained.
`ConfigEntry` is a guarded adapter with a boxed document variant, keeping small
directory entries from occupying the full nested configuration size.
All setup model Debug implementations redact values, including nested config
and response extensions. Shared `ProviderSettings` is redacted in its owning
discovery target. Root types are re-exported; detailed types are public under
`models::setup`. No configuration mutation API is implied by these response types.

## Reviewed integration and tool discovery

`--tool-discovery` generates the six location/data envelopes and nested integration,
MCP, plugin and search-provider models. Integration/search IDs reuse `--setup`;
authentication forms reuse `FormFields` with the same response/event equivalence
guard as `--forms`. Five unions use explicit discriminator dispatch. Optional
plugin feature/update flags accept only literal true and reject explicit null.

Guards cover envelope shape, shared IDs/forms, union tags, literal flags and the
handwritten missing-integration error. Additive response fields and exact JSON
numbers are preserved. Every generated Debug implementation redacts values;
public detailed types live in `models::tool_discovery`. No mutation input or
credential-write contract is implied. Check all targets together for shared types.

## Verify checked-in models

From the repository root, use the same Rust/rustfmt **1.99.1** toolchain as CI.
Install that toolchain with rustfmt and fetch the locked dependencies once before
running these offline checks. The subshell keeps the toolchain override local:

```sh
(
  set -e
  export RUSTUP_TOOLCHAIN=1.99.1
  cargo run --locked --offline -p opencoders-model-spike -- --check
  cargo run --locked --offline -p opencoders-model-spike -- --server-info --check
  cargo run --locked --offline -p opencoders-model-spike -- --discovery --check
  cargo run --locked --offline -p opencoders-model-spike -- --sessions --check
  cargo run --locked --offline -p opencoders-model-spike -- --messages --check
  cargo run --locked --offline -p opencoders-model-spike -- --inbox --check
  cargo run --locked --offline -p opencoders-model-spike -- --events --check
  cargo run --locked --offline -p opencoders-model-spike -- --permissions --check
  cargo run --locked --offline -p opencoders-model-spike -- --forms --check
  cargo run --locked --offline -p opencoders-model-spike -- --workspace --check
  cargo run --locked --offline -p opencoders-model-spike -- --ptys --check
  cargo run --locked --offline -p opencoders-model-spike -- --workflows --check
  cargo run --locked --offline -p opencoders-model-spike -- --setup --check
  cargo run --locked --offline -p opencoders-model-spike -- --shells --check
  cargo run --locked --offline -p opencoders-model-spike -- --tool-discovery --check
)
```

Each `--check` compares generated text without rewriting it. To regenerate a
reviewed slice, use the same toolchain and omit only `--check` from its command.
The diagnostic full-schema experiment below is separate from these fifteen checks.

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
| All 497 component schemas | Generate and compile with the non-null override (verified after the 2026-11-09 schema update). This establishes mechanical feasibility, not endpoint or behavioral coverage. |
| `not: {type: "null"}` | Unmodified Typify panics. An exact conversion to `NonNullValue` preserves all non-null JSON kinds and rejects null. |
| Messages and structured errors | Reduced unions decode correctly. Typify emits untagged unions with constrained tag fields for these probes; use explicit dispatch for public event/error handling. |
| Optional nullable update fields | Raw generation collapses omission and null. The tested `Field<T>` adapter preserves omission, null, and a value; the `SessionTitlePatch` probe covers only the actual title field. Production models use `RequestField<T>`; separate v2.2.5 live checks verify all four null update fields are no-ops. |
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

The `--permissions` target also generates saved-rule ID/info/time envelopes and
strict action/resource patches. Both nullable patch fields use `RequestField`;
timestamps use `serde_json::Number`, and project IDs reuse the shared model.
Contract guards require review when IDs, input optionality or envelope references
change. This remains part of the existing fifteen-target generation check.
