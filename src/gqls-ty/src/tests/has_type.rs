use crate::{ty, val, TyDatabase};

use super::TestDB;

#[test]
fn test_has_type() {
    let db = TestDB::default();

    assert!(db.has_type(val!(null), ty!(Boolean)));
    assert!(db.has_type(val!(null), ty!([Boolean])));
    assert!(db.has_type(val!(null), ty!([!Boolean])));
    assert!(!db.has_type(val!(null), ty!(![Boolean])));

    assert!(db.has_type(val!(false), ty!(Boolean)));
    assert!(db.has_type(val!(1.2), ty!(Float)));
    assert!(db.has_type(val!(1.3), ty!(!Float)));
    assert!(!db.has_type(val!(1.4), ty!([!Float])));

    assert!(db.has_type(val!("20"), ty!(!ID)));
    assert!(db.has_type(val!("string"), ty!(!String)));

    assert!(!db.has_type(val!(null), ty!(!ID)));

    assert!(db.has_type(val!(A), ty!(A | B)));
    assert!(db.has_type(val!(B), ty!(A | B)));
    assert!(!db.has_type(val!(C), ty!(A | B)));

    assert!(db.has_type(val!([0, 1]), ty!([Int])));
    assert!(!db.has_type(val!([0, false]), ty!([Int])));

    assert!(db.has_type(val!([0, null]), ty!([Int])));
    assert!(db.has_type(val!([0, null]), ty!(![Int])));
    assert!(!db.has_type(val!([0, null]), ty!([!Int])));
}
