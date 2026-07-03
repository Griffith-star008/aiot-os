# Cluster Recovery Runbook

## Symptoms
- Total loss of consensus (Raft).
- Complete unresponsiveness of the API.
- All nodes failing health checks.

## Actions
1. **Identify Root Cause**: Check if it's a network partition across regions or a cascading software panic.
2. **Isolate**: Block all incoming gateway traffic to prevent queue flooding.
3. **Bootstrap**: Select a single node to form a new cluster seed.
4. **Rejoin**: Incrementally start other nodes and have them join the new seed node.
5. **Sync**: Allow nodes to sync states via the Gossip protocol.
6. **Re-open Traffic**: Gradually allow 10%, 50%, then 100% of traffic back through the gateway.
