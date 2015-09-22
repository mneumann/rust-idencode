use biterator::{BiteratorLsb, build_from};
use itertools::Itertools;
use std::str;

const ALPHABET: &'static[u8] = b"0123456789abcdefghjkmnpqrstvwxyz";

// char.uppercase - '0'
const ALPHABET_INV2: [i8; 43] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, -1, -1, -1, -1, -1, -1, -1,
                                10, 11, 12, 13, 14, 15, 16, 17, 1, 18, 19, 1, 20, 21, 0,
                                22, 23, 24, 25, 26, -1, 27, 28, 29, 30, 31];

const ALPHABET_INV_FIRST: usize = 48;
const ALPHABET_INV_LAST:  usize = 122;
const ALPHABET_INV: [i8; 75] = [
     0 /* 0 */,  1 /* 1 */,  2 /* 2 */,  3 /* 3 */,  4 /* 4 */,  5 /* 5 */,  6 /* 6 */,  7 /* 7 */,
     8 /* 8 */,  9 /* 9 */, -1 /* : */, -1 /* ; */, -1 /* < */, -1 /* = */, -1 /* > */, -1 /* ? */,
    -1 /* @ */, 10 /* A */, 11 /* B */, 12 /* C */, 13 /* D */, 14 /* E */, 15 /* F */, 16 /* G */,
    17 /* H */,  1 /* I */, 18 /* J */, 19 /* K */,  1 /* L */, 20 /* M */, 21 /* N */,  0 /* O */,
    22 /* P */, 23 /* Q */, 24 /* R */, 25 /* S */, 26 /* T */, -1 /* U */, 27 /* V */, 28 /* W */,
    29 /* X */, 30 /* Y */, 31 /* Z */, -1 /* [ */, -1 /* \ */, -1 /* ] */, -1 /* ^ */, -1 /* _ */,
    -1 /* ` */, 10 /* a */, 11 /* b */, 12 /* c */, 13 /* d */, 14 /* e */, 15 /* f */, 16 /* g */,
    17 /* h */,  1 /* i */, 18 /* j */, 19 /* k */,  1 /* l */, 20 /* m */, 21 /* n */,  0 /* o */,
    22 /* p */, 23 /* q */, 24 /* r */, 25 /* s */, 26 /* t */, -1 /* u */, 27 /* v */, 28 /* w */,
    29 /* x */, 30 /* y */, 31 /* z */
];

// 2**CHUNK_SIZE = ALPHABET.len()
const CHUNK_SIZE: usize = 5;

pub fn encode_slow(id: u64) -> String {
    let mut iter = BiteratorLsb::new(id).peekable();

    let mut v = Vec::with_capacity(13);
    while !iter.is_empty() {
        let chunk: usize = build_from(&mut iter, CHUNK_SIZE);
        v.push(ALPHABET[chunk]);
    }

    unsafe {str::from_utf8_unchecked(&v)}.to_string()
}

pub fn encode_id(id: u64) -> String {
    let mut v = Vec::with_capacity(13);
    encode_id_into_vec(&mut v, id);
    unsafe {str::from_utf8_unchecked(&v)}.to_string()
}

pub fn decode_id(bytes: &[u8]) -> Option<u64> {
    let mut id: u64 = 0;
    for &byte in bytes.iter().rev() {
        if byte as usize >= ALPHABET_INV_FIRST && byte as usize <= ALPHABET_INV_LAST {
            let val = ALPHABET_INV[(byte as usize - ALPHABET_INV_FIRST)];
            if val >= 0 {
                let new_id = (id << 5) | (val as u64);
                if new_id < id {
                    return None; // Overflow
                }
                id = new_id;
            } else {
                return None;
            }
        } else {
            return None;
        }
    }
    return Some(id);
}

#[inline]
pub fn encode_id_into_vec(vec: &mut Vec<u8>, id: u64) {
    let mut n = id;

    loop {
        vec.push(ALPHABET[(n&31)as usize]);
        n = n >> 5;
        if n == 0 { break; }
    }
}

pub fn decode(bytes: &[u8]) -> Option<u64> {
    use std::ascii::AsciiExt;
    let mut failure = false;
    let n = {
        let mut iter = bytes.into_iter().filter_map(|&b| {
            match ALPHABET_INV2.get(b.to_ascii_uppercase().wrapping_sub(b'0') as usize) {
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
fn test_encode_slow() {
    assert_eq!("gfnqb", encode_slow(12310000u64));
}

#[test]
fn test_encode_id() {
    assert_eq!("gfnqb", encode_id(12310000u64));
}

#[test]
fn test_decode() {
    assert_eq!(Some(12310000u64), decode("gfnqb".as_bytes()));
    assert_eq!(Some(12310000u64), decode("GfnQb".as_bytes()));
    assert_eq!(None, decode(".gfnqb".as_bytes()));
}

#[test]
fn test_decode_id() {
    assert_eq!(Some(12310000u64), decode_id(b"gfnqb"));
    assert_eq!(Some(12310000u64), decode_id(b"GfnQb"));
    assert_eq!(None, decode_id(b".gfnqb"));
}

#[test]
fn test_cyclic() {
    use super::rand::random;
    assert_eq!(Some(0), decode(encode_id(0).as_bytes()));
    for _ in 0 .. 1_000 {
        let i = random::<u64>();
        let enc = encode_id(i);
        let dec = decode_id(enc.as_bytes());
        assert_eq!(Some(i), dec);
    }
}
