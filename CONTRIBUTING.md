# Contributing to Astronaut

Thanks for your interest in contributing! Astronaut is in an early development phase and evolving rapidly. We welcome issues, discussions, and pull requests.

## How to Contribute
- **Report bugs / request features**: Open a GitHub issue with steps to reproduce, expected vs actual behavior, and environment details.
- **Discuss**: If unsure about direction, start a discussion or a draft PR to scope the change.
- **Pull Requests**: Small, focused PRs are easiest to review. Include context in the description and tests when possible.

## Project Setup
```bash
# Build entire workspace
cargo build --workspace

# Run tests
cargo test --workspace

# Lint & format
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
```

## Repo Structure (high-level)
- `crates/astronaut-core`: Core types, traits, and in-memory storage.
- `crates/astronaut-api-types`: HTTP API request/response models.
- `crates/astronaut-server`: Axum-based HTTP server.
- `crates/astronaut-cli`: CLI wrapper and developer tooling.
- `crates/astronaut-distance`: Distance metrics (SIMD-ready).
- `crates/astronaut-index-hnsw`: HNSW index (placeholder).
- `crates/astronaut-index-ivf`: IVF index (placeholder).
- `crates/astronaut-quant`: Quantization (placeholder).
- `crates/astronaut-storage`: Storage backends (placeholder).

## Coding Guidelines
- **Rust Edition**: 2021
- **Style**: Use `rustfmt`. Keep `clippy` warnings at zero.
- **Errors**: Prefer `thiserror` for error types and `anyhow` at boundaries.
- **APIs**: Favor clear, well-documented public APIs. Add doc comments and examples where helpful.
- **Concurrency**: Prefer `Arc` + `RwLock` or channels as appropriate. Keep hot paths allocation-aware.
- **Testing**: Add unit tests for core logic; integration tests for public behavior.

## Commit & Branching
- **Branches**: Use feature branches (`feat/hnsw-levels`, `fix/search-scores`).
- **Commits**: Write clear messages. Example: `core: add InMemoryStorage::snapshot for scans`.
- **PR Titles**: Be descriptive, include scope (`server`, `core`, `index-hnsw`, etc.).

## Performance & Benchmarks
- When introducing performance-sensitive changes, include Criterion benchmarks or measurements and a brief note in the PR.

## Security
- If you discover a vulnerability, please do not open a public issue. Email the maintainer(s) privately or mark the issue clearly as security-related and minimal.

## License
- By contributing, you agree that your contributions are licensed under the Apache-2.0 License (see `LICENSE.md`).

## Code of Conduct
- Be respectful, constructive, and inclusive. No harassment or discrimination of any kind.

Thank you for helping build Astronaut! 🚀
