use crate::core::io::read_edge_list_csv;

pub type Node = Vec<usize>;
/// A directed graph.
///
/// A directed graph represented as an adjacency list.
/// Nodes are densely indexed, starting at 0,
/// meaning that a graph with N nodes will have the nodes:
/// 0, 1, ..., N-1.
///
/// # Examples:
/// Basic usage:
/// ```
/// # use networkg::core::graph::Graph;
/// let mut graph = Graph::new(10);
///
/// assert_eq!(10, graph.size());
/// assert!(graph.nodes[0].is_empty());
///
/// graph.add_edge(0, 9);
/// assert_eq!(vec![9], graph.nodes[0]);
/// ```
///
/// Read graph from csv:
/// ```no_run
/// # use networkg::core::graph::Graph;
/// # fn main() -> Result<(), String> {
/// let graph = Graph::from_csv("graph.csv", 10, b',')?;
/// # Ok(())
/// # }
/// ```
pub struct Graph {
    pub nodes: Vec<Node>,
}

impl Graph {
    pub fn new(size: usize) -> Self {
        Graph {
            nodes: vec![vec![]; size],
        }
    }

    pub fn size(&self) -> usize {
        self.nodes.len()
    }

    pub fn fully_connected(size: usize) -> Self {
        Graph {
            nodes: (0..size)
                .map(|n| (0..size).filter(|i| *i != n).collect())
                .collect(),
        }
    }

    pub fn from_csv(path: &str, size: usize, delimiter: u8) -> Result<Self, String> {
        let mut graph = Graph::new(size);
        graph.add_falliable_edges(read_edge_list_csv(path, delimiter)?)?;
        Ok(graph)
    }

    pub fn add_edge(&mut self, n1: usize, n2: usize) -> Result<(), String> {
        if n1.max(n2) >= self.nodes.len() {
            Err(format!(
                "Node with id {} does not fit in Graph of size {}.",
                n1.max(n2),
                self.nodes.len(),
            ))
        } else {
            self.nodes[n1].push(n2);
            Ok(())
        }
    }

    pub fn add_edges(
        &mut self,
        mut edges: impl Iterator<Item = (usize, usize)>,
    ) -> Result<(), String> {
        edges.try_for_each(|(n1, n2)| self.add_edge(n1, n2))
    }
    pub fn add_falliable_edges(
        &mut self,
        mut edges: impl Iterator<Item = Result<(usize, usize), impl std::error::Error>>,
    ) -> Result<(), String> {
        edges.try_for_each(|x| match x {
            Ok((n1, n2)) => self.add_edge(n1, n2),
            Err(error) => Err(error.to_string()),
        })
    }
}
