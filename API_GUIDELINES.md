# AIoT Framework API Guidelines

These guidelines define the conventions for building Crates, Services, and Plugins in the AIoT Global Framework. All contributors must adhere to these standards before v1.0.

## 1. Directory & Layer Organization
- `api/`: Stable traits and public structures. Only breaking changes in major SemVer releases.
- `internal/`: Implementation details. Must not be exposed to external users.
- `experimental/`: New features flagged with `#[cfg(feature = "experimental")]`. Not subject to SemVer guarantees.
- `deprecated/`: Features slated for removal in the next major release.

## 2. The Builder Pattern
Do NOT expose raw constructors (`new()`, `new2()`, `new_default()`) with many parameters. Use the fluent Builder Pattern.
```rust
// BAD
pub fn create_runtime(config: Config, state: State, workers: u32) -> Runtime { ... }

// GOOD
let runtime = RuntimeBuilder::new()
    .with_config(config)
    .with_state(state)
    .with_workers(4)
    .build();
```

## 3. Error Handling
- Never use `unwrap()` or `panic!()` in library code.
- Always use `thiserror` to define module-level errors.
- Bubble up all errors to the root `AiotError` enum in the `core` crate.

## 4. Trait-First Design
Every major subsystem must define a Trait in the `api` module. Implementations are injected via the `DiContainer`.
- e.g., `pub trait StorageEngine { ... }`

## 5. Asynchronous Design
- Ensure traits use `#[async_trait]` if they involve I/O.
- Use `tokio` primitives but abstract them behind the Framework's own traits where possible to allow RTOS/No-std alternatives in the future.
