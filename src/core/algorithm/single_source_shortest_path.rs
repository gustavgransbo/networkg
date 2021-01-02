//!

use std::collections::{HashMap, VecDeque};

use crate::core::graph::Graph;

impl Graph {
    /// Calculates the shortest path length from a source node to all reachable nodes.
    ///
    /// Returns a HashMap with one entry per reachable node, and it's path length.
    ///
    /// # Examples
    /// ```
    /// # use networkg::core::graph::Graph;
    /// let mut graph = Graph::new(4);
    /// graph.add_edge(0, 1);
    /// graph.add_edge(1, 2);
    /// graph.add_edge(2, 3);
    /// let shortest_paths = graph.single_source_shortest_path_length(0);
    /// assert_eq!(3, shortest_paths[&3]);
    /// ```
    ///
    /// # Implementation
    /// Implemented using breadth-first-search (BFS).
    pub fn single_source_shortest_path_length(&self, source: usize) -> HashMap<usize, usize> {
        let mut q = VecDeque::new();
        let mut path_lengths = HashMap::new();
        q.push_back(source);
        path_lengths.insert(source, 0);

        while let Some(reached_node) = q.pop_front() {
            let path_length = path_lengths[&reached_node] + 1;
            for reachable_node in &self.nodes[reached_node] {
                if !path_lengths.contains_key(&reachable_node) {
                    path_lengths.insert(*reachable_node, path_length);
                    q.push_back(*reachable_node);
                }
            }
        }
        path_lengths
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::iter::FromIterator;

    use crate::core::graph::Graph;

    #[test]
    fn test_distance_to_self() {
        let g = Graph::new(1);
        let path_lengths = g.single_source_shortest_path_length(0);
        assert_eq!(Some(&0), path_lengths.get(&0));
    }

    #[test]
    fn test_only_reaches_self_when_disconnected() {
        let g = Graph::new(10);
        let path_lengths = g.single_source_shortest_path_length(0);
        assert!(path_lengths.contains_key(&0));
        assert_eq!(1, path_lengths.len());
    }

    #[test]
    fn test_small() {
        let mut g = Graph::new(4);
        g.add_edge(0, 1).unwrap();
        g.add_edge(1, 2).unwrap();
        g.add_edge(2, 3).unwrap();
        g.add_edge(0, 3).unwrap();

        let path_lengths = g.single_source_shortest_path_length(0);
        let target: HashMap<usize, usize> =
            HashMap::from_iter([(0, 0), (1, 1), (2, 2), (3, 1)].iter().cloned());
        assert_eq!(target, path_lengths);
    }
}
