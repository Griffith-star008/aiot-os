# AIoT Framework API Policy

This document outlines the strict API Governance and Lifecycle policies for the Global AIoT Framework v1.0 and beyond.

## 1. Public API Definition
- All traits and structs exported from any module located in an `api/` directory.
- All macros exported from a `sdk/` or `crates/*_sdk` crate.
- Command Line Interface (CLI) arguments and subcommands (`aiot *`).
- Configuration file schemas (`plugin.toml`, `aiot_config.yaml`).

## 2. Breaking Changes Policy
A breaking change is any change that causes existing downstream code or plugins to fail compilation or alter their runtime behavior drastically.
- Breaking changes are **STRICTLY PROHIBITED** in MINOR or PATCH versions.
- If a breaking change is necessary, it MUST be deferred to the next MAJOR version release.

## 3. Deprecation Lifecycle
1. **Mark as Deprecated**: The API must be marked with `#[deprecated(since = "...", note = "...")]`.
2. **Grace Period**: The deprecated API must remain functional for at least one full MINOR version release (e.g., deprecated in v1.1, removed no earlier than v2.0).
3. **Migration Guide**: A migration guide (`docs/migration/`) must be provided demonstrating how to transition away from the deprecated API.

## 4. Stability Guarantees
- `Stable`: Ready for production. Backwards compatible.
- `Experimental`: Subject to change. Hidden behind feature flags (`#[cfg(feature = "experimental")]`). Use at your own risk.
- `Internal`: Hidden (`pub(crate)`). No stability guarantees. Do not use.
