# Changelog

This page summarizes the public release history of `pysysinfo`.

The canonical changelog in the repository root remains available at
[`CHANGELOG.md`](https://github.com/mrcsparker/pysysinfo/blob/main/CHANGELOG.md).

## Unreleased

- Contributor-focused project polish:
  type-checking, coverage, package smoke tests, dependency audits, pre-commit
  hooks, and a documentation site.

## 0.2.0

- Modernized the project around `uv` + `maturin`.
- Reworked the bindings for modern `sysinfo` and `PyO3`.
- Added a Python-first `Sysinfo` API with immutable snapshot objects.
- Expanded process, metadata, and serialization coverage across the package.
