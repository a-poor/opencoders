# Integration and tool discovery

[Documentation index](README.md)

Applications can use these six stable operations to populate integration settings
and tool catalogs:

| Resource | Methods | Returned data |
| --- | --- | --- |
| `client.integrations()` | `list`, `get` | Integration names, authentication methods and connection descriptors. |
| `client.mcp()` | `list`, `resource_catalog` | Configured MCP server states; resources and resource templates. |
| `client.plugins()` | `list` | Plugin sources, features and activation/failure states. |
| `client.websearch()` | `providers` | Registered search-provider IDs and names. |

Every method requires HTTP 200 and retains the `location`/`data` envelope,
additional object fields and server ordering. All have `_at` variants accepting
an explicit `Location`; `Location::Unspecified` omits the query and preserves
header/server fallback. Ordinary client deadlines, bounded response/error bodies,
authentication, and typed transport/HTTP/decode errors apply. The SDK does not
poll or add readiness waits to these calls.

```rust,no_run
use opencoders::{Client, Location};

async fn inspect(client: &Client) -> opencoders::Result<()> {
    let integrations = client.integrations().list().await?;
    if let Some(item) = integrations.data.first() {
        let details = client.integrations().get(&item.id).await?;
        // Use typed methods/forms to present connection choices.
    }
    let servers = client.mcp().list().await?;
    let catalog = client.mcp().resource_catalog().await?;
    let plugins = client.plugins().list_at(Location::directory("/server/project")).await?;
    let search = client.websearch().providers().await?;
    Ok(())
}
```

See the [runnable example](../examples/tool_discovery.rs), which prints counts
instead of raw payloads. Detailed generated types live in
`models::tool_discovery`; common response/enum types are also re-exported from
`models`. Integration and web-search IDs reuse the configuration model types.
Discovery objects redact Debug output. Explicit fields and serialization retain
all values, including form defaults, metadata, connection messages/URLs and plugin
error strings. Shared form fields keep the existing form API's behavior.

## Integration readiness and connection descriptors

On v2.2.5, `integration.list` waits for plugin activation, sorts integrations by
name, and omits entries denied by `integration.connect` policy. Integrations
marked with `metadata.source = "mcp"` bypass that list filter; MCP availability
has its own policy rules. `integration.get` reads the current registry directly:
it does not wait for activation or apply the list's filter. A cold lookup can
therefore return `ApiError::IntegrationNotFound`, and a registered integration
absent from the list can still be retrieved. This distinction describes the
pinned server, not a client authorization guarantee.

`IntegrationMethod` explicitly distinguishes OAuth, command, key, external and
environment methods. OAuth/key/external forms reuse the existing nonempty
`FormFields` response model, including conditions and special-number values.
Unknown method tags and malformed known variants fail decoding. Displaying a
command method does not execute its command; these operations perform no
authentication or credential writes.

`ConnectionInfo` describes saved credentials by ID/label/method or environment
connections by variable name. It does not contain credential values. Optional
runtime status can indicate `needs_auth` with a message and URL. The pinned core
orders saved credential descriptors newest first, followed by nonempty matching
environment variables; the SDK retains that order. This slice does not implement
the separate credential-list operation, which exposes secret values.

IDs are encoded as single path segments. Since integration IDs are unrestricted
strings, `get` rejects empty IDs and `.`/`..` locally rather than sending a
different route. `IntegrationNotFound` preserves the reported ID and message.

## MCP catalogs are partial observations

Configured MCP startup is asynchronous. `mcp.list` reports current `pending`,
`connected`, `disabled`, `failed` or `needs_auth` variants, sorted by server name
on v2.2.5. Failure/auth variants require error text. Title and integration ID are
optional; client-attached MCP connections are separate from this configured
server inventory. A list call is not a readiness barrier or a health probe.

`resource_catalog` contacts currently connected configured servers to request
resource and template listings. The server may retry/recover those MCP requests.
It tolerates unavailable template listing and omits failed/disconnected catalog
sources. Thus HTTP 200 or empty arrays do not establish that every server was
successfully queried. Each item retains its server name, resource URI or URI
template, and optional description/MIME type. The pinned server sorts by server,
name and URI/template; the SDK preserves the returned order and duplicates.
Resource contents are not fetched by this operation.

Applications own refresh policy and can respond to existing MCP/plugin/integration
events. Catalog reads and events are not an atomic snapshot across changing
registrations or connections. Initial location creation can itself start configured
plugins and MCP connections, even when the triggering SDK request is a GET.

## Plugin and search snapshots

`plugin.list` includes the current inventory and failed entries, without waiting
for activation or checking for package updates. Sources distinguish builtin,
package, local and SDK definitions. A failed load may lack an ID. Feature flags
(`server`, `tui`, `rpc`, `gui`) and package `outdated`/`updating` flags are optional
literal `true`; the SDK preserves absence and rejects false/null for those flags.
Unknown source/state tags and incomplete known variants are decoding errors.
Object extensions remain available in `extra` maps.

`websearch.providers` returns registered IDs/names sorted by name on v2.2.5.
It does not select a provider, check credentials, execute a query or wait for
plugin startup. Providers can remain registered when web search is disabled.

The [compatibility notes](../tools/live/README.md) distinguish disposable local
fixture evidence from source review. Real OAuth/credential flows, remote MCP
authentication, package updates, dynamic plugin reload and remote search execution
remain separate validation work.
