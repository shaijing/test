# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Minimal Rust binary project using Rust Edition 2024. Depends on `ferroid` crate (v1.0.2) with features: `ulid`, `basic`, `atomic`, `serde`.

## Development Commands

```bash
cargo build              # Development build
cargo build --release    # Release build (size-optimized)
cargo run                # Run the binary
cargo check              # Quick error check
cargo clippy             # Lint with Clippy
cargo clean              # Clean build artifacts
```

## Release Configuration

Aggressive size optimization in `Cargo.toml`:
- `opt-level = 'z'`, `lto = true`, `codegen-units = 1`, `panic = 'abort'`, `strip = true`

## Target Configuration

Windows static linking via `.cargo/config.toml`:
- Target: `x86_64-pc-windows-msvc`
- Rustflags: `-C target-feature=+crt-static` (static C runtime)

## CI/CD

GitHub Actions (`build-release-binaries.yml`) triggers on version tags (`v*.*.*`):
- Builds for Windows x86_64
- Creates GitHub releases with archived binaries and SHA256 checksums