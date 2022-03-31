use super::*;

#[test]
fn test_exact() {
    assert_eq_test("かたづける", "かたずける");
    assert_eq_test("つつける", "つづける");
}

#[test]
fn test_mismatch() {
    assert!(!is_eq("せんたく", "かもく"));
}

#[test]
fn test_no_similar() {
    assert!(!diff("りんご", "あるく").similar());
    assert!(!diff("だいじ", "けっこ").similar());
    assert!(!diff("ほんと", "うばう").similar());
    assert!(!diff("へんこう", "こうこう").similar());
    assert!(!diff("だきしめる", "でけど").similar());
    assert!(!diff("ともだち", "たまごやき").similar());
    assert!(!diff("へんじ", "えんじる").similar());
}

/*
#[test]
fn test_japanese2() {
    let a = Hash::new_2("せいかく");
    println!("");
    let b = Hash::new_2("せっかく");
    println!("a:\t{:128b}", a.hash);
    println!("b:\t{:128b}", b.hash);
    println!("xor:\t{:128b}", (a - b).xor());
    println!("dist: {}", (a - b).dist());
}
*/

#[test]
fn test_japanese() {
    let a = Hash::new("しおり");
    let b = Hash::new("りんご");
    let xor = (a - b).xor();
    println!("a\t{:128b}", a.hash);
    println!("b\t{:128b}", b.hash);
    println!("XOR\t{:128b}", xor);
    println!("dist: {}", (a - b).hamming());
}

#[test]
fn test_distance() {}

#[test]
fn test_no_weights() {
    //
}

#[test]
fn test_similar() {
    assert!(diff("つきあう", "づきあう").similar());
    assert!(diff("ふく", "ふぐ").similar());
    assert!(diff("つきあう", "ずきあう").similar());
    assert!(diff("ふむ", "すむ").similar());
    assert!(diff("すいぶん", "すいふん").similar());
    assert!(diff("すっぴん", "すっぱん").similar());
    assert!(diff("たんがい", "だんがい").similar());
    assert!(diff("だんさん", "たんざん").similar());
    assert!(diff("きづく", "きずく").similar());
    assert!(diff("きずく", "きつく").similar());
    assert!(diff("きづく", "きつく").similar());
    assert!(diff("ひょか", "ひょうか").similar());
    assert!(diff("えいがかん", "えいご").similar());
}

pub fn diff(a: &str, b: &str) -> Difference {
    Hash::new(a) - Hash::new(b)
}

pub fn assert_eq_test(a: &str, b: &str) {
    assert_eq!(Hash::new(a), Hash::new(b));
}

pub fn is_eq(a: &str, b: &str) -> bool {
    Hash::new(a) == Hash::new(b)
}
