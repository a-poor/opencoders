//! Reviewed global credential storage and location-scoped direct connections.
use crate::{
    event_models::enum_dispatch_tag, generate_with_settings, select_schemas,
    server_info::allow_additive_fields,
};
use serde_json::{Value, json};
use syn::{Item, parse_quote};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn generate_credentials(spec: &Value) -> Result<String> {
    crate::forms::guard_shared_forms(spec)?;
    let reference = |name: &str| json!({"$ref":format!("#/components/schemas/{name}")});
    let body = |path: &str, method: &str| {
        spec["paths"][path][method]["requestBody"]["content"]["application/json"]["schema"].clone()
    };
    let nullable = |value: Value| json!({"anyOf":[value,{"type":"null"}]});
    let mut spec = spec.clone();
    let create = spec["components"]["schemas"]["Credential.CreateInput"].clone();
    let update = body("/api/credential/{credentialID}", "patch");
    let key = body("/api/integration/{integrationID}/connect/key", "post");
    let external = body("/api/integration/{integrationID}/connect/external", "post");
    if body("/api/credential", "post") != reference("Credential.CreateInput")
        || create
            != json!({"type":"object","properties":{"id":reference("Credential.ID"),"integrationID":reference("Integration.ID"),"label":{"type":"string"},"value":reference("Credential.Value"),"activate":{"type":"boolean"}},"required":["integrationID","value"],"additionalProperties":false})
        || update
            != json!({"type":"object","properties":{"label":{"type":"string"}},"required":["label"],"additionalProperties":false})
        || key
            != json!({"type":"object","properties":{"key":{"type":"string"},"answer":nullable(reference("Form.Answer")),"label":nullable(json!({"type":"string"}))},"required":["key"],"additionalProperties":false})
        || external
            != json!({"type":"object","properties":{"methodID":reference("Integration.MethodID"),"answer":nullable(reference("Form.Answer")),"label":nullable(json!({"type":"string"}))},"required":["methodID"],"additionalProperties":false})
    {
        return Err("credential/direct-connection input contract changed".into());
    }
    let safe = json!({"type":"integer","minimum":0,"maximum":9007199254740991_u64});
    for (name, expected) in [
        (
            "Credential.Key",
            json!({"type":"object","properties":{"type":{"type":"string","enum":["key"]},"key":{"type":"string"},"metadata":{"type":"object"}},"required":["type","key"],"additionalProperties":false}),
        ),
        (
            "Credential.External",
            json!({"type":"object","properties":{"type":{"type":"string","enum":["external"]},"methodID":reference("Integration.MethodID"),"metadata":{"type":"object"}},"required":["type","methodID"],"additionalProperties":false}),
        ),
        (
            "Credential.OAuth",
            json!({"type":"object","properties":{"type":{"type":"string","enum":["oauth"]},"methodID":reference("Integration.MethodID"),"refresh":{"type":"string"},"access":{"type":"string"},"expires":safe,"metadata":{"type":"object"}},"required":["type","methodID","refresh","access","expires"],"additionalProperties":false}),
        ),
        (
            "Credential.Value",
            json!({"anyOf":[reference("Credential.OAuth"),reference("Credential.Key"),reference("Credential.External")]}),
        ),
    ] {
        if spec["components"]["schemas"][name] != expected {
            return Err(format!("{name} changed; review strict inputs and tagged dispatch").into());
        }
    }
    for (path, method) in [
        ("/api/credential/{credentialID}", "patch"),
        ("/api/credential/{credentialID}", "delete"),
        ("/api/credential/{credentialID}/activate", "post"),
        ("/api/integration/{integrationID}/connect/key", "post"),
        ("/api/integration/{integrationID}/connect/external", "post"),
    ] {
        if spec["paths"][path][method]["responses"]["204"] != json!({"description":"<No Content>"})
        {
            return Err("credential mutation status changed".into());
        }
    }
    for (name, method, data) in [
        (
            "CredentialListResponse",
            "get",
            json!({"type":"array","items":reference("Credential.Entry")}),
        ),
        ("CredentialResponse", "post", reference("Credential.Entry")),
    ] {
        let expected = json!({"type":"object","properties":{"data":data},"required":["data"],"additionalProperties":false});
        if spec["paths"]["/api/credential"][method]["responses"]["200"]["content"]["application/json"]
            ["schema"]
            != expected
            || spec["paths"]["/api/credential"][method]["parameters"] != json!([])
        {
            return Err("global credential envelope/scope changed".into());
        }
        spec["components"]["schemas"][name] = expected;
    }
    let mut settings = typify::TypeSpaceSettings::default();
    for (wire, rust) in [
        ("Credential.ID", "CredentialId"),
        ("Integration.ID", "IntegrationId"),
        ("Integration.MethodID", "IntegrationMethodId"),
        ("Form.Answer", "FormAnswer"),
    ] {
        if wire != "Form.Answer" && spec["components"]["schemas"][wire] != json!({"type":"string"})
        {
            return Err("shared credential identifier changed".into());
        }
        spec["components"]["schemas"][wire] = json!({});
        settings.with_replacement(rust, format!("super::{rust}"), std::iter::empty());
    }
    settings.with_conversion(
        serde_json::from_value(safe)?,
        "super::CredentialExpires",
        std::iter::empty(),
    );
    let mut schema = select_schemas(&spec, &["CredentialListResponse", "CredentialResponse"])?;
    allow_additive_fields(&mut schema);
    for (name, input) in [
        ("CredentialCreateRequest", create),
        ("CredentialUpdateRequest", update),
        ("IntegrationConnectKeyRequest", key),
        ("IntegrationConnectExternalRequest", external),
    ] {
        spec["components"]["schemas"][name] = input;
    }
    spec["components"]["schemas"]["CredentialCreateRequest"]["properties"]["value"] =
        reference("CredentialValueInput");
    spec["components"]["schemas"]["CredentialValueInput"] = json!({"anyOf":[reference("CredentialOauthInput"),reference("CredentialKeyInput"),reference("CredentialExternalInput")]});
    for (source, target) in [
        ("Credential.OAuth", "CredentialOauthInput"),
        ("Credential.Key", "CredentialKeyInput"),
        ("Credential.External", "CredentialExternalInput"),
    ] {
        spec["components"]["schemas"][target] = spec["components"]["schemas"][source].clone();
    }
    let inputs = select_schemas(
        &spec,
        &[
            "CredentialCreateRequest",
            "CredentialUpdateRequest",
            "IntegrationConnectKeyRequest",
            "IntegrationConnectExternalRequest",
        ],
    )?;
    schema["definitions"]
        .as_object_mut()
        .unwrap()
        .extend(inputs["definitions"].as_object().unwrap().clone());
    let mut dispatch = String::new();
    for (wire, rust, input) in [
        ("Credential.Value", "CredentialValue", false),
        ("CredentialValueInput", "CredentialValueInput", true),
    ] {
        let mut variants = Vec::new();
        for (suffix, tag) in [("Oauth", "oauth"), ("Key", "key"), ("External", "external")] {
            let name = format!("Credential{suffix}{}", if input { "Input" } else { "" });
            if !input {
                let old = format!(
                    "Credential.{}",
                    if suffix == "Oauth" { "OAuth" } else { suffix }
                );
                let branch = schema["definitions"]
                    .as_object_mut()
                    .unwrap()
                    .remove(&old)
                    .ok_or("missing credential branch")?;
                schema["definitions"][&name] = branch;
            }
            variants.push((suffix.to_owned(), name, tag.to_owned()));
        }
        schema["definitions"][wire] = json!({});
        settings.with_replacement(rust, rust, std::iter::empty());
        dispatch.push_str(&enum_dispatch_tag(rust, &variants, false, "type"));
    }
    let mut syntax = syn::parse_file(&generate_with_settings(schema, &settings)?)?;
    syntax.items.extend(syn::parse_file(&dispatch)?.items);
    let mut debug = Vec::new();
    let mut adapted = 0;
    for item in &mut syntax.items {
        let (name, attrs) = match item {
            Item::Struct(s) => {
                if matches!(s.fields, syn::Fields::Named(_)) {
                    s.attrs.push(parse_quote!(#[non_exhaustive]));
                    for field in &mut s.fields {
                        let key = field.ident.as_ref().unwrap().to_string();
                        let name = s.ident.to_string();
                        let nullable = name.starts_with("IntegrationConnect")
                            && matches!(key.as_str(), "answer" | "label");
                        let optional = (name == "CredentialCreateRequest"
                            && matches!(key.as_str(), "id" | "label" | "activate"))
                            || key == "metadata";
                        if nullable || optional {
                            let inner: syn::Type = match key.as_str() {
                                "answer" => parse_quote!(super::FormAnswer),
                                "label" => parse_quote!(String),
                                "id" => parse_quote!(super::CredentialId),
                                "activate" => parse_quote!(bool),
                                "metadata" => {
                                    parse_quote!(::serde_json::Map<String,::serde_json::Value>)
                                }
                                _ => unreachable!(),
                            };
                            field.attrs.clear();
                            if nullable {
                                field.ty = parse_quote!(crate::RequestField<#inner>);
                                field.attrs.push(parse_quote!(#[serde(default,skip_serializing_if="crate::RequestField::is_omitted")]));
                            } else {
                                field.ty = parse_quote!(Option<#inner>);
                                field.attrs.push(parse_quote!(#[serde(default,skip_serializing_if="Option::is_none",deserialize_with="super::deserialize_optional_value")]));
                            }
                            adapted += 1;
                        }
                    }
                }
                (&s.ident, &mut s.attrs)
            }
            Item::Enum(e) => (&e.ident, &mut e.attrs),
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
    if adapted != 13 {
        return Err(format!("credential adapter count changed: {adapted}").into());
    }
    syntax.items.extend(debug);
    Ok(format!(
        "// Generated from openapi/opencode-v2.json by model-spike --credentials.\n// Do not edit manually; adaptations live in tools/model-spike/src/credentials.rs.\n{}",
        prettyplease::unparse(&syntax)
    ))
}
