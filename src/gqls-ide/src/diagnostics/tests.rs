use gqls_fixture::Fixture;

use crate::Ide;

fn test(fixture: &Fixture) {
    let ide = Ide::from_fixture(fixture);
}

#[cfg(test)]
mod syntax;
