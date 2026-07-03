# Service Level Objectives (SLO) & SLAs

## Service Level Agreements (SLA)
- **Availability**: 99.99% uptime per month.
- **Downtime Allowance**: ~4.32 minutes per month.

## Service Level Objectives (SLO)
- **API Latency**: 95th percentile (P95) < 200ms.
- **Agent Scheduling Latency**: P99 < 500ms.
- **Task Success Rate**: > 99.9%.

## Error Budgets
- **Availability Error Budget**: 0.01% (4.32 minutes/month).
- **Latency Error Budget**: 5% of requests allowed > 200ms.
- **Action**: If Error Budget is depleted, freeze all non-critical feature deployments and prioritize reliability fixes.

## Key Metrics
- **MTTR (Mean Time To Recovery)**: Target < 15 minutes.
- **MTBF (Mean Time Between Failures)**: Target > 30 days.
