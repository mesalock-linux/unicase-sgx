use std::prelude::v1::*;
use unicase::Ascii;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

//#[test]
pub fn test_case_insensitive() {
    let a = Ascii("foobar");
    let b = Ascii("FOOBAR");

    assert_eq!(a, b);
    assert_eq!(hash(&a), hash(&b));

    assert_eq!(a, "fooBar");
    assert_eq!("fooBar", a);
    assert_eq!(String::from("fooBar"), a);
    assert_eq!(a, String::from("fooBar"));
}

//#[cfg(feature = "nightly")]
//#[bench]
//fn bench_ascii_eq(b: &mut ::test::Bencher) {
//    b.bytes = b"foobar".len() as u64;
//    b.iter(|| assert_eq!(Ascii("foobar"), Ascii("FOOBAR")));
//}

//#[cfg(__unicase__iter_cmp)]
//#[test]
pub fn test_case_cmp() {
    assert!(Ascii("foobar") == Ascii("FOOBAR"));
    assert!(Ascii("a") < Ascii("B"));

    assert!(Ascii("A") < Ascii("b"));
    assert!(Ascii("aa") > Ascii("a"));

    assert!(Ascii("a") < Ascii("aa"));
    assert!(Ascii("a") < Ascii("AA"));
}

//#[cfg(__unicase__const_fns)]
//#[test]
pub fn test_ascii_new_const() {
    const _ASCII: Ascii<&'static str> = Ascii::new("");
}
