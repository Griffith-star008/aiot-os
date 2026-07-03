# AIoT Framework Long-term Roadmap

This roadmap outlines the strategic milestones and features planned for the AIoT Global Framework.

## 🎯 v0.8: Core Foundations
* **Stable Runtime**: Dependency Injection Container, standardized Error Taxonomy, and fluent Builder API.
* **Stable Scheduler**: Autonomous scheduling engine with Priority, Cron, and Round Robin support.
* **Storage Layer**: Trait-first storage abstractions (Memory, SQLite, KeyValue).
* **Plugin API**: Standardized Plugin Registry and lifecycle management.

## 🚀 v1.0: Production & Ecosystem (Current Target)
* **Stable Public API**: Strict adherence to Builder Pattern and Trait-First design across all 45+ crates.
* **SDK Rollout**: Initial multi-language SDKs (starting with Python).
* **Comprehensive Documentation**: Complete architecture, tutorials, cookbooks, and `mdBook` website.
* **Examples Gallery**: Standardized examples (`hello_world`, `sensor_mqtt`, `vision`).
* **Cloud Native SRE**: Helm Charts, Kubernetes manifests, Telemetry (OpenTelemetry), and Error Budgets (SLO).

## 🧠 v1.5: Intelligent & Edge Operations
* **AI Runtime**: Flagship feature supporting ONNX, TensorRT, Ollama, and OpenAI API natively.
* **Cluster Runtime**: Robust leader election, Raft consensus, and node discovery.
* **Edge Runtime**: Highly optimized runtimes tailored for ESP32, Jetson, and Raspberry Pi.
* **Marketplace Beta**: Centralized registry for third-party plugins and templates.

## 🌍 v2.0: Global Distributed Enterprise
* **Distributed AI**: Federated learning and distributed inference across edge nodes.
* **Multi-Cluster Orchestration**: Managing hundreds of edge clusters via a unified control plane.
* **AI Workflow Designer**: Visual Node-based GUI for building autonomous agent workflows.
* **Enterprise Support**: Advanced RBAC, mTLS, Secret Management, and commercial SLA backing.
