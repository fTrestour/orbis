use std::{fs::File, io::BufReader, path::Path};

use anyhow::{Context, Ok};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CompilerOptions {
    pub base_url: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TsConfig {
    pub compiler_options: CompilerOptions,
}

impl TryFrom<&Path> for TsConfig {
    type Error = anyhow::Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let file = File::open(path).context(format!("Error opening file {}", path.display()))?;
        let reader = BufReader::new(file);

        let ts_config = serde_json::from_reader(reader)?;
        Ok(ts_config)
    }
}
