# Package and release validation

Version 0.1.0 is unreleased. This procedure validates a candidate without uploading
it. See [release readiness](RELEASE_READINESS.md) for the validated clean candidate
and hosted Linux/macOS results, and the [changelog](CHANGELOG.md) for
consumer-facing limits.

## Archive policy

The `Cargo.toml` include list keeps Rust sources, all sixteen examples and their
support modules, integration tests and JSON fixtures, README, `docs/` guides,
changelog, and both licenses. Keeping tests lets consumers verify the extracted source distribution.
The two live tests remain ignored and require the repository's separate runners.

Planning documents, CI, pinned schema, generation experiment, coverage tools and
live-server runners stay in the repository. Links to those resources use
repository URLs; guides, examples and tests are available in both locations. Normal
library builds use checked-in models and never run a generator or fetch a schema.
Cargo also supplies the normalized manifest, original manifest, minimized lockfile
and VCS metadata; inspect these along with the explicit files.

## Validate the candidate

Start from the intended release revision. Complete the repository checks in
[RELEASE_READINESS.md](RELEASE_READINESS.md#4-collect-release-candidate-validation).
Use the normal Cargo cache after fetching locked dependencies; `--offline` keeps
the following build checks independent of network access.

```sh
cargo package -p opencoders --list --locked --offline
cargo package -p opencoders --locked --offline
tar -tzf target/package/opencoders-0.1.0.crate
```

Review for missing fixtures/licenses and unintended local files. Cargo's package
command builds an extracted archive, but does not run its tests. For those, use a
new directory outside the checkout (adjust the version when preparing a release):

```sh
package_review_dir=$(mktemp -d)
tar -xzf target/package/opencoders-0.1.0.crate -C "$package_review_dir"
cd "$package_review_dir/opencoders-0.1.0"
cargo test --locked --offline
cargo test --locked --offline --no-default-features
cargo check --examples --locked --offline
cargo check --examples --locked --offline --no-default-features
cargo +1.88.0 check --lib --examples --locked --offline
cargo +1.88.0 check --lib --examples --locked --offline --no-default-features
RUSTDOCFLAGS='-D warnings' cargo doc --no-deps --locked --offline
```

Do not pass `--ignored`; package validation must not start OpenCode or submit work.
Use a fresh build directory for the archive so repository artifacts cannot hide a
missing input. Inspect the normalized `Cargo.toml`: it must not require workspace
members, local path dependencies or generation. Record the archive checksum,
revision, toolchain, platform and results.

From the original checkout, exercise the publishing path without uploading.
Cargo needs registry network access for this step, even with `--dry-run`:

```sh
cargo publish -p opencoders --dry-run --locked --registry crates-io
```

During development, `--allow-dirty` may be added to package/publish commands;
record that evidence as a working-tree check, not a clean release artifact.
Repeat from the final clean revision before release. A dry run does not reserve
a crate name, establish publishing permission, prove server-side acceptance or
produce a docs.rs build. See Cargo's official
[package](https://doc.rust-lang.org/cargo/commands/cargo-package.html) and
[publish](https://doc.rust-lang.org/cargo/commands/cargo-publish.html) references.

## Release handoff

Confirm hosted CI on the intended revision. Recorded live compatibility evidence
can be reused when the SDK, fixtures, workflows and server baseline are unchanged;
otherwise dispatch **Pinned server compatibility** with `active_execution` enabled
to run both smoke and active jobs on Linux/macOS.
Update the changelog date, confirm the README dependency matches the release
version, and rerun package validation after those final edits. Publishing and creating a version tag are
separate, explicit actions; this procedure does neither. After an authorized publication, verify registry
metadata and docs.rs before announcing compatibility or availability.
