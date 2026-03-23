# Changelog

All notable changes to `pysysinfo` are documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and the project uses [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Added

- Contributor-facing project hygiene, including pre-commit hooks, coverage
  checks, type-checking, package smoke tests, dependency audits, and a MkDocs
  documentation site.

## [0.2.0] - 2026-03-22

### Added

- A modern mixed `uv` + `maturin` packaging layout with a typed Python package
  under `python/pysysinfo`.
- A Python-first `System` API that replaces the legacy `Sysinfo` entry point.
- Immutable snapshot classes for CPU, disk, network, component, user, group,
  process, cgroup, motherboard, product, and load-average data.
- Process lookups and safe live process-control helpers backed by the owning
  system state.
- Rich Python and Rust test coverage, examples, and API documentation.

### Changed

- Upgraded the bindings to modern `sysinfo` and `PyO3` APIs.
- Reworked refresh controls, serialization helpers, and collector ownership to
  match the split collectors introduced by newer `sysinfo` releases.

### Removed

- The old `pysysinfo.Sysinfo` compatibility surface.
