<div align="center">
  <h1>AIoT Global Framework</h1>
  <p>A modular, high-performance, edge-to-cloud AIoT framework written in Rust.</p>
  
  [![Status](https://img.shields.io/badge/Status-Beta-yellow)](#)
  [![License](https://img.shields.io/badge/License-MIT-green)](#)
  [![Rust](https://img.shields.io/badge/Rust-1.80%2B-orange)](#)
</div>

---

## 📖 Introduction

The **AIoT Global Framework** is an enterprise-grade platform designed for building distributed, scalable, and resilient AIoT (Artificial Intelligence of Things) applications. Built purely in Rust, it emphasizes a **Trait-First** architecture, zero-cost abstractions, and memory safety.

Whether you are deploying lightweight agents on an Edge device or coordinating distributed AI workloads across a Cloud-Native Kubernetes cluster, the AIoT Framework provides the robust scaffolding you need.

## ✨ Features

- 🏗️ **Dependency Injection Core**: True `DiContainer` allowing loosely coupled architectures via Type-Safe trait injection.
- 🧩 **Robust Plugin Ecosystem**: Develop, load, and hot-swap plugins using our standardized Plugin Manifests (`plugin.toml`) and lifecycles.
- ⚡ **High Performance Runtime**: Predictable execution with minimal overhead (< 2% CPU, < 10MB Base Memory).
- 🧠 **AI First**: Built-in support for orchestrating Cognitive planners, LLMs, and Vision models.
- 🛡️ **Resilience**: Integrated Chaos Engineering, Circuit Breakers, and exponential backoff Retry frameworks.
- 📊 **Telemetry & SRE**: First-class support for OpenTelemetry, Metrics, Distributed Tracing, and strict SLA Error Budgets.
- ☁️ **Cloud Native**: Helm charts and Kubernetes manifests provided out-of-the-box for production scale.

## 🏗️ Architecture

The framework operates on a strict multi-layered architecture to enforce clean dependency flow:

```
Application (CLI, Examples, Services)
         ↓
SDK (Python, Go, Rust Client - WIP)
         ↓
Runtime (DI Container, Lifecycle)
         ↓
Subsystems (Scheduler, Storage, Telemetry, Cluster)
         ↓
Core (AiotError, Traits)
```

See [ARCHITECTURE.md](ARCHITECTURE.md) and our [Docs Portal](docs/) for deep dives.

## 🚀 Quick Start

### Prerequisites
- Rust `1.80+`
- (Optional) `uv` for Python SDK / tooling.

### Installation & Scaffolding
Get started immediately using our CLI tool to generate a new AIoT Plugin:

```bash
# Build the Developer CLI
cargo build -p cli --release

# Scaffold a new AIoT Plugin
./target/release/cli new plugin my_first_plugin
```

### Running Examples
We provide examples to demonstrate core capabilities.
```bash
# Run the Hello World DI Container example
cargo run -p examples --bin hello_world
```

## 🗺️ Roadmap

We have a clear path to becoming a mature global framework.
Check out our [ROADMAP.md](ROADMAP.md) for detailed milestones (v0.8 through v2.0).

## 🤝 Contributing

We welcome contributions from the community!
- Read our [CONTRIBUTING.md](CONTRIBUTING.md).
- Check our [RFCs and ADRs](docs/) before proposing major architectural changes.

## 📝 License

Distributed under the MIT License.
