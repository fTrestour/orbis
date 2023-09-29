use std::path::Path;

use anyhow::{anyhow, bail};
use swc_common::SourceMap;
use swc_ecma_ast::{EsVersion, Module};
use swc_ecma_parser::{parse_file_as_module, Syntax};

pub fn try_parse(path: &Path) -> Result<Module, anyhow::Error> {
    let code_map = SourceMap::default();
    let file = code_map.load_file(path)?;

    let mut recovered_errors = vec![];
    let module = parse_file_as_module(
        &file,
        Syntax::Typescript(Default::default()),
        EsVersion::latest(),
        None,
        &mut recovered_errors,
    )
    .map_err(|e| {
        dbg!(e);
        anyhow!("Could not parse file")
    })?;

    if !recovered_errors.is_empty() {
        bail!("Errors when parsing file")
    } else {
        Ok(module)
    }
}
