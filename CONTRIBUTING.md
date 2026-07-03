# Contributing to AIoT Framework

First off, thank you for considering contributing to the AIoT Framework! 

## Code of Conduct

By participating in this project, you agree to abide by our Code of Conduct (TBD).

## Development Workflow

1. **Fork the repository** and create your branch from `main`.
2. **Ensure Code Quality**: We use `clippy` for linting. Ensure your code passes all pedantic and workspace lints.
   ```bash
   cargo clippy --workspace --all-targets -- -D warnings
   ```
3. **Write Tests**: All new features must be covered by Unit and/or Integration tests.
   ```bash
   cargo test --workspace
   ```
4. **Documentation**: Document all public traits, structs, and functions. We enforce `missing_docs` at the workspace level.

## Pull Request Process

1. Provide a clear description of the problem and the proposed solution.
2. Link any related issues or RFCs.
3. Ensure CI checks pass successfully before requesting a review.
4. Two approvals from core maintainers are required for a merge.

## Architectural Decision Records (ADRs)

For significant architectural changes, please submit an ADR in the `docs/adr/` directory before writing code. This ensures alignment with the project's long-term vision.
