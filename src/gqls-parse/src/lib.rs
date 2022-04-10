#![deny(rust_2018_idioms)]

use tree_sitter::{Language, Parser, Query, Tree};

extern "C" {
    fn tree_sitter_graphql() -> Language;
}

pub fn language() -> Language {
    unsafe { tree_sitter_graphql() }
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
