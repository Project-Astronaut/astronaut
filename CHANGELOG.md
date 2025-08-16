# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
- Initial workspace scaffolding refinements
- Repo hygiene: LICENSE, CONTRIBUTING, CODE_OF_CONDUCT, Dockerfile
- Crate directory renames (no prefix)
- CI improvements (build/test/lint)

## [0.1.0] - 2025-08-16
### Added
- Cargo workspace with core crates
- Basic Axum server with insert/search endpoints (linear scan)
- In-memory storage with snapshot support
- API types, distance utilities, placeholder index/quant/storage crates
- README and initial documentation
