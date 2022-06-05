use super::super::Queries;
use gqls_syntax::{parse_fresh, Query, QueryCursor, QueryExt};

fn _test(query: &Query, source: &str, should_match: bool) {
    let tree = parse_fresh(source);
    let mut cursor = QueryCursor::new();
    assert_eq!(query.is_match(&mut cursor, tree.root_node(), source.as_bytes()), should_match);
}

#[test]
fn test_queries() {
    let _queries = Queries::default();
}
