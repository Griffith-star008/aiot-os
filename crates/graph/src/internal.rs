use crate::api::{DependencyGraph, GraphNode};
use std::collections::{BTreeMap, VecDeque};
use std::string::String;
use std::vec::Vec;
use core::fmt::Write;

/// Documentation for StandardDependencyGraph.
pub struct StandardDependencyGraph {
    /// Documentation for field `nodes`.
    pub nodes: BTreeMap<u32, GraphNode>,
}

impl StandardDependencyGraph {
    /// Documentation for fn.
    pub const fn new() -> Self {
        Self {
            nodes: BTreeMap::new(),
        }
    }
}

impl Default for StandardDependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl DependencyGraph for StandardDependencyGraph {
    fn add_node(&mut self, node: GraphNode) {
        self.nodes.insert(node.component_id, node);
    }

    fn validate_dag(&self) -> bool {
        let mut in_degree: BTreeMap<u32, usize> = BTreeMap::new();
        let mut graph: BTreeMap<u32, Vec<u32>> = BTreeMap::new();

        // Khởi tạo đồ thị
        for &id in self.nodes.keys() {
            in_degree.insert(id, 0);
            graph.insert(id, Vec::new());
        }

        // Tạo ma trận cạnh và đếm bán bậc vào
        for node in self.nodes.values() {
            in_degree.insert(node.component_id, node.dependencies.len());
            for &dep in &node.dependencies {
                graph.entry(dep).or_default().push(node.component_id);
            }
        }

        // Thuật toán Kahn (Topological Sort)
        let mut queue = VecDeque::new();
        for (&id, &deg) in &in_degree {
            if deg == 0 {
                queue.push_back(id);
            }
        }

        let mut visited_count = 0;
        while let Some(id) = queue.pop_front() {
            visited_count += 1;

            if let Some(neighbors) = graph.get(&id) {
                for &neighbor in neighbors {
                    if let Some(deg) = in_degree.get_mut(&neighbor) {
                        *deg -= 1;
                        if *deg == 0 {
                            queue.push_back(neighbor);
                        }
                    }
                }
            }
        }

        // Nếu số node đã duyệt bằng tổng số node thì không có chu trình
        visited_count == self.nodes.len()
    }

    fn generate_visualization(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(&mut out, "digraph DependencyGraph {{");
        for node in self.nodes.values() {
            let _ = writeln!(
                &mut out,
                "    N{} [label=\"Component {}\"];",
                node.component_id, node.component_id
            );
            for &dep in &node.dependencies {
                let _ = writeln!(&mut out, "    N{} -> N{};", dep, node.component_id);
            }
        }
        let _ = writeln!(&mut out, "}}");
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn test_valid_dag() {
        let mut graph = StandardDependencyGraph::new();
        graph.add_node(GraphNode::new(1, vec![]));
        graph.add_node(GraphNode::new(2, vec![1]));
        graph.add_node(GraphNode::new(3, vec![2]));

        assert!(graph.validate_dag());
    }

    #[test]
    fn test_invalid_dag_circular() {
        let mut graph = StandardDependencyGraph::new();
        graph.add_node(GraphNode::new(1, vec![3])); // 1 depends on 3
        graph.add_node(GraphNode::new(2, vec![1])); // 2 depends on 1
        graph.add_node(GraphNode::new(3, vec![2])); // 3 depends on 2

        assert!(!graph.validate_dag());
    }
}
