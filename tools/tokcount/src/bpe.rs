//! Byte-level BPE, the merge loop shared by every vocabulary.
//!
//! A piece of text starts as one symbol per byte. At each step the
//! adjacent pair with the best rank is merged, leftmost first on a tie,
//! until no adjacent pair has a rank. The number of symbols left is the
//! token count. The two vocabulary families only differ in where the
//! rank of a pair comes from:
//!
//! - tiktoken files give a rank to every token, and the rank of a pair is
//!   the rank of the concatenated bytes, when that is a token;
//! - Hugging Face models give an ordered list of merges, and the rank of a
//!   pair is its position in that list.
//!
//! The merge loop is quadratic in the piece length. Pieces come from the
//! pre-tokenizer and are words, numbers or runs of punctuation, so it does
//! not matter.

use std::collections::HashMap;

pub enum Model {
    /// Token bytes to rank. A piece that is itself a token counts as one
    /// without going through the loop, as tiktoken does.
    Ranks(HashMap<Vec<u8>, u32>),
    /// Length-prefixed pair (`left.len()` as four little-endian bytes,
    /// then `left`, then `right`) to merge rank.
    Merges(HashMap<Vec<u8>, u32>),
}

impl Model {
    pub fn count_piece(&self, piece: &[u8]) -> usize {
        if piece.len() < 2 {
            return piece.len();
        }
        if let Model::Ranks(ranks) = self
            && ranks.contains_key(piece)
        {
            return 1;
        }
        let mut bounds: Vec<usize> = (0..=piece.len()).collect();
        let mut key = Vec::with_capacity(piece.len() + 4);
        loop {
            let mut best: Option<(u32, usize)> = None;
            for i in 0..bounds.len() - 2 {
                let left = &piece[bounds[i]..bounds[i + 1]];
                let right = &piece[bounds[i + 1]..bounds[i + 2]];
                if let Some(rank) = self.pair_rank(left, right, &mut key)
                    && best.is_none_or(|(r, _)| rank < r)
                {
                    best = Some((rank, i));
                }
            }
            match best {
                Some((_, i)) => {
                    bounds.remove(i + 1);
                }
                None => return bounds.len() - 1,
            }
        }
    }

    fn pair_rank(&self, left: &[u8], right: &[u8], key: &mut Vec<u8>) -> Option<u32> {
        key.clear();
        match self {
            Model::Ranks(ranks) => {
                key.extend_from_slice(left);
                key.extend_from_slice(right);
                ranks.get(key.as_slice()).copied()
            }
            Model::Merges(merges) => {
                key.extend_from_slice(&(left.len() as u32).to_le_bytes());
                key.extend_from_slice(left);
                key.extend_from_slice(right);
                merges.get(key.as_slice()).copied()
            }
        }
    }
}

pub fn merge_key(left: &[u8], right: &[u8]) -> Vec<u8> {
    let mut key = Vec::with_capacity(left.len() + right.len() + 4);
    key.extend_from_slice(&(left.len() as u32).to_le_bytes());
    key.extend_from_slice(left);
    key.extend_from_slice(right);
    key
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ranks(tokens: &[&str]) -> Model {
        Model::Ranks(
            tokens
                .iter()
                .enumerate()
                .map(|(i, t)| (t.as_bytes().to_vec(), i as u32))
                .collect(),
        )
    }

    #[test]
    fn whole_piece_is_one_token() {
        assert_eq!(ranks(&["hello"]).count_piece(b"hello"), 1);
    }

    #[test]
    fn merges_follow_rank_order() {
        // "ab" ranks before "bc": "abc" becomes "ab" + "c", never "a" + "bc".
        let model = ranks(&["ab", "bc", "abc"]);
        assert_eq!(model.count_piece(b"abc"), 1);
        assert_eq!(model.count_piece(b"abcd"), 2);
        assert_eq!(model.count_piece(b"xbc"), 2);
    }

    #[test]
    fn unmergeable_bytes_stay_separate() {
        assert_eq!(ranks(&[]).count_piece(b"abc"), 3);
        assert_eq!(ranks(&[]).count_piece(b""), 0);
    }

    #[test]
    fn merge_list_uses_pair_positions() {
        let mut merges = HashMap::new();
        merges.insert(merge_key(b"a", b"b"), 0);
        merges.insert(merge_key(b"ab", b"c"), 1);
        let model = Model::Merges(merges);
        assert_eq!(model.count_piece(b"abc"), 1);
        // "bc" is never merged on its own: no rule (b, c).
        assert_eq!(model.count_piece(b"bc"), 2);
    }
}
