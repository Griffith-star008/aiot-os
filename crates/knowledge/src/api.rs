//! Public API Interfaces


#![deny(unsafe_code)]

/// Knowledge Graph Layer
/// Cho phép Reasoning -> Inference -> Decision dựa trên ngữ nghĩa.

extern crate alloc;

/// Documentation for ConceptId.
pub struct ConceptId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Documentation for RelationType.
pub enum RelationType {
    IsA,
    Causes,
    Requires,
    Prevents,
}

/// Documentation for Triple.
pub struct Triple {
    /// Documentation for field `subject`.
    pub subject: ConceptId,
    /// Documentation for field `predicate`.
    pub predicate: RelationType,
    /// Documentation for field `object`.
    pub object: ConceptId,
}

/// Documentation for ReasonerEngine.
pub trait ReasonerEngine {
    /// Thêm một quy luật / tri thức mới vào đồ thị
    fn add_fact(&mut self, fact: Triple) -> Result<(), KnowledgeError>;

    /// Đặt câu hỏi truy vấn logic (Ví dụ: "Nhiệt độ cao CAUSES gì?")
    fn query(&self, subject: ConceptId, predicate: RelationType) -> Option<ConceptId>;

    /// Tự động suy luận ra các Fact mới từ các Fact cũ
    fn infer_new_knowledge(&mut self) -> Result<u32, KnowledgeError>; // Trả về số lượng rules mới học được
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Documentation for KnowledgeError.
pub enum KnowledgeError {
    GraphFull,
    ContradictionDetected,
}

/// Documentation for registry.
pub mod registry;
/// Exported module/item.
pub use registry::{InMemoryRegistry, ModelRegistry, ModelVersion, PromptRegistry, PromptVersion};
