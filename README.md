# Astronaut: In-memory Vector DB (Rust)

Astronaut is a modular, production-oriented in-memory vector database written in Rust.

Status: the project is in an early development phase and under active work.

We welcome contributors! If you want to help build a high-performance vector DB from the ground up, please see the Contributing section.

## Project Status
- __Early Stage__: APIs and modules are evolving.
- __Placeholders__: Indexing (HNSW/IVF), quantization, and storage backends are currently scaffolds.
- __Goals__: High-throughput inserts, ANN search, observability, and optional persistence.

## Workspace Layout
- `crates/astronaut-core`: Core types, traits, in-memory storage
- `crates/astronaut-api-types`: API request/response models
- `crates/astronaut-server`: Axum-based HTTP server
- `crates/astronaut-cli`: CLI wrapper
- `crates/astronaut-distance`: Distance functions (SIMD-ready)
- `crates/astronaut-index-hnsw`: HNSW (placeholder)
- `crates/astronaut-index-ivf`: IVF (placeholder)
- `crates/astronaut-quant`: Quantization (placeholder)
- `crates/astronaut-storage`: Storage backends (placeholder)

## Quickstart
- __Run server__
```bash
cargo run -p astronaut-server
```
- __Insert__
```bash
curl -X POST http://localhost:8080/v1/insert \
  -H 'Content-Type: application/json' \
  -d '{"id":"a","vector":[0,1,0],"payload":{}}'
```
- __Search__
```bash
curl -X POST http://localhost:8080/v1/search \
  -H 'Content-Type: application/json' \
  -d '{"vector":[0,1,0],"top_k":10}'
```

## Roadmap (High-level)
- __Indexing__: HNSW, IVF, IVF-PQ
- __Compute__: SIMD-optimized distances
- __Storage__: Snapshots, mmap, optional WAL
- __Scale__: Sharding, background compaction
- __Ops__: Tracing/metrics (OpenTelemetry), CI/CD

## Contributing
- __Issues/Discussions__: Open issues with context and repro if possible.
- __PRs__: Small, focused PRs are easier to review. Include tests where possible.
- __Style__: `rustfmt` and `clippy` clean.
- __Good first tasks__: Docs improvements, tests, simple distance kernels, server endpoints.

## Development
```bash
# Build all
cargo build --workspace

# Test all
cargo test --workspace
```

## License
Apache-2.0
