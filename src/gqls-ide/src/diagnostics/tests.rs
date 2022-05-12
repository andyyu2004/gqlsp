use gqls_fixture::Fixture;

use crate::Ide;

fn test(fixture: &Fixture) {
    let ide = Ide::from_fixture_allow_errors(fixture);
}

#[cfg(test)]
mod syntax;
