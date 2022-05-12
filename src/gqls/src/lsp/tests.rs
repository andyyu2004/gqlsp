use lsp_types::WorkspaceFolder;
use maplit::hashmap;

use crate::config::DEFAULT_PROJECT;
use crate::lsp;

macro_rules! fixture_path {
    ($name:literal) => {{
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src/lsp/fixtures")
            .join($name);
        assert!(path.exists(), "fixture `{}` does not exist (path `{}`)", $name, path.display());
        path
    }};
}

macro_rules! fixtures {
    ($name:literal) => {{ lsp_types::Url::from_file_path(fixture_path!($name)).unwrap() }};
}

#[test]
fn test_project_discovery() -> anyhow::Result<()> {
    let path = fixture_path!("simple");
    let config = lsp::read_config(&path)?;
    assert!(config.is_some());
    let mut projects = lsp::discover_projects(std::iter::once(WorkspaceFolder {
        uri: fixtures!("simple"),
        name: String::new(),
    }))?;
    projects.iter_mut().for_each(|(_, files)| files.sort());
    assert_eq!(
        projects,
        hashmap! {
            DEFAULT_PROJECT.to_owned() => vec![
                (path.join("bar.graphql"), "".to_owned()),
                (path.join("foo.graphql"), "".to_owned()),
            ]
        }
    );
    Ok(())
}

#[test]
fn test_project_discovery_multi() -> anyhow::Result<()> {
    let path = fixture_path!("multi");
    let projects = lsp::discover_projects(std::iter::once(WorkspaceFolder {
        uri: fixtures!("multi"),
        name: String::new(),
    }))?;
    assert_eq!(
        projects,
        hashmap! {
            "bar".to_owned() => vec![
                (path.join("bar.graphql"), "".to_owned()),
            ],
            "foo".to_owned() => vec![
                (path.join("foo.graphql"), "".to_owned()),
            ]
        }
    );
    Ok(())
}
