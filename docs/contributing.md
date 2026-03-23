# Contributing

`pysysinfo` is maintained as both a Python package and a Rust crate, so good
changes here usually touch code, docs, and tests together.

The canonical contributor guide lives at
[`CONTRIBUTING.md`](https://github.com/mrcsparker/pysysinfo/blob/main/CONTRIBUTING.md).

## Setup

```bash
uv sync --group dev
pre-commit install --hook-type pre-commit --hook-type pre-push
uv run maturin develop
```

## Usual Checks

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
uv run ruff check .
uv run ruff format --check .
uv run mypy tests examples
uv run cargo test --all-targets
uv run pytest
uv run mkdocs build --strict
```

## Expectations

- Keep the Python API clean and Pythonic.
- Prefer immutable snapshot objects over mutable live wrappers.
- Update tests and docs when the public behavior changes.
- Treat examples as part of the product, not as disposable snippets.
