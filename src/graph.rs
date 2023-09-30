use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use petgraph::{
    dot::{Config, Dot},
    graph::NodeIndex,
    prelude::DiGraph,
};

#[derive(Default)]
pub struct Graph {
    graph: DiGraph<String, isize, usize>,
    nodes: HashMap<String, usize>,
}

impl Graph {
    pub fn add_file(&mut self, path: &Path, imports: Vec<PathBuf>) {
        let origin = self.get_node(path);

        for import in imports.iter() {
            let target = self.get_node(import);
            if origin == target {
                continue;
            }

            if self.graph.find_edge(origin, target) == None {
                self.graph.add_edge(origin, target, 1);
            }
        }
    }

    fn get_node(&mut self, path: &Path) -> NodeIndex<usize> {
        let path = path.display().to_string();

        self.nodes
            .get(&path)
            .map(|id| NodeIndex::from(*id))
            .unwrap_or_else(|| {
                let new_node = self.graph.add_node(path.clone());
                self.nodes.insert(path, new_node.index());
                new_node
            })
    }

    pub fn to_dot(&self) -> String {
        Dot::with_config(&self.graph, &[Config::EdgeNoLabel]).to_string()
    }
}
