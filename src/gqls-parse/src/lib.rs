use tree_sitter::{Language, Parser, Tree};

extern "C" {
    fn tree_sitter_graphql() -> Language;
}

pub fn parse(text: &str, old_tree: Option<&Tree>) -> Tree {
    let mut parser = make_parser();
    parser.parse(text, old_tree).unwrap()
}

fn make_parser() -> Parser {
    let language = unsafe { tree_sitter_graphql() };
    let mut parser = Parser::new();
    parser.set_language(language).unwrap();
    parser
}

#[test]
fn test_make_parser() {
    make_parser();
}
