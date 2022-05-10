use std::collections::BTreeMap;
use std::path::Path;

use globset::Glob;
use serde::{Deserialize, Deserializer};

#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
#[serde(untagged)]
pub enum Config {
    Project(ProjectConfig),
    Projects(Projects),
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
pub struct Projects {
    projects: BTreeMap<String, ProjectConfig>,
}

impl Projects {
    #[cfg(test)]
    fn new(projects: BTreeMap<String, ProjectConfig>) -> Self {
        Self { projects }
    }
}

pub const DEFAULT_PROJECT: &str = "default";

impl Config {
    /// Return list of projects that a `path` belongs to
    pub fn project_matches(&self, path: &Path) -> Vec<&str> {
        match self {
            Config::Project(project) =>
                project.matches(path).then(|| DEFAULT_PROJECT).into_iter().collect(),
            Config::Projects(projects) => projects
                .projects
                .iter()
                .filter_map(|(name, project)| project.matches(path).then(|| name.as_ref()))
                .collect(),
        }
    }

    pub(crate) fn read(path: &Path) -> anyhow::Result<Self> {
        assert!(path.exists());
        let s = std::fs::read_to_string(path)?;
        let ext = path.extension();
        if ext == Some("toml".as_ref()) {
            Ok(toml::from_str(&s)?)
        } else if ext.is_none() || ext == Some("yaml".as_ref()) {
            Ok(serde_yaml::from_str(&s)?)
        } else {
            Err(anyhow::anyhow!("unsupported config file extension (yaml or toml only)"))
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
pub struct ProjectConfig {
    #[serde(deserialize_with = "deserialize_schema")]
    schema: OneOrMany<Glob>,
}

// minor hack as the default `Glob` deserialize impl doesn't work with owned strings
fn deserialize_schema<'de, D>(deserializer: D) -> Result<OneOrMany<Glob>, D::Error>
where
    D: Deserializer<'de>,
{
    let strings = OneOrMany::<String>::deserialize(deserializer)?;
    match strings.as_ref().map(|s| s.trim_start_matches("./")) {
        OneOrMany::One(glob) =>
            Glob::new(glob).map_err(serde::de::Error::custom).map(OneOrMany::One),
        OneOrMany::Many(globs) => globs
            .iter()
            .map(AsRef::as_ref)
            .map(Glob::new)
            .collect::<Result<Vec<_>, _>>()
            .map_err(serde::de::Error::custom)
            .map(OneOrMany::Many),
    }
}

impl ProjectConfig {
    fn matches(&self, path: &Path) -> bool {
        // FIXME don't keep recompiling the glob
        self.schema.iter().any(|glob| glob.compile_matcher().is_match(path))
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum OneOrMany<T> {
    One(T),
    Many(Vec<T>),
}

pub enum OneOrManyRef<'a, T> {
    One(&'a T),
    Many(&'a [T]),
}

impl<'a, T> OneOrManyRef<'a, T> {
    pub fn map<U>(self, mut f: impl FnMut(&'a T) -> U) -> OneOrMany<U> {
        match self {
            Self::One(x) => OneOrMany::One(f(x)),
            Self::Many(xs) => OneOrMany::Many(xs.iter().map(f).collect()),
        }
    }
}

impl<T> OneOrMany<T> {
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.into_iter()
    }

    pub fn as_ref(&self) -> OneOrManyRef<'_, T> {
        match self {
            OneOrMany::One(x) => OneOrManyRef::One(x),
            OneOrMany::Many(xs) => OneOrManyRef::Many(xs),
        }
    }
}

impl<'a, T> IntoIterator for &'a OneOrMany<T> {
    type IntoIter = std::slice::Iter<'a, T>;
    type Item = &'a T;

    fn into_iter(self) -> Self::IntoIter {
        match &self {
            OneOrMany::One(x) => std::slice::from_ref(x).iter(),
            OneOrMany::Many(xs) => xs.iter(),
        }
    }
}

impl<T> IntoIterator for OneOrMany<T> {
    type IntoIter = std::vec::IntoIter<T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        Vec::from(self).into_iter()
    }
}

impl<T> From<OneOrMany<T>> for Vec<T> {
    fn from(from: OneOrMany<T>) -> Self {
        match from {
            OneOrMany::One(val) => vec![val],
            OneOrMany::Many(vec) => vec,
        }
    }
}

#[cfg(test)]
mod tests;
