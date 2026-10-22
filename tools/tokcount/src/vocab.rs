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
// /tools/tokcount/src/vocab.rs
// ================================================================
// The frozen vocabularies, and what each one needs around the
// merge loop: its pre-tokenizer, and whether it normalizes first.
// ================================================================
// Author: Maxime MARCONATO (aka MaaaX^^, aka EPOCH84)
// ================================================================
// License: MIT OR Apache-2.0, at your option — see LICENSE-MIT
// and LICENSE-APACHE.
// SPDX-License-Identifier: MIT OR Apache-2.0
// ================================================================

//! The frozen vocabularies and what each one needs around the merge loop:
//! its pre-tokenizer, and whether it normalizes the text first.
//!
//! The files under `vocab/` never change. The split patterns below are
//! copied verbatim from the reference implementations, because a single
//! character of difference changes the counts: tiktoken's
//! `openai_public.py` for the two OpenAI vocabularies, `tokenizer.json`
//! for the two Hugging Face ones. Each entry has been checked against its
//! reference implementation on the whole measurement corpus
//! (`check/compare.py`).

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use fancy_regex::Regex;
use unicode_normalization::UnicodeNormalization;

use crate::base64;
use crate::bpe::{self, Model};

pub enum Format {
    /// One `base64(token) rank` per line.
    TiktokenRanks,
    /// One `base64(left) base64(right)` per line, in merge order; `#`
    /// lines are comments.
    MergeList,
}

pub struct Spec {
    pub name: &'static str,
    pub file: &'static str,
    pub format: Format,
    /// The text goes through NFC before splitting, as the reference
    /// tokenizer does.
    pub nfc: bool,
    /// Split patterns applied in order. The pieces matched by one pattern
    /// are final; the gaps between them go to the next pattern. What no
    /// pattern matched is kept as pieces too.
    pub splits: &'static [&'static str],
    pub origin: &'static str,
}

pub const SPECS: &[Spec] = &[
    Spec {
        name: "o200k_base",
        file: "o200k_base.tiktoken",
        format: Format::TiktokenRanks,
        nfc: false,
        splits: &[concat!(
            r"[^\r\n\p{L}\p{N}]?[\p{Lu}\p{Lt}\p{Lm}\p{Lo}\p{M}]*[\p{Ll}\p{Lm}\p{Lo}\p{M}]+(?i:'s|'t|'re|'ve|'m|'ll|'d)?",
            r"|[^\r\n\p{L}\p{N}]?[\p{Lu}\p{Lt}\p{Lm}\p{Lo}\p{M}]+[\p{Ll}\p{Lm}\p{Lo}\p{M}]*(?i:'s|'t|'re|'ve|'m|'ll|'d)?",
            r"|\p{N}{1,3}",
            r"| ?[^\s\p{L}\p{N}]+[\r\n/]*",
            r"|\s*[\r\n]+",
            r"|\s+(?!\S)",
            r"|\s+"
        )],
        origin: "OpenAI, tiktoken 0.14.0, MIT",
    },
    Spec {
        name: "cl100k_base",
        file: "cl100k_base.tiktoken",
        format: Format::TiktokenRanks,
        nfc: false,
        splits: &[
            r"'(?i:[sdmt]|ll|ve|re)|[^\r\n\p{L}\p{N}]?+\p{L}++|\p{N}{1,3}+| ?[^\s\p{L}\p{N}]++[\r\n]*+|\s++$|\s*[\r\n]|\s+(?!\S)|\s",
        ],
        origin: "OpenAI, tiktoken 0.14.0, MIT",
    },
    Spec {
        name: "qwen3",
        file: "qwen3.merges",
        format: Format::MergeList,
        nfc: true,
        splits: &[
            r"(?i:'s|'t|'re|'ve|'m|'ll|'d)|[^\r\n\p{L}\p{N}]?\p{L}+|\p{N}| ?[^\s\p{L}\p{N}]+[\r\n]*|\s*[\r\n]+|\s+(?!\S)|\s+",
        ],
        origin: "Alibaba, Qwen/Qwen3-8B tokenizer.json, Apache-2.0",
    },
    Spec {
        name: "deepseek_v3",
        file: "deepseek_v3.merges",
        format: Format::MergeList,
        nfc: false,
        splits: &[
            r"\p{N}{1,3}",
            r"[\u{4e00}-\u{9fa5}\u{3040}-\u{309f}\u{30a0}-\u{30ff}]+",
            r##"[!"#$%&'()*+,\-./:;<=>?@\[\\\]^_`{|}~][A-Za-z]+|[^\r\n\p{L}\p{P}\p{S}]?[\p{L}\p{M}]+| ?[\p{P}\p{S}]+[\r\n]*|\s*[\r\n]+|\s+(?!\S)|\s+"##,
        ],
        origin: "DeepSeek, deepseek-ai/DeepSeek-V3 tokenizer.json, MIT",
    },
];

pub struct Vocab {
    pub name: &'static str,
    model: Model,
    nfc: bool,
    splits: Vec<Regex>,
}

impl Vocab {
    pub fn load(spec: &'static Spec, dir: &Path) -> Result<Vocab, String> {
        let path = dir.join(spec.file);
        let text = fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        let model = match spec.format {
            Format::TiktokenRanks => parse_ranks(&text),
            Format::MergeList => parse_merges(&text),
        }
        .map_err(|e| format!("{}: {e}", path.display()))?;
        let splits = spec
            .splits
            .iter()
            .map(|p| Regex::new(p).map_err(|e| format!("{}: split pattern: {e}", spec.name)))
            .collect::<Result<_, _>>()?;
        Ok(Vocab {
            name: spec.name,
            model,
            nfc: spec.nfc,
            splits,
        })
    }

    pub fn load_all(dir: &Path) -> Result<Vec<Vocab>, String> {
        SPECS.iter().map(|spec| Vocab::load(spec, dir)).collect()
    }

    pub fn count(&self, text: &str) -> Result<usize, String> {
        let normalized;
        let text = if self.nfc {
            normalized = text.nfc().collect::<String>();
            normalized.as_str()
        } else {
            text
        };
        let mut total = 0;
        for piece in self.pieces(text)? {
            total += self.model.count_piece(piece.as_bytes());
        }
        Ok(total)
    }

    fn pieces<'t>(&self, text: &'t str) -> Result<Vec<&'t str>, String> {
        let mut done: Vec<&'t str> = Vec::new();
        let mut pending: Vec<&'t str> = vec![text];
        for regex in &self.splits {
            let mut gaps: Vec<&'t str> = Vec::new();
            for segment in pending {
                let mut last = 0;
                for found in regex.find_iter(segment) {
                    let found = found.map_err(|e| format!("{}: {e}", self.name))?;
                    if found.start() > last {
                        gaps.push(&segment[last..found.start()]);
                    }
                    done.push(found.as_str());
                    last = found.end();
                }
                if last < segment.len() {
                    gaps.push(&segment[last..]);
                }
            }
            pending = gaps;
        }
        done.extend(pending);
        Ok(done)
    }
}

fn parse_ranks(text: &str) -> Result<Model, String> {
    let mut ranks = HashMap::new();
    for (n, line) in text.lines().enumerate() {
        let (token, rank) = line
            .split_once(' ')
            .ok_or_else(|| format!("line {}: expected `token rank`", n + 1))?;
        let rank: u32 = rank
            .parse()
            .map_err(|_| format!("line {}: bad rank", n + 1))?;
        ranks.insert(
            base64::decode(token).map_err(|e| format!("line {}: {e}", n + 1))?,
            rank,
        );
    }
    Ok(Model::Ranks(ranks))
}

fn parse_merges(text: &str) -> Result<Model, String> {
    let mut merges = HashMap::new();
    let mut rank = 0;
    for (n, line) in text.lines().enumerate() {
        if line.starts_with('#') || line.is_empty() {
            continue;
        }
        let (left, right) = line
            .split_once(' ')
            .ok_or_else(|| format!("line {}: expected `left right`", n + 1))?;
        let left = base64::decode(left).map_err(|e| format!("line {}: {e}", n + 1))?;
        let right = base64::decode(right).map_err(|e| format!("line {}: {e}", n + 1))?;
        merges.insert(bpe::merge_key(&left, &right), rank);
        rank += 1;
    }
    Ok(Model::Merges(merges))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vocab(splits: &'static [&'static str]) -> Vocab {
        Vocab {
            name: "test",
            model: Model::Ranks(HashMap::new()),
            nfc: false,
            splits: splits.iter().map(|p| Regex::new(p).unwrap()).collect(),
        }
    }

    #[test]
    fn sequential_splits_keep_earlier_matches_and_leftovers() {
        let v = vocab(&[r"\p{N}+", r"[a-z]+"]);
        assert_eq!(v.pieces("ab12cd-").unwrap(), vec!["12", "ab", "cd", "-"]);
    }

    #[test]
    fn every_split_pattern_compiles() {
        for spec in SPECS {
            for p in spec.splits {
                Regex::new(p).unwrap_or_else(|e| panic!("{}: {e}", spec.name));
            }
        }
    }
}
