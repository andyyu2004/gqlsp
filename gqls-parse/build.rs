fn main() {
    let package = "tree-sitter-graphql";
    let source_directory = format!("../{}/src", package);
    let source_file = format!("{}/parser.c", source_directory);

    cc::Build::new().file(source_file).include(source_directory).compile(package);
}
