// ================================================================
//        _____ __         _
//       / ___// /_  _____(_)___ ___  ____
//       \__ \/ __ \/ ___/ / __ `__ \/ __ \
//      ___/ / / / / /  / / / / / / / /_/ /
//     /____/_/ /_/_/  /_/_/ /_/ /_/ .___/
//                                /_/
//
// Shrimp — Small core, jumbo library.
// A programming language for two readers:
//      the human who reads, the AI who writes.
// ================================================================
// /tools/tokcount/src/base64.rs
// ================================================================
// Standard base64 with padding, the encoding tiktoken files use
// for token bytes. Forty lines instead of a pinned dependency.
// ================================================================
// Author: Maxime MARCONATO (aka MaaaX^^, aka EPOCH84)
// ================================================================
// License: MIT OR Apache-2.0, at your option — see LICENSE-MIT
// and LICENSE-APACHE.
// SPDX-License-Identifier: MIT OR Apache-2.0
// ================================================================

//! Standard base64 with padding, the encoding tiktoken files use for
//! token bytes. Written here rather than pulled in: forty lines against a
//! dependency that would have to be pinned for as long as the vocabularies.

const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn encode(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let mut buf = [0u8; 3];
        buf[..chunk.len()].copy_from_slice(chunk);
        let n = u32::from(buf[0]) << 16 | u32::from(buf[1]) << 8 | u32::from(buf[2]);
        let sextets = [n >> 18, n >> 12, n >> 6, n];
        for (i, s) in sextets.iter().enumerate() {
            if i <= chunk.len() {
                out.push(ALPHABET[(s & 0x3f) as usize] as char);
            } else {
                out.push('=');
            }
        }
    }
    out
}

pub fn decode(text: &str) -> Result<Vec<u8>, String> {
    let digits = text.trim_end_matches('=').as_bytes();
    let mut out = Vec::with_capacity(digits.len() * 3 / 4);
    let mut acc: u32 = 0;
    let mut bits = 0;
    for &d in digits {
        let value = ALPHABET
            .iter()
            .position(|&a| a == d)
            .ok_or_else(|| format!("invalid base64 digit {:?}", d as char))?;
        acc = acc << 6 | value as u32;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((acc >> bits) as u8);
            acc &= (1 << bits) - 1;
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_every_length() {
        for len in 0..10 {
            let bytes: Vec<u8> = (0..len)
                .map(|i| (i as u8).wrapping_mul(37).wrapping_add(200))
                .collect();
            assert_eq!(decode(&encode(&bytes)).unwrap(), bytes);
        }
    }

    #[test]
    fn matches_known_values() {
        assert_eq!(encode(b"IQ"), "SVE=");
        assert_eq!(decode("IQ==").unwrap(), b"!");
        assert_eq!(decode("4pyT").unwrap(), "✓".as_bytes());
    }
}
