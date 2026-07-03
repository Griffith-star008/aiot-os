# AIOT

<div align="center">

**A Modular AI Operating Platform for Edge, Cloud, Robotics and AIoT**

High-performance • Plugin-based • Distributed • AI Native • Built with Rust

![Rust](https://img.shields.io/badge/Rust-2021-orange)
![License](https://img.shields.io/badge/License-Apache%202.0-blue)
![Platform](https://img.shields.io/badge/Platform-Cross--Platform-green)

</div>

---

# Overview

AIOT is an open-source AI Operating Platform designed to simplify building intelligent applications across Edge, Cloud, Robotics and IoT environments.

Instead of providing only an AI runtime, AIOT offers a complete ecosystem including:

- AI Runtime
- Workflow Engine
- Plugin Framework
- Distributed Runtime
- Edge Runtime
- AI Studio
- RAG Framework
- Enterprise Platform
- Device Framework
- Cloud Integration

---

# Features

- Modular architecture
- AI-first runtime
- Plugin ecosystem
- Workflow orchestration
- Multi-model routing
- RAG framework
- Edge AI
- Distributed execution
- WASM support
- Enterprise-ready
- Cross-platform
- Built with Rust

---

# Architecture

```
Applications
        │
CLI / SDK / Dashboard
        │
Workflow / Plugins
        │
AI Runtime
        │
Distributed Runtime
        │
Core Runtime
```

---

# Project Structure

```
AIOT/

├── crates/
├── docs/
├── examples/
├── scripts/
├── tools/
├── Cargo.toml
└── README.md
```

---

# Requirements

- Rust stable
- Cargo
- Git

Check version

```bash
rustc --version
cargo --version
```

---

# Installation

Clone repository

```bash
git clone https://github.com/Griffith-star008/aiot-os.git
```

Go to project

```bash
cd aiot-os
```

Build

```bash
cargo build
```

Release build

```bash
cargo build --release
```

---

# Quick Start

Run all tests

```bash
cargo test
```

Format source

```bash
cargo fmt
```

Lint

```bash
cargo clippy
```

Build documentation

```bash
cargo doc --open
```

---

# Examples

Run example

```bash
cargo run --example hello_world
```

or

```bash
cargo run --example workflow
```

Example source is located in:

```
examples/
```

---

# Using AIOT

Typical workflow

```
Create Project

↓

Create Workflow

↓

Register Plugins

↓

Load AI Models

↓

Deploy Runtime

↓

Monitor

↓

Scale
```

---

# Documentation

| Document | Description |
|-----------|-------------|
| Architecture | Overall architecture |
| Cookbook | Practical examples |
| API | API reference |
| Plugin Guide | Build plugins |
| Workflow Guide | Create workflows |
| SDK Guide | SDK usage |
| Security | Security model |
| Performance | Optimization guide |

---

# Roadmap

- Alpha
- Beta
- Stable
- Enterprise
- Cloud Platform

---

# Contributing

We welcome contributions from the community.

Please read:

- CONTRIBUTING.md
- CODE_OF_CONDUCT.md

---

# Development

Run formatting

```bash
cargo fmt
```

Run linter

```bash
cargo clippy
```

Run tests

```bash
cargo test
```

Run benchmarks

```bash
cargo bench
```

---

# License

Licensed under Apache License 2.0.

See LICENSE for details.

---

# Community

- GitHub Issues
- GitHub Discussions
- Pull Requests

Contributions are always welcome.

---

<div align="center">

Built with ❤️ using Rust.

</div>
