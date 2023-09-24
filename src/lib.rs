use std::{path::Path, sync::Arc};
use swc::config::IsModule;
use swc_common::{
    errors::{ColorConfig, Handler},
    Globals, SourceMap, GLOBALS,
};
use swc_ecma_ast::{EsVersion, Program};
use swc_ecma_parser::{Syntax, TsConfig};

pub fn get_imports(path: &Path) -> Vec<String> {
    get_program(path)
        .module()
        .expect("Expected module, got script")
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

fn get_program(path: &Path) -> Program {
    let code_map = Arc::<SourceMap>::default();
    let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, true, Some(code_map.clone()));
    let compiler = swc::Compiler::new(code_map.clone());
    let file = code_map.load_file(path).expect("Failed to load file");
    let globals = Globals::default();

    let program = GLOBALS.set(&globals, || {
        compiler
            .parse_js(
                file,
                &handler,
                EsVersion::latest(),
                Syntax::Typescript(TsConfig {
                    tsx: true,
                    decorators: false,
                    dts: false,
                    no_early_errors: true,
                    disallow_ambiguous_jsx_like: true,
                }),
                IsModule::Bool(true),
                None,
            )
            .expect("Failed to parse file")
    });
    program
}
