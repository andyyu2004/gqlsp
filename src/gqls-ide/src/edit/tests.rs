use crate::point;
use ropey::Rope;

#[macro_export]
macro_rules! patch {
    ($a:literal:$b:literal..$x:literal:$y:literal => $with:expr) => {
        $crate::Patch { range: $crate::range!($a: $b..$x: $y), with: $with.to_owned() }
    };
}

#[test]
fn test_apply_patch() {
    let mut rope = Rope::default();

    assert_eq!(
        patch!(0:0..0:0 => "the").apply(&mut rope),
        tree_sitter::InputEdit {
            start_byte: 0,
            old_end_byte: 0,
            new_end_byte: 3,
            start_position: point!(0:0),
            old_end_position: point!(0:0),
            new_end_position: point!(0:3),
        }
    );
    assert_eq!(rope.to_string(), "the");

    assert_eq!(
        patch!(0:3..0:3 => " quick fox").apply(&mut rope),
        tree_sitter::InputEdit {
            start_byte: 3,
            old_end_byte: 3,
            new_end_byte: 13,
            start_position: point!(0:3),
            old_end_position: point!(0:3),
            new_end_position: point!(0:13),
        }
    );
    assert_eq!(rope.to_string(), "the quick fox");

    assert_eq!(
        patch!(0:13..0:13 => "\njumps").apply(&mut rope),
        tree_sitter::InputEdit {
            start_byte: 13,
            old_end_byte: 13,
            new_end_byte: 19,
            start_position: point!(0:13),
            old_end_position: point!(0:13),
            new_end_position: point!(1:5),
        }
    );
    assert_eq!(rope.to_string(), "the quick fox\njumps");

    assert_eq!(
        patch!(1:5..1:5 => "\nover\nthe").apply(&mut rope),
        tree_sitter::InputEdit {
            start_byte: 19,
            old_end_byte: 19,
            new_end_byte: 28,
            start_position: point!(1:5),
            old_end_position: point!(1:5),
            new_end_position: point!(3:3),
        }
    );
    assert_eq!(rope.to_string(), "the quick fox\njumps\nover\nthe");

    assert_eq!(
        patch!(2:0..3:3 => "nothing").apply(&mut rope),
        tree_sitter::InputEdit {
            start_byte: 20,
            old_end_byte: 28,
            new_end_byte: 27,
            start_position: point!(2:0),
            old_end_position: point!(3:3),
            new_end_position: point!(2:7),
        }
    );
    assert_eq!(rope.to_string(), "the quick fox\njumps\nnothing");
}
