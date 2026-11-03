use opencoders_model_spike::{generate, select_schemas};
use serde_json::Value;
use std::{env, error::Error, fs, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()?;
    let mut spec: Value =
        serde_json::from_slice(&fs::read(root.join("openapi/opencode-v2.json"))?)?;
    let args: Vec<_> = env::args().skip(1).collect();
    if args == ["--permissions"] || args == ["--permissions", "--check"] {
        let output = opencoders_model_spike::permissions::generate_permissions(&spec)?;
        let path = root.join("src/models/permissions.rs");
        if args.len() == 2 {
            if fs::read_to_string(&path)? != output {
                return Err("permission models are stale; regenerate with --permissions".into());
            }
            println!("Permission models match the pinned inputs");
        } else {
            fs::write(&path, output)?;
            println!("Generated {}", path.display());
        }
        return Ok(());
    }
    if args == ["--events"] || args == ["--events", "--check"] {
        let output = opencoders_model_spike::event_models::generate_events(&spec)?;
        let path = root.join("src/models/events.rs");
        if args.len() == 2 {
            if fs::read_to_string(&path)? != output {
                return Err("event models are stale; regenerate with --events".into());
            }
            println!("Event models match the pinned inputs");
        } else {
            fs::write(&path, output)?;
            println!("Generated {}", path.display());
        }
        return Ok(());
    }
    if args == ["--inbox"] || args == ["--inbox", "--check"] {
        let output = opencoders_model_spike::inbox::generate_inbox(&spec)?;
        let path = root.join("src/models/inbox.rs");
        if args.len() == 2 {
            if fs::read_to_string(&path)? != output {
                return Err("inbox models are stale; regenerate with --inbox".into());
            }
            println!("Inbox models match the pinned inputs");
        } else {
            fs::write(&path, output)?;
            println!("Generated {}", path.display());
        }
        return Ok(());
    }
    if args == ["--messages"] || args == ["--messages", "--check"] {
        let output = opencoders_model_spike::messages::generate_messages(&spec)?;
        let path = root.join("src/models/messages.rs");
        if args.len() == 2 {
            if fs::read_to_string(&path)? != output {
                return Err("message models are stale; regenerate with --messages".into());
            }
            println!("Message models match the pinned inputs");
        } else {
            fs::write(&path, output)?;
            println!("Generated {}", path.display());
        }
        return Ok(());
    }
    if args == ["--sessions"] || args == ["--sessions", "--check"] {
        let output = opencoders_model_spike::sessions::generate_sessions(&spec)?;
        let path = root.join("src/models/sessions.rs");
        if args.len() == 2 {
            if fs::read_to_string(&path)? != output {
                return Err("session models are stale; regenerate with --sessions".into());
            }
            println!("Session models match the pinned inputs");
        } else {
            fs::create_dir_all(path.parent().unwrap())?;
            fs::write(&path, output)?;
            println!("Generated {}", path.display());
        }
        return Ok(());
    }
    if args == ["--discovery"] || args == ["--discovery", "--check"] {
        let output = opencoders_model_spike::discovery::generate_discovery(&spec)?;
        let path = root.join("src/models/discovery.rs");
        if args.len() == 2 {
            if fs::read_to_string(&path)? != output {
                return Err("discovery models are stale; regenerate with --discovery".into());
            }
            println!("Discovery models match the pinned inputs");
        } else {
            fs::create_dir_all(path.parent().unwrap())?;
            fs::write(&path, output)?;
            println!("Generated {}", path.display());
        }
        return Ok(());
    }
    if args == ["--server-info"] || args == ["--server-info", "--check"] {
        let output = opencoders_model_spike::server_info::generate_server_info(&spec)?;
        let path = root.join("src/models/generated.rs");
        if args.len() == 2 {
            if fs::read_to_string(&path)? != output {
                return Err("server-info models are stale; regenerate with --server-info".into());
            }
            println!("Server-info models match the pinned inputs");
        } else {
            fs::create_dir_all(path.parent().unwrap())?;
            fs::write(&path, output)?;
            println!("Generated {}", path.display());
        }
        return Ok(());
    }
    let all = args.iter().any(|arg| arg == "--all");
    let raw = args.iter().any(|arg| arg == "--raw");
    let check = args.iter().any(|arg| arg == "--check");
    if args
        .iter()
        .any(|arg| !["--all", "--raw", "--check"].contains(&arg.as_str()))
        || (check && (all || raw))
    {
        return Err(
            "usage: model-spike [--check | --all [--raw] | --raw | --server-info [--check] | --discovery [--check] | --sessions [--check] | --messages [--check] | --inbox [--check] | --events [--check] | --permissions [--check]]".into(),
        );
    }
    let config: Value = serde_json::from_str(include_str!("../probes.json"))?;
    let names: Vec<_> = if all {
        spec["components"]["schemas"]
            .as_object()
            .unwrap()
            .keys()
            .cloned()
            .collect()
    } else {
        for (name, pointer) in config["inline_schemas"].as_object().unwrap() {
            let schema = spec
                .pointer(pointer.as_str().unwrap())
                .ok_or("inline schema not found")?
                .clone();
            spec["components"]["schemas"][name] = schema;
        }
        for (name, schema) in config["synthetic_schemas"].as_object().unwrap() {
            spec["components"]["schemas"][name] = schema.clone();
        }
        config["roots"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap().to_owned())
            .collect()
    };
    let names: Vec<_> = names.iter().map(String::as_str).collect();
    let schema = select_schemas(&spec, &names)?;
    let output = generate(schema, !raw)?;
    let path = root.join(if raw {
        "target/model-spike-raw.rs"
    } else if all {
        "target/model-spike-all.rs"
    } else {
        "tools/model-spike/src/generated.rs"
    });
    if check {
        if fs::read_to_string(&path)? != output {
            return Err("generated models are stale; run model-spike without --check".into());
        }
        println!("Generated models match the pinned inputs");
    } else {
        fs::create_dir_all(path.parent().unwrap())?;
        fs::write(&path, output)?;
        println!("Generated {}", path.display());
    }
    Ok(())
}
