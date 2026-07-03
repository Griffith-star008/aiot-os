# Deploying to Kubernetes (Manifests)

If you prefer applying raw Kubernetes manifests over using Helm, you can use the predefined YAML files in the `kubernetes/` directory.

## Prerequisites
- Kubernetes cluster >= 1.25
- kubectl configured

## Quick Start

Apply all manifests in the directory:
```bash
kubectl apply -f kubernetes/
```

This will create:
1. `deployment.yaml`: The core application pods.
2. `service.yaml`: ClusterIP service for internal routing.
3. `configmap.yaml`: Core configuration.
4. `secret.yaml`: Secrets (ensure you encrypt this in production using tools like SealedSecrets).
5. `hpa.yaml`: Horizontal Pod Autoscaling rules.
6. `network-policy.yaml`: Zero-trust network constraints.

## Best Practices
- **Do not commit raw secrets**: The `secret.yaml` provided is for demonstration. Use ExternalSecrets or SOPS for production.
- **Resource Quotas**: Ensure your namespace has adequate CPU/Memory quotas for the AIoT components, especially the GPU nodes if scheduling ML workloads.
- **Ingress**: You will need to bring your own Ingress Controller (e.g. NGINX, Traefik) and configure the routing.
