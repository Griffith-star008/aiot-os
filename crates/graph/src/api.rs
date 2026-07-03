use std::string::String;
use std::vec::Vec;

#[derive(Clone, Debug, PartialEq, Eq)]
/// Documentation for GraphNode.
pub struct GraphNode {
    /// Documentation for field `component_id`.
    pub component_id: u32,
    /// Documentation for field `dependencies`.
    pub dependencies: Vec<u32>,
}

impl GraphNode {
    /// Documentation for new.
    pub fn new(component_id: u32, dependencies: Vec<u32>) -> Self {
        Self {
            component_id,
            dependencies,
        }
    }
}

/// Documentation for DependencyGraph.
pub trait DependencyGraph {
    fn add_node(&mut self, node: GraphNode);
    /// Xác thực đồ thị DAG (Directed Acyclic Graph)
    /// Trả về true nếu không có circular dependency, false nếu phát hiện chu trình.
    fn validate_dag(&self) -> bool;
    /// Tạo mô tả đồ thị bằng định dạng DOT (Graphviz)
    fn generate_visualization(&self) -> String;
}
