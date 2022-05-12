use gqls_fixture::{fixture, Fixture};

use crate::Ide;

fn test(fixture: &Fixture, allow: bool) {
    let ide = Ide::from_fixture(&fixture);
    for position in fixture.all_positions() {
        let result = ide.snapshot().prepare_rename(position);
        assert_eq!(result.is_ok(), allow);
    }
}

#[test]
fn test_prepare_rename_allowed_locations() {
    let fixture = fixture! {
        "foo" => "
            type Foo {
                #^^^
                bar: [Foo!]
                    #^^^^^^
            }
        "
    };
    test(&fixture, true);
}

#[test]
fn test_prepare_rename_disallowed_locations() {
    let fixture = fixture! {
        "foo" => "

           #^^^
            type Foo {
           #^^^^^   ^^^^
                bar: [Foo!]
              #^^^^^^      ^^^
            }
        #^^^^^^^^^^^^^^^^^^^^^^
        "
    };
    test(&fixture, false);
}
