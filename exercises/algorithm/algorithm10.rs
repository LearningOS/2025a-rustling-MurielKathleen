/*
    graph
    This problem requires you to implement a basic graph functio
*/

use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone)]
pub struct NodeNotInGraph;

impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}

pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}

impl Graph for UndirectedGraph {
    fn new() -> UndirectedGraph {
        UndirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }
    
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }
    
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }
    
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        let (from, to, weight) = edge;
        
        // 确保两个节点都存在
        self.add_node(from);
        self.add_node(to);
        
        // 获取邻接表的可变引用
        let adj_table = self.adjacency_table_mutable();
        
        // 添加 from -> to 的边
        if let Some(neighbors) = adj_table.get_mut(from) {
            if !neighbors.iter().any(|(n, _)| n == to) {
                neighbors.push((to.to_string(), weight));
            }
        }
        
        // 添加 to -> from 的边（无向图）
        if let Some(neighbors) = adj_table.get_mut(to) {
            if !neighbors.iter().any(|(n, _)| n == from) {
                neighbors.push((from.to_string(), weight));
            }
        }
    }
}

pub trait Graph {
    fn new() -> Self;
    
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;
    
    fn add_node(&mut self, node: &str) -> bool {
        let adj_table = self.adjacency_table_mutable();
        if !adj_table.contains_key(node) {
            adj_table.insert(node.to_string(), Vec::new());
            true
        } else {
            false
        }
    }
    
    fn add_edge(&mut self, edge: (&str, &str, i32));
    
    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().get(node).is_some()
    }
    
    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }
    
    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();
        let mut added_edges = HashSet::new();
        
        for (from_node, from_node_neighbours) in self.adjacency_table() {
            for (to_node, weight) in from_node_neighbours {
                // 为了避免重复添加同一条边（无向图中每条边会存储两次）
                // 我们使用一个有序对来确保只添加一次
                let edge_key = if from_node < to_node {
                    (from_node.as_str(), to_node.as_str())
                } else {
                    (to_node.as_str(), from_node.as_str())
                };
                
                if !added_edges.contains(&edge_key) {
                    edges.push((from_node, to_node, *weight));
                    added_edges.insert(edge_key);
                }
            }
        }
        edges
    }
}

#[cfg(test)]
mod test_undirected_graph {
    use super::Graph;
    use super::UndirectedGraph;
    
    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));
        
        let expected_edges = [
            (&String::from("a"), &String::from("b"), 5),
            (&String::from("b"), &String::from("a"), 5),
            (&String::from("c"), &String::from("a"), 7),
            (&String::from("a"), &String::from("c"), 7),
            (&String::from("b"), &String::from("c"), 10),
            (&String::from("c"), &String::from("b"), 10),
        ];
        
        for edge in expected_edges.iter() {
            assert_eq!(graph.edges().contains(edge), true);
        }
    }
}
