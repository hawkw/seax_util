use super::Atom;
use super::Atom::*;
#[test]
fn test_atom_show () {
    let mut a: Atom;

    a = Char('a');
    assert_eq!(format!("{}", a), "'a'");

    a = UInt(1u64);
    assert_eq!(format!("{}", a), "1");

    a = SInt(42i64);
    assert_eq!(format!("{}", a), "42");

    a = SInt(-1i64);
    assert_eq!(format!("{}", a), "-1");

    a = Float(5.55f64);
    assert_eq!(format!("{}", a), "5.55");

    a = Float(1f64);
    assert_eq!(format!("{}", a), "1");

}
