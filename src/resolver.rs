use std::path::{Path, PathBuf};

use anyhow::Context;

use crate::ts_config::TsConfig;

pub struct Resolver {
    project_url: PathBuf,
    base_url: Option<PathBuf>,
    is_ts: bool,
}

impl TryFrom<&Path> for Resolver {
    type Error = anyhow::Error;

    fn try_from(ts_config_path: &Path) -> Result<Self, Self::Error> {
        let ts_config_directory = ts_config_path
            .parent()
            .context(format!(
                "{} does not have a parent directory",
                ts_config_path.display()
            ))?
            .to_owned();

        let ts_config = TsConfig::try_from(ts_config_path);

        let base_url = match &ts_config {
            Ok(ts_config) => {
                Some(PathBuf::from(ts_config_directory.clone()).join(ts_config.base_url()))
            }
            Err(_) => None,
        };

        Ok(Self {
            base_url,
            is_ts: true,
            project_url: ts_config_directory,
        })
    }
}

impl Resolver {
    pub fn from_project_url(project_url: &Path) -> Self {
        Self {
            base_url: None,
            is_ts: false,
            project_url: project_url.to_owned(),
        }
    }

    pub fn resolve(&self, current_file: &Path, dependency: &str) -> anyhow::Result<PathBuf> {
        let path = if dependency.starts_with(".") {
            self.resolve_relative(current_file, dependency)
        } else if dependency.starts_with("..") {
            self.resolve_relative(
                current_file
                    .parent()
                    .context(format!("Failed getting parent of {}", dependency))?,
                dependency,
            )
        } else {
            self.resolve_node_module(dependency)
                .or(self.resolve_absolute(dependency))
        }?;

        Ok(path)
    }

    fn resolve_relative(&self, current_file: &Path, dependency: &str) -> anyhow::Result<PathBuf> {
        let current_dir = current_file
            .parent()
            .context(format!("Failed getting parent of {}", dependency))?;

        let target_path = current_dir.to_owned().join(dependency);
        let target_path = self.resolve_module(&target_path);

        target_path.clone().canonicalize().context(format!(
            "Failed finding path {}",
            target_path.display().to_string()
        ))
    }

    fn resolve_absolute(&self, dependency: &str) -> anyhow::Result<PathBuf> {
        let target_path = self
            .base_url
            .clone()
            .context("resolve_absolute should not be called without a base url")?;

        let target_path = target_path.join(dependency);
        let target_path = self.resolve_module(&target_path);

        target_path
            .canonicalize()
            .context(format!("Failed finding path {}", dependency))
    }

    fn resolve_node_module(&self, dependency: &str) -> anyhow::Result<PathBuf> {
        let target_path = self.project_url.clone();

        let target_path = target_path.join("node_modules").join(dependency);

        target_path
            .canonicalize()
            .context(format!("Failed finding path {}", dependency))
    }

    fn resolve_module(&self, module: &PathBuf) -> PathBuf {
        let mut target_path = module.clone();
        if module.is_dir() {
            target_path = module.join("index");
        }
        target_path.set_extension(self.extension());

        target_path
    }

    fn extension(&self) -> &str {
        if self.is_ts {
            "ts"
        } else {
            "js"
        }
    }
}
