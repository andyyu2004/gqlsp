use gqls_db::DefDatabase;
use gqls_fixture::{fixture, Fixture};
use ropey::Rope;

use crate::Ide;

fn test(fixture: &Fixture, to: &str) {
    let ide = Ide::from_fixture(&fixture);
    let mut positions = fixture.positions();

    let position = positions.next().expect("fixture must have at least one position");
    assert!(positions.next().is_none());

    let snapshot = ide.snapshot();
    let name = snapshot.name_at(position).expect("position must be at a name");

    let file_patches = snapshot.rename(position, to).unwrap();

    for mut file_patch in file_patches {
        let mut rope = Rope::from_str(&fixture.files()[file_patch.file].text);
        file_patch.patches.sort();
        for patch in &mut file_patch.patches.iter().rev() {
            _ = patch.apply(&mut rope);
        }

        // The expected result is just calculated by doing a textual find and replace for the given name
        // This is probably good enough for graphql
        assert_eq!(
            fixture.files()[file_patch.file].text.replace(name.as_str(), to),
            rope.to_string(),
        );
    }
}

#[test]
fn test_rename_simple() {
    let fixture = fixture! {
        "foo" => "
            scalar Bar
                   #^
        "
    };
    test(&fixture, "Foobar");
}

#[test]
fn test_rename_field_references() {
    let before = fixture! {
        "foo" => "
            scalar Bar
                  # ^

            type Foo implements I {
                bar: Bar
            }

            interface I {
                bar: Bar
            }
        "
    };
    test(&before, "Foobar");
}

#[cfg(test)]
mod prepare_rename;
