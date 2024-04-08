default:
    just --list --unsorted

# Fmt
fmt:
    cargo fmt -- --check

# Build the entire project
build-dev:
    cargo build

# Build the entire project
build:
    cargo build --release

# Run clippy
clippy:
    cargo clippy -- -Dclippy::all -D warnings

# Test
test:
    cargo test

# CI
ci:
    just fmt
    just clippy
    just test
    just build-dev
