/*
    bfs
    This problem requires you to implement a basic BFS algorithm
*/

use std::collections::{VecDeque, HashSet};

// Define a graph
struct Graph {
    adj: Vec<Vec<usize>>, 
}

impl Graph {
    // Create a new graph with n vertices
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    // Add an edge to the graph
    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest); 
        self.adj[dest].push(src); 
    }

    // Perform a breadth-first search on the graph, return the order of visited nodes
    fn bfs_with_return(&self, start: usize) -> Vec<usize> {
        // 用于存储访问顺序的结果
        let mut visit_order = Vec::new();
        
        // 如果图是空的，直接返回空结果
        if self.adj.is_empty() {
            return visit_order;
        }
        
        // 检查起始节点是否有效
        if start >= self.adj.len() {
            return visit_order;
        }
        
        // 使用队列进行BFS
        let mut queue = VecDeque::new();
        
        // 使用集合记录已访问的节点，避免重复访问
        let mut visited = HashSet::new();
        
        // 从起始节点开始
        queue.push_back(start);
        visited.insert(start);
        
        while !queue.is_empty() {
            // 从队列前端取出节点
            let current_node = queue.pop_front().unwrap();
            
            // 将当前节点添加到访问顺序中
            visit_order.push(current_node);
            
            // 遍历当前节点的所有邻居
            for &neighbor in &self.adj[current_node] {
                // 如果邻居节点还没有被访问过
                if !visited.contains(&neighbor) {
                    // 标记为已访问
                    visited.insert(neighbor);
                    // 将邻居节点加入队列末尾
                    queue.push_back(neighbor);
                }
            }
        }
        
        visit_order
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_all_nodes_visited() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 4);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(1, 4);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 4, 2, 3]);
    }

    #[test]
    fn test_bfs_different_start() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visited_order = graph.bfs_with_return(2);
        assert_eq!(visited_order, vec![2, 1, 0]);
    }

    #[test]
    fn test_bfs_with_cycle() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_bfs_single_node() {
        let mut graph = Graph::new(1);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0]);
    }
}
