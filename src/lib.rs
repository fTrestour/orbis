mod dot_petgraph;
pub mod graph;
mod parse;
pub mod resolver;
mod ts_config;

use std::path::{Path, PathBuf};

use anyhow::Context;
use parse::try_parse;
use resolver::Resolver;

pub fn get_imports(path: &Path, resolver: &Resolver) -> anyhow::Result<Vec<PathBuf>> {
    Ok(try_parse(path)
        .context(format!("Error parsing file {}", path.display()))?
        .body
        .into_iter()
        .filter_map(|module_item| {
            let import = module_item.module_decl()?.import()?;
            let import = import.src.value.to_string();

            let resolved_import = resolver
                .resolve(path, &import)
                .map_err(|err| eprintln!("{:?}", err));
            resolved_import.ok()
        })
        .collect())
}
