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
// /tools/tokcount/src/main.rs
// ================================================================
// Token counter for the Shrimp specification: a text, several
// frozen vocabularies, and the worst score against the ceiling.
// ================================================================
// Author: Maxime MARCONATO (aka MaaaX^^, aka EPOCH84)
// ================================================================
// License: MIT OR Apache-2.0, at your option — see LICENSE-MIT
// and LICENSE-APACHE.
// SPDX-License-Identifier: MIT OR Apache-2.0
// ================================================================

//! Token counter for the Shrimp specification.
//!
//! Counts a text under several frozen byte-level BPE vocabularies and
//! reports each count with the worst of them, the figure that is compared
//! with the ceiling fixed in `docs/spec_taille.md`. The vocabularies live
//! under `vocab/` next to this crate and are never updated in place: a
//! count made today is reproducible as long as the repository exists.

mod base64;
mod bpe;
mod html;
mod vocab;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use vocab::Vocab;

const USAGE: &str = "\
usage:
  tokcount count [--html] FILE...     count tokens under every vocabulary
  tokcount strip-html FILE            print the text extracted from an HTML page
  tokcount convert-hf TOKENIZER.json  print a Hugging Face BPE model as a merge list
  tokcount list                       list the vocabularies and their origin

The vocabulary directory is `vocab/` next to the crate, or $TOKCOUNT_VOCAB_DIR.";

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    let result = match args.first().map(String::as_str) {
        Some("count") => count(&args[1..]),
        Some("strip-html") if args.len() == 2 => strip_html(&args[1]),
        Some("convert-hf") if args.len() == 2 => convert_hf(&args[1]),
        Some("list") => list(),
        _ => Err(USAGE.to_string()),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}

fn vocab_dir() -> PathBuf {
    env::var_os("TOKCOUNT_VOCAB_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("vocab"))
}

fn count(args: &[String]) -> Result<(), String> {
    let (strip, files): (Vec<_>, Vec<_>) = args.iter().partition(|a| *a == "--html");
    let strip = !strip.is_empty();
    if files.is_empty() {
        return Err(USAGE.to_string());
    }
    let vocabs = Vocab::load_all(&vocab_dir())?;

    let mut rows: Vec<(String, Vec<usize>)> = Vec::new();
    for file in files {
        let text = read_text(file)?;
        let text = if strip { html::strip(&text) } else { text };
        let counts = vocabs
            .iter()
            .map(|v| v.count(&text))
            .collect::<Result<Vec<_>, _>>()?;
        rows.push((file.clone(), counts));
    }
    if rows.len() > 1 {
        let totals = (0..vocabs.len())
            .map(|i| rows.iter().map(|(_, c)| c[i]).sum())
            .collect();
        rows.push(("total".to_string(), totals));
    }

    print!("| text |");
    for v in &vocabs {
        print!(" {} |", v.name);
    }
    println!(" worst |");
    print!("|---|");
    for _ in &vocabs {
        print!("---:|");
    }
    println!("---:|");
    for (name, counts) in &rows {
        print!("| {name} |");
        for c in counts {
            print!(" {c} |");
        }
        println!(" {} |", counts.iter().max().copied().unwrap_or(0));
    }
    Ok(())
}

fn strip_html(file: &str) -> Result<(), String> {
    print!("{}", html::strip(&read_text(file)?));
    Ok(())
}

fn list() -> Result<(), String> {
    for spec in vocab::SPECS {
        println!("{:<12} {:<24} {}", spec.name, spec.file, spec.origin);
    }
    Ok(())
}

fn read_text(file: &str) -> Result<String, String> {
    fs::read_to_string(file).map_err(|e| format!("{file}: {e}"))
}

/// Turns a Hugging Face `tokenizer.json` into the merge list this tool
/// loads: one `base64(left) base64(right)` line per merge, in order. Only
/// plain byte-level BPE is accepted. Token strings in such files map each
/// byte to a printable character (the GPT-2 table); that mapping is
/// undone here so the merges operate on bytes like everything else.
///
/// The pre-tokenizer patterns are not converted: they go verbatim into
/// `vocab::SPECS`, and the converter prints them for that purpose.
fn convert_hf(file: &str) -> Result<(), String> {
    let json: serde_json::Value =
        serde_json::from_str(&read_text(file)?).map_err(|e| format!("{file}: {e}"))?;
    let model = &json["model"];
    if model["type"] != "BPE" {
        return Err(format!("{file}: model type is not BPE"));
    }
    for (key, forbidden) in [
        ("byte_fallback", true),
        ("fuse_unk", true),
        ("ignore_merges", true),
    ] {
        if model[key] == forbidden {
            return Err(format!("{file}: {key} = {forbidden} is not supported"));
        }
    }
    for key in ["continuing_subword_prefix", "end_of_word_suffix", "dropout"] {
        if !(model[key].is_null() || model[key] == "") {
            return Err(format!("{file}: {key} is not supported"));
        }
    }
    let Some(merges) = model["merges"].as_array() else {
        return Err(format!("{file}: no merges"));
    };
    let byte_of = gpt2_byte_table();
    let unmap = |token: &str| -> Result<Vec<u8>, String> {
        token
            .chars()
            .map(|c| {
                byte_of
                    .get(&c)
                    .copied()
                    .ok_or_else(|| format!("{file}: {c:?} is not in the byte table"))
            })
            .collect()
    };

    for (n, merge) in merges.iter().enumerate() {
        let (left, right) = match merge {
            serde_json::Value::String(s) => s
                .split_once(' ')
                .ok_or_else(|| format!("{file}: merge {n} is not `left right`"))?,
            serde_json::Value::Array(pair) if pair.len() == 2 => {
                match (pair[0].as_str(), pair[1].as_str()) {
                    (Some(l), Some(r)) => (l, r),
                    _ => return Err(format!("{file}: merge {n} is not a pair of strings")),
                }
            }
            _ => return Err(format!("{file}: merge {n} has an unknown shape")),
        };
        println!(
            "{} {}",
            base64::encode(&unmap(left)?),
            base64::encode(&unmap(right)?)
        );
    }

    eprintln!("{} merges written", merges.len());
    eprintln!("normalizer: {}", json["normalizer"]);
    eprintln!("pre_tokenizer: {}", json["pre_tokenizer"]);
    let added: Vec<&str> = json["added_tokens"]
        .as_array()
        .map(|a| {
            a.iter()
                .filter(|t| t["special"] == false)
                .filter_map(|t| t["content"].as_str())
                .collect()
        })
        .unwrap_or_default();
    eprintln!(
        "non-special added tokens, matched literally by the reference and not here: {added:?}"
    );
    Ok(())
}

/// The GPT-2 mapping from bytes to printable characters, inverted.
/// Printable ASCII and Latin-1 letters map to themselves; the other 68
/// byte values map to U+0100 onwards, in increasing order.
fn gpt2_byte_table() -> std::collections::HashMap<char, u8> {
    let mut table = std::collections::HashMap::new();
    let mut next = 0x100u32;
    for byte in 0..=255u8 {
        let printable =
            (b'!'..=b'~').contains(&byte) || (0xa1..=0xac).contains(&byte) || byte >= 0xae;
        let c = if printable {
            char::from(byte)
        } else {
            next += 1;
            char::from_u32(next - 1).expect("below the surrogate range")
        };
        table.insert(c, byte);
    }
    table
}
