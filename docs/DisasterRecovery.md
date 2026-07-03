# Disaster Recovery Plan

This document outlines the Disaster Recovery (DR) procedures for the AIoT Cluster.

## 1. Backup Strategy
- **Frequency**: Every 6 hours (Incremental), Daily (Full).
- **Storage**: Off-site Object Storage (e.g., S3).
- **Coverage**: Persistent Queues, Database WALs, Configuration States.

## 2. Snapshot
- File system and EBS volume snapshots taken automatically every 24 hours.

## 3. Restore Procedure
1. Halt all incoming traffic via EdgeRouter.
2. Download the latest verified snapshot from cold storage.
3. Apply WAL records to catch up to the latest known state.

## 4. Verification
- Run internal cluster health checks (`/health`, `/metrics`).
- Verify data consistency via `StorageManager::verify_checksums()`.

## 5. Failover
- In a Multi-Region setup, shift DNS to Region B if Region A is unrecoverable within 30 minutes.
- Ensure Leader Election re-balances the workload.
