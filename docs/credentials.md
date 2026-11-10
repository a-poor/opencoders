# Credentials and direct connections

[Documentation index](README.md)

Use `client.credentials()` for global stored credentials and
`client.integrations().connect_key` / `connect_external` to connect through a
location's registered integration methods. These seven operations are stable in
the pinned schema. Discover choices with the [integration catalog](tool-discovery.md).

| Method | Scope and result |
| --- | --- |
| `credentials().list()` | Global; HTTP 200 with `data: Vec<CredentialEntry>`, including secret values. |
| `credentials().create(&request)` | Global; HTTP 200 with the stored entry, including its secret value and selection state. |
| `credentials().update(&id, &request)` | Global; rename only, HTTP 204. |
| `credentials().activate(&id)` | Global; select the credential for its integration, HTTP 204. |
| `credentials().remove(&id)` | Global; delete the stored credential, HTTP 204. |
| `integrations().connect_key(&id, &request)` | Location registry/policy; stores a global key credential, HTTP 204. |
| `integrations().connect_external(&id, &request)` | Location registry/policy; stores a global external reference, HTTP 204. |

The five credential methods ignore the SDK default location and have no `_at`
variants. The two direct-connection methods use it and offer `_at` overrides;
`Location::Unspecified` leaves location resolution to headers/server defaults.
Credential response envelopes contain `data`, not `location`. Object extensions,
exact JSON numbers and server ordering are retained. IDs are encoded as single
path segments; empty IDs and `.`/`..` fail locally for methods addressing a URL ID.
An ID in a create body is not subject to that URL restriction.

## Secret-bearing data and request types

Listing reads every stored credential's value. Prefer integration discovery when
only names, method choices and connection descriptors are needed. Generated
credential/request/nested-value Debug output is redacted, as are ordinary SDK
error Display/Debug strings. Explicit fields, serialization, error bodies and
error sources retain potentially sensitive data. Do not send those to logs.
Values are ordinary owned Rust data, without a zeroization guarantee.

`CredentialValue` distinguishes key, OAuth and external response variants and
preserves added fields. `CredentialValueInput` uses separate strict variants that
reject unknown properties. OAuth expiry is a checked nonnegative integer up to
9,007,199,254,740,991; constructing an OAuth value stores supplied tokens and does
not perform an OAuth flow. Metadata is an optional JSON object: omission and an
explicit empty object are distinct, and null is rejected.

```rust,no_run
use opencoders::{Client, models::*};

async fn store(client: &Client, integration: IntegrationId, key: String)
    -> opencoders::Result<CredentialId>
{
    let request = CredentialCreateRequest::new(integration, CredentialValueInput::key(key))
        .label("Work account")
        .activate(false);
    let stored = client.credentials().create(&request).await?;
    Ok(stored.data.id)
}
```

Create's optional `id`, `label` and `activate` reject null and preserve omission,
false and empty strings. `CredentialUpdateRequest::new(label)` changes only the
label; it cannot update the secret, integration or method. Direct-connection
requests reuse `FormAnswer`, with `RequestField` for answer/label so omitted,
null and concrete values remain distinct on the wire.

## Selection and lifecycle on v2.2.5

Creation normally selects the new credential. `activate: false` preserves an
existing selection, but the first credential still becomes active. A supplied ID
that already exists returns `ApiError::Conflict`; it is not an idempotency key.
Raw creation checks global configuration's `integration.connect` policy and does
not require a registered integration. It does not validate provider access.
Omitted labels on raw creation become `default`.

Credential listings place each integration's selected credential last. `active`
describes that snapshot; applications should use the field instead of relying on
ordering. Rename, activate and remove return success for missing IDs on v2.2.5.
Activating an already-selected credential also succeeds. Removing the active
credential selects the newest remaining one or leaves none selected. Deletion
removes local storage; it does not revoke provider tokens or undo active work.

## Direct key and external methods

Call integration listing first when waiting for plugin activation is useful.
Direct connection handlers do not wait for activation themselves. Both check
the selected location's policy and registered integration; missing integrations
return `IntegrationNotFound`. Policy rejection is `InvalidRequest` with kind
`integration_policy`. The stored credentials remain global across locations.

On v2.2.5 a key connection validates the method's form and stores the supplied key
and answer metadata. An external connection validates the named method/form and
stores its method ID plus answer metadata; it does not resolve the external
secret. Neither operation verifies remote provider access. Invalid answers or a
missing external method produce `InvalidRequest` with kind
`integration_authorization`. A missing key method currently takes the server's
generic failure path; do not infer a typed method-not-found error for that case.

Omitted/null answers behave as an empty answer map; an explicit empty map remains
visible on the wire. Form validation stays on the server. Defaults are not filled
into the stored metadata by the SDK. Omitted/null labels choose an available
integration display name, adding a numeric suffix when needed; an empty label
remains empty. HTTP 204 does not return the created ID, so callers can refresh the
credential list or integration connections to observe the new state.

Mutations use ordinary client deadlines and never retry automatically. A timeout
or lost response can follow a successful write; inspect state before retrying.
The same applies to create even when the caller supplies an ID.

The [example](../examples/credentials.rs) reads credentials and prints only counts;
it makes no changes. The [compatibility notes](../tools/live/README.md) describe
isolated fake-credential storage and direct-method evidence. Real authentication,
OAuth/command attempts, provider revocation, token refresh, concurrent writers and
cross-process persistence remain separate work.
