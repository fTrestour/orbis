use std::{fs::File, io::BufReader, path::Path};

use anyhow::Ok;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct CompilerOptions {
    base_url: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TsConfig {
    compiler_options: CompilerOptions,
}

impl TryFrom<&Path> for TsConfig {
    type Error = anyhow::Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let ts_config = serde_json::from_reader(reader)?;
        Ok(ts_config)
    }
}

impl TsConfig {
    pub fn base_url(&self) -> &String {
        &self.compiler_options.base_url
    }
}
