//! jpeudex is a Soundex-esque phonetic reduction/hashing algorithm, providing locality sensitive
//! "hashes" of words, based on the spelling and pronunciation.

use std::ops;

use jp_utils::{hiragana::Syllable, JapaneseExt};
use serde::{Deserialize, Serialize};

pub mod raw;

#[cfg(test)]
pub mod tests;

/// A phonetic hash.
///
/// Using the `Sub` implementation of the hashes will give you the difference.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug, Deserialize, Serialize)]
pub struct Hash {
    hash: u128,
}

impl Hash {
    pub fn new_unchecked(string: &str) -> Hash {
        let hash = Self::new(string);
        if hash.is_none() {
            panic!("Hash {string} is invalid");
        }
        hash.unwrap()
    }

    pub fn new(string: &str) -> Option<Hash> {
        if !string.is_hiragana() || string.trim().is_empty() {
            return None;
        }

        let chars: Vec<_> = string.chars().collect();

        let mut b = 0;
        let mut res = 0;
        let mut n = 1u8;
        let mut last = 0;

        let first_syll = split_syllable(chars[0]);
        let first_len = first_syll.len();
        let mut first: u128 = 0;
        for i in first_syll {
            first <<= 8;
            first |= i as u128;
            n <<= 1;
            last = i;
        }
        first <<= (16 - first_len) * 8;
        let mut added = 0;

        loop {
            b += 1;

            // Detect overflows
            if n == 0 || b >= chars.len() {
                break;
            }

            let mut iter = split_syllable(chars[b]);

            if chars[b] == 'っ' {
                iter.push(last);
            }

            for i in iter {
                if n == 0 {
                    break;
                }

                res <<= 8;
                res |= i as u128;
                n <<= 1;
                last = i;
                added += 1;
            }
        }

        if added < chars.len() {
            return None;
        }

        // Align to left of byte array
        let hash = res | first;
        Some(Hash { hash })
    }
}

fn split_syllable(inp: char) -> Vec<u8> {
    let curr_split = match Syllable::from_char(inp).get_splitted() {
        Some(s) => s,
        None => {
            return vec![];
        }
    };

    let mut c = curr_split.consonant().map(|i| i.to_romaji().unwrap());
    let mut v = curr_split.vowel().map(|i| i.to_romaji());
    if c.is_none() && v.is_some() {
        c = v;
        v = None;
    }
    let c = c.unwrap();
    let (cons_hash, vowel_hash) = raw::map_phone(c, v);

    let mut iter = vec![cons_hash];
    if let Some(vh) = vowel_hash {
        iter.push(vh);
    }
    iter
}

/// Get the inner hash value.
impl Into<u128> for Hash {
    #[inline]
    fn into(self) -> u128 {
        self.hash
    }
}

/// From u128 to eg. deserialize it
impl From<u128> for Hash {
    #[inline]
    fn from(hash: u128) -> Self {
        Hash { hash }
    }
}

/// Calculate the difference of two hashes.
impl ops::Sub for Hash {
    type Output = Difference;

    #[inline]
    fn sub(self, rhs: Hash) -> Difference {
        Difference {
            xor: self.hash ^ rhs.hash,
        }
    }
}

/// The difference between two words.
#[derive(Copy, Clone)]
pub struct Difference {
    xor: u128,
}

impl Difference {
    /// The "graduated" distance.
    ///
    /// This will assign different weights to each of the bytes Hamming weight and simply add it.
    /// For most use cases, this metric is the preferred one.
    #[inline(always)]
    pub fn dist(self) -> u32 {
        (self.xor as u8).count_ones()
            + ((self.xor >> 16) as u8).count_ones() * 2
            + ((self.xor >> 24) as u8).count_ones() * 2
            + ((self.xor >> 32) as u8).count_ones() * 2
            + ((self.xor >> 40) as u8).count_ones() * 3
            + ((self.xor >> 48) as u8).count_ones() * 3
            + ((self.xor >> 56) as u8).count_ones() * 5
            + ((self.xor >> 64) as u8).count_ones() * 8
            + ((self.xor >> 72) as u8).count_ones() * 8
            + ((self.xor >> 80) as u8).count_ones() * 12
            + ((self.xor >> 88) as u8).count_ones() * 12
            + ((self.xor >> 96) as u8).count_ones() * 14
            + ((self.xor >> 104) as u8).count_ones() * 14
            + ((self.xor >> 112) as u8).count_ones() * 14
            + ((self.xor >> 120) as u8).count_ones() * 14
    }

    /// The XOR distance.
    ///
    /// This is generally not recommend unless you have a very specific reason to prefer it over
    /// the other methods provided.
    #[inline]
    pub fn xor(self) -> u128 {
        self.xor
    }

    /// The "flat" Hamming based distance.
    ///
    /// This will let every byte carry the same weight, such that mismatch in the early and later
    /// mismatch counts the same.
    #[inline]
    pub fn hamming(self) -> u32 {
        self.xor.count_ones()
    }

    /// Does this difference constitute similarity?
    #[inline]
    pub fn similar(self) -> bool {
        self.dist() < 30
    }
}
