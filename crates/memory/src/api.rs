//! Public API Interfaces


#![deny(unsafe_code)]

/// Tầng lưu trữ ký ức (Memory Runtime) - Không chỉ Telemetry mà còn lưu tri thức cho AI

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Documentation for MemoryType.
pub enum MemoryType {
    Working,  // Bộ nhớ ngắn hạn (tác vụ hiện tại)
    Episodic, // Bộ nhớ sự kiện (những gì đã xảy ra, vd: Lỗi quá nhiệt hôm qua)
    Semantic, // Cấu trúc tri thức (Knowledge Graph về hệ thống)
}

/// Documentation for VectorStoreStub.
pub struct VectorStoreStub {
    /// Documentation for field `dimension`.
    pub dimension: usize,
    // Thực tế sẽ lưu các embeddings
}

impl VectorStoreStub {
    /// Documentation for new.
    pub fn new(dimension: usize) -> Self {
        Self { dimension }
    }

    /// Documentation for query_similar.
    pub fn query_similar(&self, _embedding: &[f32]) -> Option<&'static str> {
        // Trả về kinh nghiệm tương tự trong quá khứ để Reasoner tham khảo
        None
    }
}

/// Documentation for RuntimeMemory.
pub struct RuntimeMemory {
    /// Documentation for field `working_memory`.
    pub working_memory: [u8; 1024], // Giả lập dung lượng hạn chế
    /// Documentation for field `vector_store`.
    pub vector_store: VectorStoreStub,
}

impl RuntimeMemory {
    /// Documentation for new.
    pub fn new() -> Self {
        Self {
            working_memory: [0; 1024],
            vector_store: VectorStoreStub::new(128),
        }
    }
}

impl Default for RuntimeMemory {
    fn default() -> Self {
        Self::new()
    }
}
