//! Reviewed workspace responses and strict ordinary PTY inputs.
use crate::{generate_with_settings, select_schemas, server_info::allow_additive_fields};
use serde_json::{Value, json};
use syn::{Item, Type, parse_quote};

type GenerationResult = Result<String, Box<dyn std::error::Error>>;

fn integer(positive: bool) -> Value {
    let mut value = json!({"type":"integer", "maximum":9_007_199_254_740_991_u64});
    value[if positive {
        "exclusiveMinimum"
    } else {
        "minimum"
    }] = json!(0);
    value
}

fn response(spec: &Value, path: &str, method: &str) -> Result<Value, String> {
    spec["paths"][path][method]["responses"]["200"]["content"]["application/json"]
        .get("schema")
        .cloned()
        .ok_or_else(|| format!("missing {method} {path} response"))
}

fn settings() -> typify::TypeSpaceSettings {
    let mut settings = typify::TypeSpaceSettings::default();
    for name in [
        "LocationPublicRef",
        "AbsolutePath",
        "RelativePath",
        "ProjectId",
        "FileDiffInfo",
        "FileDiffInfoStatus",
    ] {
        settings.with_replacement(name, format!("super::{name}"), std::iter::empty());
    }
    settings
}

pub fn generate_workspace(spec: &Value) -> GenerationResult {
    let mut spec = spec.clone();
    if response(&spec, "/api/fs/list", "get")? != response(&spec, "/api/fs/find", "get")? {
        return Err("filesystem list/find envelopes diverged; review shared response".into());
    }
    for field in ["additions", "deletions"] {
        if spec["components"]["schemas"]["Vcs.FileStatus"]["properties"][field] != integer(false) {
            return Err("Vcs.FileStatus count constraints changed".into());
        }
    }
    let base = response(&spec, "/api/vcs/base", "get")?;
    if base["properties"]["data"]
        != json!({"anyOf":[{"$ref":"#/components/schemas/Vcs.Base"},{"type":"null"}]})
        || base["required"] != json!(["location", "data"])
    {
        return Err("VCS base nullable envelope changed".into());
    }
    let roots = [
        ("FileListResponse", "/api/fs/list"),
        ("VcsResponse", "/api/vcs"),
        ("VcsBaseResponse", "/api/vcs/base"),
        ("VcsStatusResponse", "/api/vcs/status"),
        ("VcsBranchesResponse", "/api/vcs/branch"),
        ("VcsDiffResponse", "/api/vcs/diff"),
    ];
    for (name, path) in roots {
        spec["components"]["schemas"][name] = response(&spec, path, "get")?;
    }
    let mut names: Vec<_> = roots.iter().map(|(name, _)| *name).collect();
    names.extend(["Worktree.List", "Vcs.Mode", "Vcs.Untracked"]);
    if response(&spec, "/api/worktree", "get")?
        != json!({"$ref":"#/components/schemas/Worktree.List"})
    {
        return Err("worktree.list response changed".into());
    }
    let mut schema = select_schemas(&spec, &names)?;
    allow_additive_fields(&mut schema);
    let mut syntax = syn::parse_file(&generate_with_settings(schema, &settings())?)?;
    let mut adapted = 0;
    for item in &mut syntax.items {
        match item {
            Item::Struct(item) if matches!(item.fields, syn::Fields::Named(_)) => {
                item.attrs.push(parse_quote!(#[non_exhaustive]));
                for field in &mut item.fields {
                    let key = field.ident.as_ref().unwrap().to_string();
                    if item.ident == "VcsFileStatus"
                        && matches!(key.as_str(), "additions" | "deletions")
                    {
                        field.ty = parse_quote!(u64);
                        field.attrs.push(
                            parse_quote!(#[serde(deserialize_with="super::deserialize_pid")]),
                        );
                        adapted += 1;
                    }
                    if item.ident == "VcsBaseResponse" && key == "data" {
                        field.attrs.retain(|a| !a.path().is_ident("serde"));
                        field.attrs.push(parse_quote!(#[serde(deserialize_with="super::deserialize_required_nullable")]));
                        adapted += 1;
                    }
                }
            }
            Item::Enum(item) => item.attrs.push(parse_quote!(#[non_exhaustive])),
            _ => {}
        }
    }
    if adapted != 3 {
        return Err("workspace fields changed; review adapters".into());
    }
    Ok(format!(
        "// Generated from openapi/opencode-v2.json by model-spike --workspace.\n// Do not edit manually; adaptations live in tools/model-spike/src/workspace.rs.\n{}",
        prettyplease::unparse(&syntax)
    ))
}

pub fn generate_ptys(spec: &Value) -> GenerationResult {
    let mut spec = spec.clone();
    let create =
        spec["paths"]["/api/pty"]["post"]["requestBody"]["content"]["application/json"]["schema"]
            .clone();
    let update = spec["paths"]["/api/pty/{ptyID}"]["put"]["requestBody"]["content"]["application/json"]["schema"].clone();
    if create
        != json!({"type":"object","properties":{
        "command":{"type":"string"},"args":{"type":"array","items":{"type":"string"}},
        "cwd":{"type":"string"},"title":{"type":"string"},"env":{"type":"object","additionalProperties":{"type":"string"}}
    },"additionalProperties":false})
        || update
            != json!({"type":"object","properties":{
        "title":{"type":"string"},"size":{"type":"object","properties":{"rows":integer(true),"cols":integer(true)},"required":["rows","cols"],"additionalProperties":false}
    },"additionalProperties":false})
    {
        return Err("PTY input contract changed; review strict optional fields and size".into());
    }
    for field in ["pid", "exitCode"] {
        if spec["components"]["schemas"]["Pty"]["properties"][field] != integer(false) {
            return Err("PTY response integer bounds changed".into());
        }
    }
    if spec["components"]["schemas"]["Pty"]["required"]
        != json!(["id", "title", "command", "args", "cwd", "status", "pid"])
        || spec["components"]["schemas"]["PtyTicket.ConnectToken"]["properties"]["expires_in"]
            != integer(true)
        || spec["components"]["schemas"]["PtyTicket.ConnectToken"]["required"]
            != json!(["ticket", "expires_in"])
    {
        return Err("PTY response field optionality or ticket expiry changed".into());
    }
    let detail = response(&spec, "/api/pty/{ptyID}", "get")?;
    if detail != response(&spec, "/api/pty", "post")?
        || detail != response(&spec, "/api/pty/{ptyID}", "put")?
    {
        return Err("PTY detail/create/update envelopes diverged".into());
    }
    spec["components"]["schemas"]["PtyResponse"] = detail;
    spec["components"]["schemas"]["PtyListResponse"] = response(&spec, "/api/pty", "get")?;
    spec["components"]["schemas"]["PtyConnectTokenResponse"] =
        response(&spec, "/api/pty/{ptyID}/connect-token", "post")?;
    let mut schema = select_schemas(
        &spec,
        &["PtyResponse", "PtyListResponse", "PtyConnectTokenResponse"],
    )?;
    allow_additive_fields(&mut schema);
    spec["components"]["schemas"]["PtyCreateRequest"] = create;
    spec["components"]["schemas"]["PtyUpdateRequest"] = update;
    let inputs = select_schemas(&spec, &["PtyCreateRequest", "PtyUpdateRequest"])?;
    schema["definitions"]
        .as_object_mut()
        .unwrap()
        .extend(inputs["definitions"].as_object().unwrap().clone());
    let mut syntax = syn::parse_file(&generate_with_settings(schema, &settings())?)?;
    let mut adapted = 0;
    for item in &mut syntax.items {
        match item {
            Item::Struct(item) if matches!(item.fields, syn::Fields::Named(_)) => {
                item.attrs.push(parse_quote!(#[non_exhaustive]));
                let name = item.ident.to_string();
                if matches!(name.as_str(), "PtyCreateRequest" | "PtyTicketConnectToken") {
                    for attr in &mut item.attrs {
                        if attr.path().is_ident("derive") {
                            *attr = parse_quote!(#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]);
                        }
                    }
                }
                if name == "PtyCreateRequest" {
                    item.attrs.push(parse_quote!(#[derive(Default)]));
                }
                for field in &mut item.fields {
                    let key = field.ident.as_ref().unwrap().to_string();
                    if matches!(name.as_str(), "PtyCreateRequest" | "PtyUpdateRequest") {
                        let inner = option_inner(&field.ty).unwrap_or_else(|| field.ty.clone());
                        field.ty = parse_quote!(Option<#inner>);
                        field.attrs.retain(|a| !a.path().is_ident("serde"));
                        field.attrs.push(parse_quote!(#[serde(default,skip_serializing_if="Option::is_none",deserialize_with="super::deserialize_optional_value")]));
                        adapted += 1;
                    }
                    if (name == "Pty" && key == "pid")
                        || (name == "PtyTicketConnectToken" && key == "expires_in")
                        || name == "PtyUpdateRequestSize"
                    {
                        field.ty = parse_quote!(u64);
                        let decoder = if name == "Pty" {
                            "super::deserialize_pid"
                        } else {
                            "super::deserialize_positive_integer"
                        };
                        field
                            .attrs
                            .push(parse_quote!(#[serde(deserialize_with=#decoder)]));
                        adapted += 1;
                    }
                    if name == "Pty" && key == "exit_code" {
                        field.ty = parse_quote!(Option<u64>);
                        field.attrs.push(parse_quote!(#[serde(default,deserialize_with="super::deserialize_optional_safe_integer")]));
                        adapted += 1;
                    }
                }
            }
            Item::Enum(item) => item.attrs.push(parse_quote!(#[non_exhaustive])),
            _ => {}
        }
    }
    if adapted != 12 {
        return Err(format!("PTY adapter count changed: {adapted}").into());
    }
    Ok(format!(
        "// Generated from openapi/opencode-v2.json by model-spike --ptys.\n// Do not edit manually; adaptations live in tools/model-spike/src/workspace.rs.\n{}",
        prettyplease::unparse(&syntax)
    ))
}

fn option_inner(ty: &Type) -> Option<Type> {
    let Type::Path(path) = ty else {
        return None;
    };
    let segment = path.path.segments.last()?;
    if segment.ident != "Option" {
        return None;
    }
    let syn::PathArguments::AngleBracketed(args) = &segment.arguments else {
        return None;
    };
    match args.args.first()? {
        syn::GenericArgument::Type(ty) => Some(ty.clone()),
        _ => None,
    }
}
