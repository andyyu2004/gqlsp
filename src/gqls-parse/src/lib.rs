#![deny(rust_2018_idioms)]

mod node;

pub use self::node::NodeKind;

use tree_sitter::{Language, Node, Parser, Query, Tree};

pub fn traverse(tree: &Tree) -> impl Iterator<Item = Node<'_>> {
    tree_sitter_traversal::traverse_tree(tree, tree_sitter_traversal::Order::Pre)
}

pub struct Parents<'a, 'tree> {
    node: &'a Node<'tree>,
}

impl<'tree> Iterator for Parents<'_, 'tree> {
    type Item = Node<'tree>;

    fn next(&mut self) -> Option<Self::Item> {
        self.node.parent()
    }
}

pub trait NodeExt<'tree> {
    fn parents(&self) -> Parents<'_, 'tree>;
}

impl<'tree> NodeExt<'tree> for Node<'tree> {
    fn parents(&self) -> Parents<'_, 'tree> {
        Parents { node: self }
    }
}

extern "C" {
    fn tree_sitter_graphql() -> Language;
}

pub fn language() -> Language {
    unsafe { tree_sitter_graphql() }
}

pub fn parse_fresh(text: &str) -> Tree {
    parse(text, None)
}

pub fn parse(text: &str, old_tree: Option<&Tree>) -> Tree {
    let mut parser = make_parser();
    parser.parse(text, old_tree).unwrap()
}

pub fn query(query: &str) -> Query {
    Query::new(language(), query).unwrap()
}

fn make_parser() -> Parser {
    let mut parser = Parser::new();
    parser.set_language(language()).unwrap();
    parser
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_parser() {
        make_parser();
    }

    #[test]
    fn test_parse_empty() {
        let mut parser = make_parser();
        let tree = parser.parse("", None).unwrap();
        assert_eq!(tree.root_node().to_sexp(), "(document)");
    }
}
