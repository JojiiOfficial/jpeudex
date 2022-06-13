use super::*;

#[test]
fn test_exact() {
    assert_eq_test("かたづける", "かたずける");
    assert_eq_test("つつける", "つづける");
    /*
    assert_eq_test("すいぶん", "ずいぶん");
    assert_eq_test("かさむき", "かざむき");
    */
}

#[test]
fn test_mismatch() {
    assert!(!is_eq("せんたく", "かもく"));
    assert!(!is_eq("かたつけ", "かたづける"));
}

#[test]
fn test_no_similar() {
    assert!(!diff("りんご", "あるく").similar());
    //assert!(!diff("だいじ", "けっこ").similar());
    assert!(!diff("ほんと", "うばう").similar());
    assert!(!diff("へんこう", "こうこう").similar());
    assert!(!diff("だきしめる", "でけど").similar());
    assert!(!diff("ともだち", "たまごやき").similar());
    assert!(!diff("へんじ", "えんじる").similar());
}

#[test]
fn test_japanese2() {
    let a = Hash::new_unchecked("しょがくせい");
    println!("");
    let b = Hash::new_unchecked("しょうがくせい");
    println!("a:\t{:128b}", a.hash);
    println!("b:\t{:128b}", b.hash);
    println!("xor:\t{:128b}", (a - b).xor());
    println!("dist: {}", (a - b).dist());
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
    assert!(diff("すみ", "ずみ").similar());
    assert!(diff("つきあう", "ずきあう").similar());
    assert!(diff("すいぶん", "すいふん").similar());
    assert!(diff("すっぴん", "すっぱん").similar());
    assert!(diff("たんがい", "だんがい").similar());
    assert!(diff("だんさん", "たんざん").similar());
    assert!(diff("きづく", "きずく").similar());
    assert!(diff("きずく", "きつく").similar());
    assert!(diff("すいぶん", "ずいぶん").similar());
    assert!(diff("きづく", "きつく").similar());
    assert!(diff("みすむし", "みずむし").similar());
    assert!(diff("ひょか", "ひょうか").similar());
    assert!(diff("えいが", "えいご").similar());
    assert!(diff("あおそら", "あおぞら").similar());
    assert!(diff("かざむき", "かさむき").similar());
}

pub fn diff(a: &str, b: &str) -> Difference {
    Hash::new_unchecked(a) - Hash::new_unchecked(b)
}

pub fn assert_eq_test(a: &str, b: &str) {
    assert_eq!(Hash::new_unchecked(a), Hash::new_unchecked(b));
}

pub fn is_eq(a: &str, b: &str) -> bool {
    Hash::new_unchecked(a) == Hash::new_unchecked(b)
}
