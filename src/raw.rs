//! The raw jpeudex API.

/// Number of letters in our phone map.
const LETTERS: u8 = 26;

/// The sound table.
///
/// The first bit each describes a certain property of the phone:
///
/// | Position | Modifier | Property     | Phones                   |
/// |----------|---------:|--------------|:------------------------:|
/// | 1        | 1        | Discriminant | (for tagging duplicates) |
/// | 2        | 2        | Nasal        | mn                       |
/// | 3        | 4        | Fricative    | fvsjxzhct                |
/// | 4        | 8        | Plosive      | pbtdcgqk                 |
/// | 5        | 16       | Dental       | tdnzs                    |
/// | 6        | 32       | Liquid       | lr                       |
/// | 7        | 64       | Labial       | bfpv                     |
/// | 8        | 128      | Confident¹   | lrxzq                    |
///
/// ¹hard to misspell.
///
/// Vowels are, to maxize the XOR distance, represented by 0 and 1 (open and close, respectively).
const PHONES: [u8; LETTERS as usize] = [
    0b11000001, //a
    0b01001000, //b
    99,         //c
    0b00011000, //d
    0b10100001, //e
    0b00000100, //f
    0b10001000, //g
    0b00100000, //h
    0b10100001, //i
    0b10011000, //j
    0b10001000, //k
    99,         //l
    0b00000010, //m
    0b00010010, //n
    0b10010001, //o
    0b01001000, //p
    99,         //q
    0b00010001, //r
    0b10011000, //s
    0b00011000, //t
    0b00010001, //u
    99,         //v
    0b01000100, //w
    99,         //x
    0b00000000, //y
    0b10011000, //z
];

const I_PHONE: u8 = PHONES[8];
const U_PHONE: u8 = PHONES[20];

#[inline]
pub fn map_phone(a: char, b: Option<char>) -> (u8, Option<u8>) {
    match (a, b) {
        ('z', Some('u')) => (0b0011000, Some(U_PHONE)),  // ず
        ('d', Some('u')) => (0b0011000, Some(U_PHONE)),  // づ
        ('t', Some('u')) => (0b0011000, Some(U_PHONE)),  // つ
        ('z', Some('i')) => (0b00011000, Some(I_PHONE)), // じ
        ('d', Some('i')) => (0b00011000, Some(I_PHONE)), // ぢ
        ('t', Some('i')) => (0b00011000, Some(I_PHONE)), // ち
        ('h', Some('u')) => (0b00000100, Some(U_PHONE)), // ふ
        _ => {
            let a_i = get_phone(a as u8).unwrap();
            let b_i = b.and_then(|b| get_phone(b as u8));
            (a_i, b_i)
        }
    }
}

/// Filter a non-head character.
///
/// `None` means "skip this character", whereas `Some(x)` means "push x".
///
/// jpeudex works by building up a hash by this filter and then XORing to get the difference.
#[inline(always)]
pub fn get_phone(mut x: u8) -> Option<u8> {
    x = x.wrapping_sub(b'a');
    (x < LETTERS).then(|| PHONES[x as usize])
}
