//! Public API Interfaces


#![deny(unsafe_code)]

/// Autonomous Governance Layer & Policy Engine
/// Đánh giá quyền (Policy) và tạo Audit Log.

extern crate alloc;

use std::vec::Vec;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Documentation for SubjectId.
pub struct SubjectId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Documentation for ResourceId.
pub struct ResourceId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Documentation for Permission.
pub enum Permission {
    ReadSensor,
    ActuateMotor,
    ModifyModel,
    UpdateFirmware,
    RebootSystem,
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// Documentation for AuditRecord.
pub struct AuditRecord {
    /// Documentation for field `timestamp`.
    pub timestamp: u64,
    /// Documentation for field `subject`.
    pub subject: SubjectId,
    /// Documentation for field `perm`.
    pub perm: Permission,
    /// Documentation for field `success`.
    pub success: bool,
}

/// Documentation for AuditEngine.
pub trait AuditEngine {
    fn log_action(&mut self, timestamp: u64, subject: SubjectId, perm: Permission, success: bool);
    fn get_logs(&self) -> Vec<AuditRecord>;
}

/// Policy Engine: Định nghĩa các luật Security, Thermal, Mission.
pub struct PolicyEngine {
    /// Documentation for field `strict_mode`.
    pub strict_mode: bool,
}

impl PolicyEngine {
    /// Documentation for fn.
    pub const fn new() -> Self {
        Self { strict_mode: true }
    }

    /// Documentation for evaluate_access.
    pub fn evaluate_access(
        &self,
        _subject: SubjectId,
        _resource: ResourceId,
        perm: Permission,
    ) -> bool {
        // Trong strict_mode, UpdateFirmware bị cấm trừ khi có cờ đặc biệt
        if self.strict_mode && perm == Permission::UpdateFirmware {
            return false;
        }
        true
    }
}

impl Default for PolicyEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Documentation for StandardAuditEngine.
pub struct StandardAuditEngine<const CAPACITY: usize> {
    buffer: Vec<AuditRecord>,
    head: usize,
    count: usize,
}

impl<const CAPACITY: usize> StandardAuditEngine<CAPACITY> {
    /// Documentation for new.
    pub fn new() -> Self {
        Self {
            buffer: Vec::with_capacity(CAPACITY),
            head: 0,
            count: 0,
        }
    }
}

impl<const CAPACITY: usize> Default for StandardAuditEngine<CAPACITY> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const CAPACITY: usize> AuditEngine for StandardAuditEngine<CAPACITY> {
    fn log_action(&mut self, timestamp: u64, subject: SubjectId, perm: Permission, success: bool) {
        let record = AuditRecord {
            timestamp,
            subject,
            perm,
            success,
        };

        if self.buffer.len() < CAPACITY {
            self.buffer.push(record);
            self.count += 1;
        } else {
            self.buffer[self.head] = record;
        }

        self.head = (self.head + 1) % CAPACITY;
    }

    fn get_logs(&self) -> Vec<AuditRecord> {
        let mut logs = Vec::with_capacity(self.count);
        if self.count < CAPACITY {
            logs.extend_from_slice(&self.buffer[..self.count]);
        } else {
            logs.extend_from_slice(&self.buffer[self.head..]);
            logs.extend_from_slice(&self.buffer[..self.head]);
        }
        logs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_engine_ring_buffer() {
        let mut engine = StandardAuditEngine::<3>::new();
        engine.log_action(1, SubjectId(1), Permission::ReadSensor, true);
        engine.log_action(2, SubjectId(2), Permission::ActuateMotor, true);
        engine.log_action(3, SubjectId(3), Permission::ModifyModel, false);

        let logs = engine.get_logs();
        assert_eq!(logs.len(), 3);
        assert_eq!(logs[0].timestamp, 1);

        // Gây tràn buffer
        engine.log_action(4, SubjectId(4), Permission::UpdateFirmware, false);
        let logs2 = engine.get_logs();
        assert_eq!(logs2.len(), 3);
        assert_eq!(logs2[0].timestamp, 2);
        assert_eq!(logs2[1].timestamp, 3);
        assert_eq!(logs2[2].timestamp, 4);
    }
}
