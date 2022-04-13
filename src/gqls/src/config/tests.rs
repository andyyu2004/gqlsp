use anyhow::Result;
use globset::Glob;
use maplit::btreemap;

use crate::config::OneOrMany;

use super::{Config, ProjectConfig};

#[test]
fn test_parse_config_ignores_unknown_fields() {
    toml::toml! {
        schema = "foo.graphql"
        random = "wer"
    }
    .try_into::<Config>()
    .unwrap();
}

#[test]
fn test_parse_config() -> Result<()> {
    let project_config = toml::toml! {
        schema = "foo.graphql"
    }
    .try_into::<ProjectConfig>()?;

    let expected_project_config =
        ProjectConfig { schema: OneOrMany::One(Glob::new("foo.graphql")?) };
    assert_eq!(project_config, expected_project_config);

    let config = toml::toml! {
        schema = "foo.graphql"
    }
    .try_into::<Config>()?;

    assert_eq!(config, Config::Project(expected_project_config));

    let config = toml::toml! {
        [project1]
        schema = ["foo.graphql", "bar.graphql"]

        [project2]
        schema = "**/*.graphql"
    }
    .try_into::<Config>()?;

    assert_eq!(
        config,
        Config::Projects(btreemap! {
           "project1".to_owned() => ProjectConfig {
               schema: OneOrMany::Many(vec![Glob::new("foo.graphql")?,
               Glob::new("bar.graphql")?])
            },
           "project2".to_owned() => ProjectConfig {
               schema: OneOrMany::One(Glob::new("**/*.graphql")?)
           }
        })
    );
    Ok(())
}
