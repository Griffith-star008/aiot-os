//! Public API Interfaces


#![deny(unsafe_code)]

/// AI Compiler Layer
/// Pipeline: ONNX -> Optimization -> Kernel Fusion -> Backend -> Binary

/// Documentation for OnnxModel.
pub struct OnnxModel;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Documentation for QuantizationLevel.
pub enum QuantizationLevel {
    None,
    Int8,
    Int4,
}

/// Documentation for AiCompiler.
pub trait AiCompiler {
    /// Load model từ định dạng ONNX
    fn load_onnx(&self, data: &[u8]) -> Result<OnnxModel, CompilerError>;

    /// Tối ưu hóa Model (Pruning, Dead code elimination)
    fn optimize_graph(&self, model: &mut OnnxModel) -> Result<(), CompilerError>;

    /// Kernel Fusion: Gộp các phép toán nhỏ (Conv + BatchNorm + ReLU) thành 1 kernel lớn
    fn perform_kernel_fusion(&self, model: &mut OnnxModel) -> Result<(), CompilerError>;

    /// Lượng tử hóa mô hình để chạy nhanh hơn trên biên (Edge)
    fn quantize(
        &self,
        model: &mut OnnxModel,
        level: QuantizationLevel,
    ) -> Result<(), CompilerError>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Documentation for CompilerError.
pub enum CompilerError {
    InvalidModelFormat,
    OptimizationFailed,
    FusionNotSupported,
    QuantizationError,
}
