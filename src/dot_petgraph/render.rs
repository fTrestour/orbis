use petgraph::{
    dot::{Config, Dot},
    prelude::DiGraph,
};

use super::DotNode;

pub trait FromDotNode<'a, G> {
    fn with_plop(graph: G) -> Self;
}

// TODO: make more generic
impl<'a> FromDotNode<'a, &'a DiGraph<DotNode, isize, usize>>
    for Dot<'a, &'a DiGraph<DotNode, isize, usize>>
{
    fn with_plop(graph: &'a DiGraph<DotNode, isize, usize>) -> Self {
        let test = Self::with_attr_getters(
            graph,
            &[Config::NodeNoLabel, Config::EdgeNoLabel],
            &|_, _| "".to_owned(),
            &|_, node| node.1.to_string(),
        );
        test
    }
}
