# tokcount

Counts the tokens of a text under four frozen byte-level BPE
vocabularies and keeps the worst score. That score is what
`docs/spec_taille.md` compares with the ceiling on the size of the Shrimp
core specification.

One merge loop, written here, serves every vocabulary; only the data
differ. The vocabularies live in `vocab/` and never change. The tool is
Rust today, Shrimp once Shrimp exists.

    cargo run --release -- count docs/*.md         # one line per file, plus a total
    cargo run --release -- count --html page.html  # markup dropped first
    cargo run --release -- list

`measure.sh` replays the measurement of the reference specifications
(C23, C++23, Go, Zig, Rust) at pinned versions and hashes.

## Checked against the references

`check/compare.py` counts the same texts with tiktoken 0.14.0 and Hugging
Face tokenizers 0.23.3, loaded from the original files, and fails on a
single token of difference. On the five reference specifications and the
four vocabularies, twenty counts, zero difference (2026-10-21).

Two limits, both without effect on a specification text:

- the Hugging Face tokenizers match a few literal strings such as
  `<think>` or `<tool_call>` as single tokens; this tool does not;
- Anthropic's tokenizer is not public, so it is not among the four. The
  worst of four families stands in for it.
