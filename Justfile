# Run `cargo install just`. Then run `just` to list available recipes.

default:
  just --list

# Perform the one-time local setup required for this crate.
once:
  just enable-wasm
  just install-tools

# Install the tools this crate depends on for local development.
install-tools:
  cargo install just
  cargo install cargo-deny
  cargo install cargo-semver-checks
  cargo install cargo-sort

# Enable the WASM target required by the wasm compile check.
enable-wasm:
  rustup target add wasm32-unknown-unknown

# Format the crate.
fmt:
  cargo fmt --all

# Run format checks.
fmt-check:
  cargo fmt --all --check

# Sort the dependencies in `Cargo.toml`.
sort:
  cargo sort --workspace

# Check that the dependencies in `Cargo.toml` are sorted.
sort-check:
  cargo sort --workspace --check

# Run clippy.
clippy:
  cargo clippy --all-targets --all-features -- -D warnings

# Type-check the crate with every feature, including the Leptos integration.
check:
  cargo check --all-targets --all-features

# Type-check the default shape without the `leptos` feature.
check-no-leptos:
  cargo check --all-targets

# Run clippy without the `leptos` feature.
clippy-no-leptos:
  cargo clippy --all-targets -- -D warnings

# Run the tests that do not need the `leptos` feature.
test-no-leptos:
  cargo test

# Verify the library compiles for the wasm32-unknown-unknown target.
check-wasm:
  cargo check --target wasm32-unknown-unknown --all-features --locked

# Run unit tests, integration tests, and doctests for the native target.
test:
  cargo test --all-features

# Run only the crate unit tests.
test-lib:
  cargo test --lib --all-features

# Run only the crate doc tests.
test-doc:
  cargo test --doc --all-features

# Build documentation with rustdoc warnings denied (matches CI).
doc:
  RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features --locked

# Run cargo-deny's supply-chain checks (advisories, licenses, bans, sources).
deny:
  cargo deny check

# Detect breaking public-API changes vs. the latest published release.
semver-check:
  cargo semver-checks

# Clean build artifacts for this crate.
clean:
  cargo clean

# Update dependencies, sort manifests, and format code.
tidy:
  cargo update --workspace
  just sort
  just fmt

# Run the full non-mutating validation suite.
verify:
  just fmt-check
  just sort-check
  just check
  just check-no-leptos
  just check-wasm
  just clippy
  just clippy-no-leptos
  just test
  just test-no-leptos
  just doc
