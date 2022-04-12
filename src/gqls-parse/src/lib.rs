#![deny(rust_2018_idioms)]

mod nodes;

pub use self::nodes::NodeKind;

pub use tree_sitter::{Language, Node, Parser, Query, Range, Tree};

pub fn traverse(tree: &Tree) -> impl Iterator<Item = Node<'_>> {
    tree_sitter_traversal::traverse_tree(tree, tree_sitter_traversal::Order::Pre)
}

pub struct Parents<'tree> {
    node: Node<'tree>,
}

impl<'tree> Iterator for Parents<'tree> {
    type Item = Node<'tree>;

    fn next(&mut self) -> Option<Self::Item> {
        self.node.parent()
    }
}

pub trait NodeExt<'tree> {
    fn parents(self) -> Parents<'tree>;
    fn sole_named_child(self) -> Node<'tree>;
    fn text(self, text: &str) -> &str;
    fn find_descendent(self, f: impl FnMut(&Node<'tree>) -> bool) -> Option<Node<'tree>>;
}

impl<'tree> NodeExt<'tree> for Node<'tree> {
    fn parents(self) -> Parents<'tree> {
        Parents { node: self }
    }

    fn sole_named_child(self) -> Node<'tree> {
        assert_eq!(self.child_count(), 1);
        self.child(0).unwrap()
    }

    fn text(self, source: &str) -> &str {
        self.utf8_text(source.as_bytes()).unwrap()
    }

    fn find_descendent(self, f: impl FnMut(&Node<'tree>) -> bool) -> Option<Node<'tree>> {
        tree_sitter_traversal::traverse(self.walk(), tree_sitter_traversal::Order::Pre).find(f)
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

#[cfg(test)]
mod node_generator;
