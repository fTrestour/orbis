use std::fmt::Display;

pub struct DotNode(Vec<NodeAttribute>);
impl DotNode {
    pub fn new(attributes: Vec<NodeAttribute>) -> Self {
        DotNode(attributes)
    }
}
impl Display for DotNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stringified_attributes: Vec<_> = self
            .0
            .iter()
            .map(|attribute| attribute.to_string())
            .collect();
        let s = stringified_attributes.join(" ");
        write!(f, "{}", s)
    }
}

pub enum NodeAttribute {
    Label(String),
    Shape(NodeShape),
    Color(String),
    FillColor(String),
    FontColor(String),
    FontName(String),
    Style(Vec<String>),
    Href(String),
}
impl Display for NodeAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}", self.to_key(), self.to_value())
    }
}
impl NodeAttribute {
    fn to_key(&self) -> &str {
        match self {
            NodeAttribute::Label(_) => "label",
            NodeAttribute::Shape(_) => "shape",
            NodeAttribute::Color(_) => "color",
            NodeAttribute::FillColor(_) => "fillcolor",
            NodeAttribute::FontColor(_) => "fontcolor",
            NodeAttribute::Style(_) => "style",
            NodeAttribute::Href(_) => "href",
            NodeAttribute::FontName(_) => "fontname",
        }
    }

    fn to_value(&self) -> String {
        match self {
            NodeAttribute::Label(l) => format!("\"{}\"", l),
            NodeAttribute::Shape(s) => s.to_string(),
            NodeAttribute::Color(c) => c.to_owned(),
            NodeAttribute::FillColor(c) => c.to_owned(),
            NodeAttribute::FontColor(c) => c.to_owned(),
            NodeAttribute::Style(s) => format!("\"{}\"", s.join(",")),
            NodeAttribute::Href(h) => format!("\"{}\"", h),
            NodeAttribute::FontName(f) => f.to_string(),
        }
    }
}

pub enum NodeShape {
    Box,
    Ellipse,
}
impl Display for NodeShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                NodeShape::Box => "box".to_owned(),
                NodeShape::Ellipse => "ellipse".to_owned(),
            }
        )
    }
}
