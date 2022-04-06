use tree_sitter::{Language, Parser};

extern "C" {
    fn tree_sitter_graphql() -> Language;
}

#[test]
fn test() {
    let language = unsafe { tree_sitter_graphql() };
    let mut parser = Parser::new();
    parser.set_language(language).unwrap();
}
