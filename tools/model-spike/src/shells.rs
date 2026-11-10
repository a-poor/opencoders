//! Reviewed stable shell inputs and envelopes sharing the event payloads.
use crate::{generate_with_settings, select_schemas, server_info::allow_additive_fields};
use serde_json::{Value, json};
use syn::{Item, parse_quote};

pub fn generate_shells(spec: &Value) -> Result<String, Box<dyn std::error::Error>> {
    let reference = |name: &str| json!({"$ref":format!("#/components/schemas/{name}")});
    let safe = json!({"type":"integer","minimum":0,"maximum":9007199254740991_u64});
    let body = |path: &str, method: &str| {
        spec["paths"][path][method]["requestBody"]["content"]["application/json"]["schema"].clone()
    };
    let create = body("/api/shell", "post");
    let session = body("/api/session/{sessionID}/shell", "post");
    let environment = body("/api/session/{sessionID}/environment", "put");
    if create
        != json!({"type":"object","properties":{"command":{"type":"string"},"cwd":{"type":"string"},"timeout":safe,"metadata":{"type":"object"}},"required":["command"],"additionalProperties":false})
        || session
            != json!({"type":"object","properties":{"command":{"type":"string"},"id":{"anyOf":[reference("Session.Message.ID"),{"type":"null"}]}},"required":["command"],"additionalProperties":false})
        || environment
            != json!({"type":"object","properties":{"variables":{"type":"object","additionalProperties":{"type":"string"}}},"required":["variables"],"additionalProperties":false})
    {
        return Err(
            "shell input contract changed; review strict fields, nullability and safe integers"
                .into(),
        );
    }
    let output = json!({"type":"object","properties":{"output":{"type":"string"},"cursor":safe,"size":safe,"truncated":{"type":"boolean"}},"required":["output","cursor","size","truncated"],"additionalProperties":false});
    if spec["components"]["schemas"]["session.shell.ended"]["properties"]["data"]["properties"]["output"]
        != output
        || spec["components"]["schemas"]["Shell.Info"]["properties"]["pid"] != safe
        || spec["components"]["schemas"]["Shell.ID"] != json!({"type":"string","pattern":"^sh_"})
    {
        return Err("shared shell event contract changed; review records and output".into());
    }
    let query = &spec["paths"]["/api/shell/{id}/output"]["get"]["parameters"];
    for name in ["cursor", "limit"] {
        let expected = json!({"name":name,"in":"query","schema":{"type":"string","pattern":r"^[+-]?(?:\d+(?:\.\d+)?|\.\d+)(?:[Ee][+-]?\d+)?$"},"required":false});
        if !query
            .as_array()
            .is_some_and(|items| items.contains(&expected))
        {
            return Err("shell output query changed; review numeric-string encoding and upstream refinement".into());
        }
    }
    for (path, method) in [
        ("/api/shell/{id}", "delete"),
        ("/api/session/{sessionID}/shell", "post"),
        ("/api/session/{sessionID}/environment", "put"),
    ] {
        if spec["paths"][path][method]["responses"]["204"] != json!({"description":"<No Content>"})
        {
            return Err("shell mutation success status changed".into());
        }
    }
    let mut spec = spec.clone();
    let roots = [
        (
            "ShellResponse",
            "/api/shell",
            "post",
            reference("Shell.Info"),
        ),
        (
            "ShellResponse",
            "/api/shell/{id}",
            "get",
            reference("Shell.Info"),
        ),
        (
            "ShellListResponse",
            "/api/shell",
            "get",
            json!({"type":"array","items":reference("Shell.Info")}),
        ),
        (
            "ShellOutputResponse",
            "/api/shell/{id}/output",
            "get",
            output,
        ),
    ];
    for (name, path, method, data) in &roots {
        let response=spec["paths"][*path][*method]["responses"]["200"]["content"]["application/json"]["schema"].clone();
        if response
            != json!({"type":"object","properties":{"location":reference("Location.PublicRef"),"data":data},"required":["location","data"],"additionalProperties":false})
        {
            return Err(format!("{name} changed; review envelope/shared payload").into());
        }
        spec["components"]["schemas"][*name] = response;
    }
    spec["components"]["schemas"]["ShellOutput"] = roots[3].3.clone();
    spec["components"]["schemas"]["ShellOutputResponse"]["properties"]["data"] =
        reference("ShellOutput");
    let mut schema = select_schemas(
        &spec,
        &["ShellResponse", "ShellListResponse", "ShellOutputResponse"],
    )?;
    allow_additive_fields(&mut schema);
    for (name, input) in [
        ("ShellCreateRequest", create),
        ("SessionShellRequest", session),
        ("SessionEnvironmentRequest", environment),
    ] {
        spec["components"]["schemas"][name] = input;
    }
    let inputs = select_schemas(
        &spec,
        &[
            "ShellCreateRequest",
            "SessionShellRequest",
            "SessionEnvironmentRequest",
        ],
    )?;
    schema["definitions"]
        .as_object_mut()
        .unwrap()
        .extend(inputs["definitions"].as_object().unwrap().clone());
    let mut settings = typify::TypeSpaceSettings::default();
    for name in [
        "ShellId",
        "SessionMessageId",
        "LocationPublicRef",
        "ShellInfo",
        "ShellOutput",
        "AbsolutePath",
        "ProjectId",
    ] {
        settings.with_replacement(name, format!("super::{name}"), std::iter::empty());
    }
    let mut syntax = syn::parse_file(&generate_with_settings(schema, &settings)?)?;
    let mut debug = String::new();
    let mut adapted = 0;
    for item in &mut syntax.items {
        if let Item::Struct(item) = item {
            if !matches!(item.fields, syn::Fields::Named(_)) {
                continue;
            }
            item.attrs.push(parse_quote!(#[non_exhaustive]));
            for attr in &mut item.attrs {
                if attr.path().is_ident("derive") {
                    *attr = parse_quote!(#[derive(::serde::Deserialize,::serde::Serialize,Clone)]);
                }
            }
            let name = item.ident.to_string();
            debug.push_str(&format!("impl ::std::fmt::Debug for {name} {{fn fmt(&self,f:&mut ::std::fmt::Formatter<'_>)->::std::fmt::Result {{ f.debug_struct(\"{name}\").finish_non_exhaustive() }} }}"));
            for field in &mut item.fields {
                let key = field.ident.as_ref().unwrap().to_string();
                if name == "ShellCreateRequest" && key != "command" {
                    let ty: syn::Type = match key.as_str() {
                        "cwd" => parse_quote!(String),
                        "timeout" => parse_quote!(u64),
                        "metadata" => parse_quote!(::serde_json::Map<String,::serde_json::Value>),
                        _ => return Err("unknown shell input field".into()),
                    };
                    field.ty = parse_quote!(Option<#ty>);
                    field.attrs.clear();
                    field.attrs.push(parse_quote!(#[serde(default,skip_serializing_if="Option::is_none",deserialize_with="super::deserialize_optional_value")]));
                    if key == "timeout" {
                        field.attrs.clear();
                        field.attrs.push(parse_quote!(#[serde(default,skip_serializing_if="Option::is_none",deserialize_with="super::deserialize_optional_safe_integer",serialize_with="super::shell_inputs::serialize_timeout")]));
                    }
                    adapted += 1;
                }
                if name == "SessionShellRequest" && key == "id" {
                    field.ty = parse_quote!(crate::RequestField<super::SessionMessageId>);
                    field.attrs.clear();
                    field.attrs.push(parse_quote!(#[serde(default,skip_serializing_if="crate::RequestField::is_omitted")]));
                    adapted += 1;
                }
            }
        }
    }
    if adapted != 4 {
        return Err("shell adapter count changed".into());
    }
    Ok(format!(
        "// Generated from openapi/opencode-v2.json by model-spike --shells.\n// Do not edit manually; adaptations live in tools/model-spike/src/shells.rs.\n{}{}",
        prettyplease::unparse(&syntax),
        prettyplease::unparse(&syn::parse_file(&debug)?)
    ))
}
