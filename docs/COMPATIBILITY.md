# Compatibility Matrix

The AIoT Framework is designed to be truly edge-to-cloud. Below is the official support matrix for v1.0+.

## Operating Systems
| OS | Architecture | Status | Tier |
|----|--------------|--------|------|
| Linux (Ubuntu/Debian) | x86_64, aarch64 | ✅ Tested | Tier 1 |
| macOS | Apple Silicon (M1/M2/M3) | ✅ Tested | Tier 1 |
| Windows | x86_64 | ⚠️ Best Effort | Tier 2 |

## Embedded / Edge Devices
| Device | Status | Tier | Notes |
|--------|--------|------|-------|
| Raspberry Pi 4/5 | ✅ Tested | Tier 1 | Full runtime support |
| NVIDIA Jetson (Orin/Nano) | ✅ Tested | Tier 1 | With CUDA bindings |
| STM32 / ESP32 | 🚧 WIP | Tier 3 | Needs `no_std` runtime profile |

## Cloud Native Environments
| Environment | Status | Notes |
|-------------|--------|-------|
| Docker | ✅ Supported | Official images provided |
| Kubernetes | ✅ Supported | Helm charts provided (`helm/aiot`) |
| K3s / MicroK8s | ✅ Supported | Ideal for Edge Clusters |

## Programming Languages (SDKs)
| Language | Version | Status |
|----------|---------|--------|
| Rust | 1.80+ | ✅ Core |
| Python | 3.10+ | 🚧 Alpha |
| Go | 1.21+ | 🚧 Alpha |
| Node.js | 20+ | 🚧 Planned|
