#!/usr/bin/env bash
# Compile the full component output in a disposable crate, outside normal builds.
set -euo pipefail
repo_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$repo_dir"
cargo run --locked --offline -p opencoders-model-spike -- --all
probe_dir="$repo_dir/target/model-spike-full"
mkdir -p "$probe_dir/src"
cp target/model-spike-all.rs "$probe_dir/src/generated.rs"
cp tools/model-spike/src/compat.rs "$probe_dir/src/compat.rs"
cp Cargo.lock "$probe_dir/Cargo.lock"
cat > "$probe_dir/Cargo.toml" <<'MANIFEST'
[package]
name = "opencoders-full-model-probe"
version = "0.0.0"
edition = "2024"
publish = false

[workspace]

[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
regress = "=0.12.0"
MANIFEST
cat > "$probe_dir/src/lib.rs" <<'RUST'
pub mod compat;
pub mod generated;
RUST
# The copied lockfile preserves resolved dependency versions. Cargo adds only
# this disposable package and removes unused generator dependencies, offline.
cargo check --offline --manifest-path "$probe_dir/Cargo.toml" --target-dir "$repo_dir/target"
