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

        let ts_config = TsConfig::try_from(ts_config_path)?;
        let base_url = ts_config.compiler_options.base_url.map(PathBuf::from);

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

    pub fn relative_to_project_root<'a>(&self, path: &'a Path) -> &'a Path {
        path.strip_prefix(self.project_url.clone()).unwrap()
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
        };

        path.context(format!(
            "Failed resolving {} for file {}",
            dependency,
            current_file.display()
        ))
    }

    fn resolve_relative(&self, current_file: &Path, dependency: &str) -> anyhow::Result<PathBuf> {
        let current_dir = current_file
            .parent()
            .context(format!("Failed getting parent of {}", dependency))?;

        let original_target_path = current_dir.to_owned().join(dependency);
        let target_path = self.resolve_module(&original_target_path)?;

        target_path
            .clone()
            .canonicalize()
            .context(format!(
                "Failed finding path resolving relative path {}",
                original_target_path.display().to_string()
            ))
            .context(format!("{}", target_path.display().to_string()))
    }

    fn resolve_absolute(&self, dependency: &str) -> anyhow::Result<PathBuf> {
        let target_path = self
            .base_url
            .clone()
            .context("resolve_absolute should not be called without a base url")?;

        let origin_target_path = target_path.join(dependency);
        let target_path = self.resolve_module(&origin_target_path.clone())?;

        target_path.canonicalize().context(format!(
            "Failed resolving absolute path {}",
            origin_target_path.display()
        ))
    }

    fn resolve_node_module(&self, dependency: &str) -> anyhow::Result<PathBuf> {
        let target_path = self.project_url.clone();

        let mut dependency_split = dependency.split('/');
        let mut dependency_path = dependency_split.next().unwrap().to_owned();
        if dependency_path.starts_with("@") {
            dependency_path = format!(
                "{}/{}",
                dependency_path,
                dependency_split.next().unwrap().to_string()
            );
        }
        let target_path = target_path.join("node_modules").join(dependency_path);

        target_path
            .canonicalize()
            .context("resolve_node_module")
            .context(format!("Failed resolving module {}", dependency))
    }

    fn resolve_module(&self, module: &PathBuf) -> anyhow::Result<PathBuf> {
        module
            .canonicalize()
            .or(module
                .join(format!("index.{}", self.extension()))
                .canonicalize())
            .or(PathBuf::from(format!("{}.{}", module.display(), self.extension())).canonicalize())
            .context("Failed resolving file")
    }

    fn extension(&self) -> &str {
        if self.is_ts {
            "ts"
        } else {
            "js"
        }
    }
}
