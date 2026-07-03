# Deploying with Helm

Helm is the recommended way to deploy the AIoT Global Framework to production Kubernetes clusters.

## Prerequisites
- Kubernetes cluster >= 1.25
- Helm v3+
- kubectl configured

## Installation

### 1. From Source
If you have the repository cloned:
```bash
cd helm/
helm install aiot-release ./aiot --namespace aiot-system --create-namespace
```

### 2. Configuration (`values.yaml`)
You can override default settings via a custom `values.yaml` or using `--set`:
```bash
helm install aiot-release ./aiot \
  --set replicaCount=5 \
  --set autoscaling.enabled=true \
  --set telemetry.enabled=true
```

## Key Components Deployed
- **Deployment**: Core AIoT Runtime pods.
- **HPA**: Horizontal Pod Autoscaler for dynamic load management.
- **Service & Ingress**: For external traffic routing.
- **ConfigMap**: For injecting telemetry and runtime configurations.

## Uninstallation
To remove the deployment:
```bash
helm uninstall aiot-release -n aiot-system
```
