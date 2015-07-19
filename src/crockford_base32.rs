use biterator::{BiteratorLsb, build_from};
use itertools::Itertools;
use std::str;

const ALPHABET: &'static[u8] = b"0123456789abcdefghjkmnpqrstvwxyz";

// char.uppercase - '0'
const ALPHABET_INV: [i8; 43] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, -1, -1, -1, -1, -1, -1, -1,
                                10, 11, 12, 13, 14, 15, 16, 17, 1, 18, 19, 1, 20, 21, 0,
                                22, 23, 24, 25, 26, -1, 27, 28, 29, 30, 31];

// 2**CHUNK_SIZE = ALPHABET.len()
const CHUNK_SIZE: usize = 5;

pub fn encode(id: u64) -> String {
    let mut iter = BiteratorLsb::new(id).peekable();

    let mut v = Vec::with_capacity(13);
    while !iter.is_empty() {
        let chunk: usize = build_from(&mut iter, CHUNK_SIZE);
        v.push(ALPHABET[chunk]);
    }

    unsafe {str::from_utf8_unchecked(&v)}.to_string()
}

pub fn decode(bytes: &[u8]) -> Option<u64> {
    use std::ascii::AsciiExt;
    let mut failure = false;
    let n = {
        let mut iter = bytes.into_iter().filter_map(|&b| {
            match ALPHABET_INV.get(b.to_ascii_uppercase().wrapping_sub(b'0') as usize) {
                Some(&-1) | None => {failure = true; None},
                Some(&val) => Some(val as u8)
            }
        }).flat_map(|b| BiteratorLsb::new(b).pad_using(CHUNK_SIZE, |_| false));

        build_from(&mut iter, 64)
    };

    if failure { None }
    else { Some(n) }
}

#[test]
fn test_encode() {
    assert_eq!("gfnqb", encode(12310000u64));
}

#[test]
fn test_decode() {
    assert_eq!(Some(12310000u64), decode("gfnqb".as_bytes()));
    assert_eq!(Some(12310000u64), decode("GfnQb".as_bytes()));
    assert_eq!(None, decode(".gfnqb".as_bytes()));
}

#[test]
fn test_cyclic() {
    for i in 0 .. 1_000_000u64 {
        let enc = encode(i);
        let dec = decode(enc.as_bytes());
        assert_eq!(Some(i), dec);
    }
}
