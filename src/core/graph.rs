use crate::core::io::read_edge_list_csv;

pub type Node = Vec<usize>;

pub struct Graph {
    pub nodes: Vec<Node>,
}

impl Graph {
    pub fn new(size: usize) -> Self {
        Graph {
            nodes: vec![vec![]; size],
        }
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
            //self.nodes[n2].push(n1);
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
