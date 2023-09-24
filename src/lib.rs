use std::path::Path;
use swc_common::SourceMap;
use swc_ecma_ast::{EsVersion, Module};
use swc_ecma_parser::{parse_file_as_module, Syntax};

pub fn get_imports(path: &Path) -> Vec<String> {
    get_module(path)
        .body
        .into_iter()
        .filter_map(|module_item| {
            module_item
                .module_decl()
                .and_then(|module_declaration| module_declaration.import())
        })
        .map(|import| import.src.value.to_string())
        .collect()
}

fn get_module(path: &Path) -> Module {
    let code_map = SourceMap::default();
    let file = code_map.load_file(path).expect("Failed to load file");

    let mut recovered_errors = vec![];
    let program = parse_file_as_module(
        &file,
        Syntax::Typescript(Default::default()),
        EsVersion::latest(),
        None,
        &mut recovered_errors,
    )
    .expect("Failed to parse file");

    if !recovered_errors.is_empty() {
        println!("{:?}", recovered_errors);
        panic!("Failed to parse file");
    };

    program
}
