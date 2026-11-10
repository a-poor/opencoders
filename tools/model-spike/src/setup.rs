//! Reviewed read-only application setup contracts.
use crate::{generate_with_settings, select_schemas, server_info::allow_additive_fields};
use serde_json::{Value, json};
use syn::{Item, parse_quote};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn generate_setup(spec: &Value) -> Result<String> {
    let reference = |name: &str| json!({"$ref":format!("#/components/schemas/{name}")});
    let mut spec = spec.clone();
    let roots = [
        ("ConfigResponse", "/api/config", "Config.Entry", true, false),
        (
            "ConfigShellsResponse",
            "/api/config/shell",
            "ConfigShell.Option",
            true,
            false,
        ),
        (
            "ProjectListResponse",
            "/api/project",
            "Project",
            true,
            false,
        ),
        (
            "ProviderListResponse",
            "/api/provider",
            "Provider.Info",
            true,
            true,
        ),
        (
            "ProviderResponse",
            "/api/provider/{providerID}",
            "Provider.Info",
            false,
            true,
        ),
        (
            "ReferenceListResponse",
            "/api/reference",
            "Reference.Info",
            true,
            true,
        ),
    ];
    for (name, path, wire, array, envelope) in roots {
        let data = if array {
            json!({"type":"array","items":reference(wire)})
        } else {
            reference(wire)
        };
        let expected = if envelope {
            json!({"type":"object","properties":{"location":reference("Location.PublicRef"),"data":data},"required":["location","data"],"additionalProperties":false})
        } else {
            data
        };
        let response = &spec["paths"][path]["get"]["responses"]["200"]["content"]["application/json"]
            ["schema"];
        if response != &expected {
            return Err(
                format!("{name} response changed; review array/envelope and shared types").into(),
            );
        }
        spec["components"]["schemas"][name] = expected;
    }
    // Existing targets own these validated/shared contracts. Config-specific
    // settings remain separate because their arbitrary overlays permit nulls.
    let shared = [
        ("AbsolutePath", "AbsolutePath"),
        ("Project.ID", "ProjectId"),
        ("Provider.ID", "ProviderId"),
        ("Location.PublicRef", "LocationPublicRef"),
        ("Permission.Ruleset", "PermissionRuleset"),
        ("Provider.Settings", "ProviderSettings"),
        ("Provider.Cache", "ProviderCache"),
        ("Provider.Compaction", "ProviderCompaction"),
        ("Provider.Transport", "ProviderTransport"),
        ("Provider.Encoding", "ProviderEncoding"),
        ("Model.Compatibility", "ModelCompatibility"),
        ("Model.Type", "ModelType"),
        ("Model.ReasoningField", "ModelReasoningField"),
        ("Model.MaxTokensField", "ModelMaxTokensField"),
        ("Model.MediaInput", "ModelMediaInput"),
        ("Model.CacheFormat", "ModelCacheFormat"),
        ("Money.USDPerMillionTokens", "MoneyUsdPerMillionTokens"),
    ];
    let mut settings = typify::TypeSpaceSettings::default();
    if spec["components"]["schemas"]["Config.Entry"]
        != json!({"anyOf":[reference("Config.DocumentEncoded"),reference("Config.DirectoryEncoded")]})
    {
        return Err("config entry variants changed; review boxed document adapter".into());
    }
    settings.with_replacement("ConfigEntry", "super::ConfigEntry", std::iter::empty());
    for (wire, name) in shared {
        if spec["components"]["schemas"].get(wire).is_none() {
            return Err(format!("missing shared schema {wire}").into());
        }
        spec["components"]["schemas"][wire] = json!({});
        settings.with_replacement(name, format!("super::{name}"), std::iter::empty());
    }
    let timeout = json!({"anyOf":[{"type":"number"},{"type":"boolean","enum":[false]}]});
    // `{} | null` accepts every JSON value. Normalize this equivalent open
    // object intersection so Typify retains the named, typed known properties.
    for name in [
        "Config.Provider.Settings",
        "Config.Model.Settings",
        "Config.Model.VariantSettings",
    ] {
        let object = &mut spec["components"]["schemas"][name];
        if object["allOf"]
            != json!([{"type":"object","additionalProperties":{"anyOf":[{}, {"type":"null"}]}}])
        {
            return Err("configuration overlay changed; review open-object normalization".into());
        }
        object.as_object_mut().unwrap().remove("allOf");
        object["additionalProperties"] = json!({});
    }
    for field in ["timeout", "headerTimeout", "chunkTimeout"] {
        let value =
            &mut spec["components"]["schemas"]["Config.Provider.Settings"]["properties"][field];
        if *value != timeout {
            return Err("configuration timeout changed; review false-or-number adapter".into());
        }
        *value = reference("SetupTimeout");
    }
    spec["components"]["schemas"]["SetupTimeout"] = json!({});
    settings.with_replacement("SetupTimeout", "super::ProviderTimeout", std::iter::empty());
    let interval =
        &mut spec["components"]["schemas"]["Config.WarmingEncoded"]["properties"]["interval"];
    if *interval != json!({"anyOf":[{"type":"string","enum":["auto"]},{"type":"string"}]}) {
        return Err("config warming interval changed".into());
    }
    *interval = json!({"type":"string"});
    let search =
        &mut spec["components"]["schemas"]["ConfigWebSearch.InfoEncoded"]["properties"]["provider"];
    if search["anyOf"][0]["enum"] != json!(["random"])
        || search["anyOf"][1] != reference("WebSearch.ID")
        || spec["components"]["schemas"]["WebSearch.ID"] != json!({"type":"string"})
    {
        return Err("config websearch provider union changed".into());
    }
    spec["components"]["schemas"]["ConfigWebSearch.InfoEncoded"]["properties"]["provider"] =
        reference("WebSearch.ID");
    let names: Vec<_> = roots.iter().map(|(n, ..)| *n).collect();
    let mut schema = select_schemas(&spec, &names)?;
    guard_scalars(&schema)?;
    for (minimum, maximum, positive) in [
        (-9_007_199_254_740_991_i64, 9_007_199_254_740_991_i64, false),
        (0, 9_007_199_254_740_991, false),
        (0, 9_007_199_254_740_991, true),
        (1, 65535, false),
    ] {
        let key = if positive {
            "exclusiveMinimum"
        } else {
            "minimum"
        };
        let shape = json!({"type":"integer",key:minimum,"maximum":maximum});
        let min = minimum + i64::from(positive);
        settings.with_conversion(
            serde_json::from_value(shape)?,
            format!("super::SchemaInteger<{min}, {maximum}>"),
            std::iter::empty(),
        );
    }
    for value in [false, true] {
        settings.with_conversion(
            serde_json::from_value(json!({"type":"boolean","enum":[value]}))?,
            format!("super::SchemaBoolean<{value}>"),
            std::iter::empty(),
        );
    }
    settings.with_conversion(
        serde_json::from_value(json!({"type":"number"}))?,
        "::serde_json::Number",
        std::iter::empty(),
    );
    allow_additive_fields(&mut schema);
    let mut syntax = syn::parse_file(&generate_with_settings(schema, &settings)?)?;
    let mut debug = Vec::new();
    for item in &mut syntax.items {
        let (name, attrs) = match item {
            Item::Struct(s) => {
                if matches!(s.fields, syn::Fields::Named(_)) {
                    s.attrs.push(parse_quote!(#[non_exhaustive]));
                }
                (&s.ident, &mut s.attrs)
            }
            Item::Enum(e) => {
                e.attrs.push(parse_quote!(#[non_exhaustive]));
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
        debug.push(parse_quote!(impl ::std::fmt::Debug for #name {
            fn fmt(&self,f:&mut ::std::fmt::Formatter<'_>)->::std::fmt::Result { f.write_str(#label) }
        }));
    }
    syntax.items.extend(debug);
    Ok(format!(
        "// Generated from openapi/opencode-v2.json by model-spike --setup.\n// Do not edit manually; adaptations live in tools/model-spike/src/setup.rs.\n{}",
        prettyplease::unparse(&syntax)
    ))
}

fn guard_scalars(value: &Value) -> Result<()> {
    if let Value::Object(map) = value {
        if map.get("type") == Some(&json!("integer")) {
            let mut shape = value.clone();
            shape.as_object_mut().unwrap().remove("description");
            let allowed = [
                json!({"type":"integer","minimum":-9_007_199_254_740_991_i64,"maximum":9_007_199_254_740_991_i64}),
                json!({"type":"integer","minimum":0,"maximum":9_007_199_254_740_991_i64}),
                json!({"type":"integer","exclusiveMinimum":0,"maximum":9_007_199_254_740_991_i64}),
                json!({"type":"integer","minimum":1,"maximum":65535}),
            ];
            if !allowed.contains(&shape) {
                return Err("setup integer bounds changed; review scalar adapter".into());
            }
        }
        for v in map.values() {
            guard_scalars(v)?;
        }
    } else if let Value::Array(values) = value {
        for v in values {
            guard_scalars(v)?;
        }
    }
    Ok(())
}
