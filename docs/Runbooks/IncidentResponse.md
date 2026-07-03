# Incident Response Plan

## 1. Triage
- **SEV-1**: Critical system down, data loss, massive customer impact. (Response: Immediate, all hands)
- **SEV-2**: Core feature degraded, no data loss. (Response: < 15 mins)
- **SEV-3**: Minor bug, isolated impact. (Response: Next business day)

## 2. Roles
- **Incident Commander (IC)**: Leads the response, makes decisions.
- **Communications Lead**: Updates stakeholders and customers.
- **Operations Lead**: Executes technical fixes and diagnostics.

## 3. Communication
- Open a dedicated Slack/Discord channel `#incident-<date>`.
- Update status page every 30 minutes.

## 4. Post-Mortem
- Within 48 hours of resolution, write a blameless post-mortem detailing timeline, root cause (5 Whys), and action items.
