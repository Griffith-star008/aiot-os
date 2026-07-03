# Database Failure Runbook

## Symptoms
- Extreme latency spikes on read/write operations.
- Errors containing `StorageError` or `ConnectionRefused`.
- Circuit breakers triggered for storage layer.

## Actions
1. **Fallback**: Ensure the system has fallen back to the InMemory WAL for temporary buffering.
2. **Investigate**: Check DB logs for OOM, Disk Full, or connection limits.
3. **Expand Capacity**: If Disk Full, provision additional EBS volume space.
4. **Restart**: Restart the DB replica. If Primary is down, trigger a forced failover to the replica.

## Escalation
If data corruption is detected, proceed to `DisasterRecovery.md` for snapshot restoration.
