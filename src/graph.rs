use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use petgraph::{dot::Dot, graph::NodeIndex, prelude::DiGraph};

use crate::dot_petgraph::{DotNode, FromDotNode, NodeAttribute, NodeShape};

#[derive(Default)]
pub struct Graph {
    graph: DiGraph<DotNode, isize, usize>,
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
                let new_node = self.graph.add_node(DotNode::new(vec![
                    NodeAttribute::Label(path.clone()),
                    NodeAttribute::Shape(NodeShape::Box),
                    NodeAttribute::Style(vec!["filled".to_owned(), "rounded".to_owned()]),
                    NodeAttribute::FillColor("\"#34495e\"".to_owned()),
                    NodeAttribute::Color("\"#34495e\"".to_owned()),
                    NodeAttribute::FontColor("white".to_owned()),
                    NodeAttribute::FontName("Helvetica".to_owned()),
                ]));
                self.nodes.insert(path, new_node.index());
                new_node
            })
    }

    pub fn to_dot(&self) -> String {
        Dot::with_plop(
            &self.graph,
            // TODO use url for pages load
        )
        .to_string()
    }
}
