//! Basic DOM data structures.

use std::collections::{HashMap, HashSet};

pub type AttrMap = HashMap<String, String>;

#[derive(Debug)]
pub struct ParserInfo {
    line_num: usize,
    col_num: usize,
}

impl std::fmt::Display for ParserInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<{},{}>", self.line_num, self.col_num)
    }
}

pub fn new_parser_info(line_num: usize, col_num: usize) -> ParserInfo {
    ParserInfo { line_num, col_num }
}

#[derive(Debug)]
pub struct Node {
    // data common to all nodes:
    pub children: Vec<Node>,

    // data specific to each node type:
    pub node_type: NodeType,

    // format data
    pub parsert_info: ParserInfo,
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

#[derive(Debug)]
pub enum NodeType {
    Element(ElementData),
    Text(String),
}

impl std::fmt::Display for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug)]
pub struct ElementData {
    pub tag_name: String,
    pub attributes: AttrMap,
}

impl std::fmt::Display for ElementData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}:{:?}", self.tag_name, self.attributes)
    }
}

// Constructor functions for convenience:

pub fn text(data: String, parsert_info: ParserInfo) -> Node {
    Node {
        children: vec![],
        node_type: NodeType::Text(data),
        parsert_info,
    }
}

pub fn elem(name: String, attrs: AttrMap, children: Vec<Node>, parsert_info: ParserInfo) -> Node {
    Node {
        children: children,
        node_type: NodeType::Element(ElementData {
            tag_name: name,
            attributes: attrs,
        }),
        parsert_info,
    }
}

// Element methods

impl ElementData {
    pub fn id(&self) -> Option<&String> {
        self.attributes.get("id")
    }

    pub fn classes(&self) -> HashSet<&str> {
        match self.attributes.get("class") {
            Some(classlist) => classlist.split(' ').collect(),
            None => HashSet::new(),
        }
    }
}

#[cfg(test)]
mod test_dom {
    use super::*;

    #[test]
    fn test_dom() {
        let mut root = text("root".to_owned(), new_parser_info(1, 1));
        let one_node = text("one".to_owned(), new_parser_info(1, 1));
        let two_node = text("two".to_owned(), new_parser_info(1, 1));
        root.children = vec![one_node, two_node];
        println!("root = {}", root);
    }
}
