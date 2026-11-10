//! Reviewed integration, MCP, plugin and web-search discovery contracts.
use crate::{
    event_models::{enum_dispatch_tag, pascal},
    generate_with_settings, select_schemas,
    server_info::allow_additive_fields,
};
use serde_json::{Value, json};
use syn::{Item, parse_quote};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn generate_tool_discovery(spec: &Value) -> Result<String> {
    crate::forms::guard_shared_forms(spec)?;
    let reference = |name: &str| json!({"$ref":format!("#/components/schemas/{name}")});
    let mut spec = spec.clone();
    let roots = [
        (
            "IntegrationListResponse",
            "/api/integration",
            "Integration.Info",
            true,
        ),
        (
            "IntegrationResponse",
            "/api/integration/{integrationID}",
            "Integration.Info",
            false,
        ),
        ("McpListResponse", "/api/mcp", "Mcp.Server", true),
        (
            "McpResourceCatalogResponse",
            "/api/mcp/resource",
            "Mcp.ResourceCatalog",
            false,
        ),
        ("PluginListResponse", "/api/plugin", "Plugin.Info", true),
        (
            "WebSearchProvidersResponse",
            "/api/websearch/provider",
            "WebSearch.Provider",
            true,
        ),
    ];
    for (name, path, wire, array) in roots {
        let data = if array {
            json!({"type":"array","items":reference(wire)})
        } else {
            reference(wire)
        };
        let expected = json!({"type":"object","properties":{"location":reference("Location.PublicRef"),"data":data},"required":["location","data"],"additionalProperties":false});
        if spec["paths"][path]["get"]["responses"]["200"]["content"]["application/json"]["schema"]
            != expected
        {
            return Err(
                format!("{name} envelope changed; review shared types and metadata").into(),
            );
        }
        spec["components"]["schemas"][name] = expected;
    }
    for field in ["server", "tui", "rpc", "gui"] {
        if spec["components"]["schemas"]["Plugin.Features"]["properties"][field]
            != json!({"type":"boolean","enum":[true]})
        {
            return Err("plugin feature literals changed".into());
        }
    }
    let source = &spec["components"]["schemas"]["Plugin.Source"]["anyOf"][1];
    for field in ["outdated", "updating"] {
        if source["properties"][field] != json!({"type":"boolean","enum":[true]}) {
            return Err("plugin package flag literals changed".into());
        }
    }
    if spec["components"]["schemas"]["IntegrationNotFoundErrorEncoded"]
        != json!({"type":"object","properties":{"_tag":{"type":"string","enum":["IntegrationNotFoundError"]},"integrationID":{"type":"string"},"message":{"type":"string"}},"required":["_tag","integrationID","message"],"additionalProperties":false})
    {
        return Err("integration error contract changed".into());
    }
    let status = spec["components"]["schemas"]["Mcp.Server"]["properties"]["status"].clone();
    spec["components"]["schemas"]["McpStatus"] = status;
    spec["components"]["schemas"]["Mcp.Server"]["properties"]["status"] = reference("McpStatus");
    let mut settings = typify::TypeSpaceSettings::default();
    for (wire, rust) in [
        ("Location.PublicRef", "LocationPublicRef"),
        ("Form.Fields", "FormFields"),
        ("Integration.ID", "IntegrationId"),
        ("WebSearch.ID", "WebSearchId"),
    ] {
        if matches!(wire, "Integration.ID" | "WebSearch.ID")
            && spec["components"]["schemas"][wire] != json!({"type":"string"})
        {
            return Err("shared discovery ID changed".into());
        }
        spec["components"]["schemas"][wire] = json!({});
        settings.with_replacement(rust, format!("super::{rust}"), std::iter::empty());
    }
    settings.with_conversion(
        serde_json::from_value(json!({"type":"number"}))?,
        "::serde_json::Number",
        std::iter::empty(),
    );
    settings.with_conversion(
        serde_json::from_value(json!({"type":"boolean","enum":[true]}))?,
        "super::SchemaBoolean<true>",
        std::iter::empty(),
    );
    let mut schema = select_schemas(
        &spec,
        &roots.iter().map(|(name, ..)| *name).collect::<Vec<_>>(),
    )?;
    let mut dispatch = String::new();
    for (wire, rust, key, tags) in [
        (
            "Integration.Method",
            "IntegrationMethod",
            "type",
            vec!["oauth", "command", "key", "external", "env"],
        ),
        (
            "Connection.Info",
            "ConnectionInfo",
            "type",
            vec!["credential", "env"],
        ),
        (
            "McpStatus",
            "McpStatus",
            "status",
            vec!["connected", "pending", "disabled", "failed", "needs_auth"],
        ),
        (
            "Plugin.Source",
            "PluginSource",
            "type",
            vec!["builtin", "package", "local", "sdk"],
        ),
        (
            "Plugin.State",
            "PluginState",
            "status",
            vec!["active", "failed"],
        ),
    ] {
        dispatch_union(
            &mut schema,
            &mut settings,
            wire,
            rust,
            key,
            &tags,
            &mut dispatch,
        )?;
    }
    allow_additive_fields(&mut schema);
    let mut syntax = syn::parse_file(&generate_with_settings(schema, &settings)?)?;
    syntax.items.extend(syn::parse_file(&dispatch)?.items);
    let mut debug = Vec::new();
    let mut literal_adapters = 0;
    for item in &mut syntax.items {
        let (name, attrs) = match item {
            Item::Struct(s) => {
                if matches!(s.fields, syn::Fields::Named(_)) {
                    s.attrs.push(parse_quote!(#[non_exhaustive]));
                    for field in &mut s.fields {
                        // `true`-only flags must also reject explicit null.
                        let key = field.ident.as_ref().unwrap().to_string();
                        if (s.ident == "PluginFeatures"
                            && matches!(key.as_str(), "server" | "tui" | "rpc" | "gui"))
                            || (s.ident == "PluginSourcePackage"
                                && matches!(key.as_str(), "outdated" | "updating"))
                        {
                            field.attrs.push(parse_quote!(#[serde(default,deserialize_with="super::deserialize_optional_value")]));
                            literal_adapters += 1;
                        }
                    }
                }
                (&s.ident, &mut s.attrs)
            }
            Item::Enum(e) => {
                if !e.attrs.iter().any(|a| a.path().is_ident("non_exhaustive")) {
                    e.attrs.push(parse_quote!(#[non_exhaustive]));
                }
                (&e.ident, &mut e.attrs)
            }
            _ => continue,
        };
        for attr in attrs.iter_mut().filter(|a| a.path().is_ident("derive")) {
            let paths = attr.parse_args_with(
                syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated,
            )?;
            let paths: Vec<_> = paths.into_iter().filter(|p| !p.is_ident("Debug")).collect();
            *attr = parse_quote!(#[derive(#(#paths),*)]);
        }
        let label = format!("{name} {{ .. }}");
        debug.push(parse_quote!(impl ::std::fmt::Debug for #name {fn fmt(&self,f:&mut ::std::fmt::Formatter<'_>)->::std::fmt::Result {f.write_str(#label)}}));
    }
    if literal_adapters != 6 {
        return Err("plugin literal adapter count changed".into());
    }
    syntax.items.extend(debug);
    Ok(format!(
        "// Generated from openapi/opencode-v2.json by model-spike --tool-discovery.\n// Do not edit manually; adaptations live in tools/model-spike/src/tool_discovery.rs.\n{}",
        prettyplease::unparse(&syntax)
    ))
}

fn dispatch_union(
    schema: &mut Value,
    settings: &mut typify::TypeSpaceSettings,
    wire: &str,
    rust: &str,
    key: &str,
    tags: &[&str],
    out: &mut String,
) -> Result<()> {
    let branches = schema["definitions"][wire]["anyOf"]
        .as_array()
        .ok_or("discovery union changed")?
        .clone();
    let mut variants = Vec::new();
    for branch in branches {
        let branch = if let Some(reference) = branch["$ref"].as_str() {
            let name = reference
                .strip_prefix("#/definitions/")
                .ok_or("unexpected discovery reference")?;
            schema["definitions"]
                .as_object_mut()
                .unwrap()
                .remove(name)
                .ok_or("missing discovery branch")?
        } else {
            branch
        };
        let tag = branch["properties"][key]["enum"]
            .as_array()
            .filter(|v| v.len() == 1)
            .and_then(|v| v[0].as_str())
            .ok_or("discovery discriminator changed")?;
        if !branch["required"]
            .as_array()
            .is_some_and(|v| v.contains(&json!(key)))
        {
            return Err("discovery discriminator must remain required".into());
        }
        let variant = pascal(tag);
        let name = format!("{rust}{variant}");
        schema["definitions"][&name] = branch.clone();
        variants.push((variant, name, tag.to_owned()));
    }
    if variants
        .iter()
        .map(|(_, _, tag)| tag.as_str())
        .collect::<Vec<_>>()
        != tags
    {
        return Err("discovery variants changed; review explicit dispatch".into());
    }
    schema["definitions"][wire] = json!({});
    settings.with_replacement(pascal(wire), rust, std::iter::empty());
    out.push_str(&enum_dispatch_tag(rust, &variants, false, key));
    Ok(())
}
