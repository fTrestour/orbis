use std::path::{Path, PathBuf};

use petgraph::{
    dot::{Config, Dot},
    prelude::DiGraph,
};

pub fn create_graph(path: &Path, imports: Vec<PathBuf>) -> String {
    let mut g = DiGraph::new();
    let path_node = g.add_node(path.display().to_string());

    for import in imports.iter() {
        let import_node = g.add_node(import.display().to_string());
        g.add_edge(path_node, import_node, 1);
    }

    Dot::with_config(&g, &[Config::EdgeNoLabel]).to_string()
}
