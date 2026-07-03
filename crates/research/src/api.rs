//! Public API Interfaces


#![deny(unsafe_code)]

/// Long-term Research Infrastructure
/// Formal Runtime Verification & Quantum Backend Abstraction.

/// Formal Verification: Dùng toán học để chứng minh code không bao giờ Crash
pub trait FormalVerifier {
    fn verify_state_transitions(&self, current: u32, next: u32) -> bool;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Documentation for QubitState.
pub enum QubitState {
    Superposition,
    Entangled,
    Collapsed(bool),
}

/// Chuẩn bị kiến trúc trừu tượng cho nền tảng xử lý lượng tử (QPU - Quantum Processing Unit)
pub trait QuantumBackend {
    fn initialize_qubits(&mut self, count: u32) -> Result<(), &'static str>;
    fn apply_hadamard_gate(&mut self, target_qubit: u32);
    fn measure(&mut self, target_qubit: u32) -> QubitState;
}

/// Documentation for QuantumAcceleratorStub.
pub struct QuantumAcceleratorStub;

impl QuantumBackend for QuantumAcceleratorStub {
    fn initialize_qubits(&mut self, _count: u32) -> Result<(), &'static str> {
        Ok(())
    }
    fn apply_hadamard_gate(&mut self, _target_qubit: u32) {}
    fn measure(&mut self, _target_qubit: u32) -> QubitState {
        QubitState::Collapsed(true)
    }
}
