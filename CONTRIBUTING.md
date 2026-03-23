# Contributing

Thanks for contributing to `pysysinfo`.

The project aims to feel polished from both the Python and Rust sides, so the
bar is not just "works" but also "clear, documented, and pleasant to maintain."

## Development Setup

```bash
uv sync --group dev
pre-commit install --hook-type pre-commit --hook-type pre-push
uv run maturin develop
```

`uv` manages the Python environment and lockfile.
`maturin develop` builds the native extension and installs it into the active
environment so the Python package can import `pysysinfo._core`.

## Usual Local Checks

Run the fast checks before pushing:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
uv run ruff check .
uv run ruff format --check .
uv run mypy tests examples
uv run cargo test --all-targets
uv run pytest
```

For coverage and docs work:

```bash
uv run pytest --cov=pysysinfo --cov-report=term-missing
uv run mkdocs serve
```

## Project Structure

- `src/`: Rust bindings and conversion layers
- `python/pysysinfo/`: public Python package, import surface, and stub files
- `tests/`: Python integration and public API tests
- `examples/`: runnable examples that double as smoke coverage for the public API
- `docs/`: API docs and MkDocs site content

## Design Expectations

- Keep the Python API Pythonic, even when the underlying `sysinfo` API is more
  Rust-shaped.
- Prefer immutable snapshot objects and explicit refresh points.
- Document public behavior in both docstrings and the user-facing docs when it
  affects how consumers use the library.
- Add or update tests for every behavior change. If a new binding is exposed,
  it should have at least one Python test and, when practical, a Rust-side test.
- Preserve deterministic ordering for exported collections where the bindings
  already promise it.

## Documentation Expectations

- Keep [README.md](/Users/mrcsparker/Documents/GitHub/pysysinfo/README.md) focused on orientation and quick start.
- Keep [docs/api.md](/Users/mrcsparker/Documents/GitHub/pysysinfo/docs/api.md) as the complete public API reference.
- Keep examples runnable and representative. They are part of the user
  experience, not throwaway snippets.

## Pull Requests

- Explain the user-facing behavior change.
- Call out compatibility breaks explicitly.
- Mention any platform-specific limitations or assumptions.
- Update the changelog when the change is notable for users or contributors.
