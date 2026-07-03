# Node Failure Runbook

## Symptoms
- Node status marked as `Dead` in telemetry.
- Reduced cluster throughput.
- Error rates spiking for tasks assigned to the specific node.

## Actions
1. **Verify**: Check Grafana dashboard for node CPU/RAM metrics to ensure it is actually dead and not a network partition.
2. **Quarantine**: If the node is unreachable, the cluster will automatically quarantine it. Verify via cluster API.
3. **Restart**: Attempt a hard restart of the node via orchestration (e.g., Kubernetes `kubectl delete pod <node-id>`).
4. **Data Consistency**: The persistent queue will re-assign incomplete tasks to healthy nodes.

## Escalation
If >30% of nodes fail simultaneously, proceed to `ClusterRecovery.md`.
