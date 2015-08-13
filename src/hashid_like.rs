use std::io;

const ALPHABET: &'static[u8] =   b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

#[inline]
pub fn encode_wr<W:io::Write>(wr: &mut W, id: u64) -> io::Result<()> {
    let mut n = id;
    let alphasize = ALPHABET.len() as u64;
    loop {
        let rem = n % alphasize;
        n = n / alphasize;
        try!(wr.write_all(&ALPHABET[rem as usize .. rem as usize + 1]));
        if n == 0 { break; }
    }
    Ok(())
}

pub fn encode(id: u64) -> String {
    let mut v = Vec::new();
    encode_wr(&mut v, id).unwrap();
    String::from_utf8(v).unwrap()
}

pub fn decode(bytes: &[u8]) -> Option<u64> {
    let mut inv: Vec<i8> = (0..256).map(|_| -1).collect();
    for (i, &b) in ALPHABET.iter().enumerate() {
        inv[b as usize] = i as i8;
    }

    let mut failure = false;
    let n = {
        bytes.into_iter().rev().filter_map(|&b| {
            match inv.get(b as usize) {
                Some(&-1) | None => {failure = true; None},
                Some(&val) => Some(val as u8)
            }
        }).fold(0u64, |acc, item| acc * ALPHABET.len() as u64 + item as u64)
    };

    if failure { None }
    else { Some(n) }
}

#[test]
fn test_encode() {
    assert_eq!("c4NP", encode(12343344));
}

#[test]
fn test_decode() {
    assert_eq!(Some(12343344), decode(b"c4NP"));
}

#[test]
fn test_cyclic() {
    use super::rand::random;
    assert_eq!(Some(0), decode(encode(0).as_bytes()));
    for _ in 0 .. 1_000 {
        let i = random::<u64>();
        let enc = encode(i);
        let dec = decode(enc.as_bytes());
        assert_eq!(Some(i), dec);
    }
}
