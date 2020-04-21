use tree_sitter_core::api::*;
use tree_sitter_languages::lookup;

// Languages::lookup is a function of tree_sitter_languages,
// contained in the lib.rs, that chooses the right parser
// according to the input string

fn main() {
    let language = Languages::lookup("rust").unwrap();
    let mut parser = Parser::new();
    parser.set_language(language).unwrap();

    let source_code = "fn test() {}";
    let tree = parser.parse(source_code, None).unwrap();
    let root_node = tree.root_node();

    assert_eq!(root_node.kind(), "source_file");
    assert_eq!(root_node.start_position().column, 0);
    assert_eq!(root_node.end_position().column, 12);
}
