# Size of the core language specification — Shrimp

Decision of point 1.3 of the roadmap, 2026-10-21. The requirements
(§2, §8) demand that the core language specification fit in the context
window of an AI and leave the figure to be set; the budget
(`docs/budget.md`, §3) makes that figure the ceiling that refuses a
feature admitted by the grid. Here: what we count, with what, what the
reference languages weigh, and the ceiling. Everything quantified was
measured on the day of the decision, by a replayable tool that is in the
repository.

**Read and approved, Maxime MARCONATO (aka MaaaX^^, aka EPOCH84),
2026-10-21.** Any later change is dated and signed.

*Translated into English on 2026-10-22. The French text approved at commit
888832d remains the reference until this translation is re-read.*

Three trade-off decisions taken by MaaaX before writing, on 2026-10-21:
several open tokenizers and a single rewritten engine, worst score
retained; a fixed figure, not a fraction of a window that grows; the
whole of Book I counted, as the AI loads it.

## 1. What we count

**Book I of the Bible, whole, as the AI loads it**: normative text,
grammar, and the examples of the core language, since they are what the
AI reads the language with. In tokens, because that is the unit that
fills a context window: a word or a byte measures only a reflection of
it, and a text dense in symbols costs more tokens per word than prose.

Outside the ceiling:

- the edition appendices, what an edition masks and what the AI writing
  in the current edition does not have to load (`docs/budget.md`, §3);
- the index of `std`, which has its own measurement, set at step 4;
- the following books of the Bible, which describe the library and the
  tooling, not the language.

## 2. With what: four vocabularies, one engine, the worst score

A token is the unit of one specific model, and the tokenizer of
Anthropic's models is not public. Two ways out were bad: counting with a
single open vocabulary and guessing a margin, or making up a vocabulary
of our own, which would measure nothing. The way out retained:

- **four open vocabularies, from three publishers**, frozen in the
  repository with their hash and their license (`tools/tokcount/vocab/`):
  `o200k_base` and `cl100k_base` from OpenAI, that of Alibaba's Qwen3,
  that of DeepSeek-V3. They no longer move: the count of 2026 is redone
  identically in 2031, whatever happens to their publishers;
- **a single BPE engine, written in the repository** (`tools/tokcount/`,
  Rust today, Shrimp when Shrimp exists), which depends on no library
  from the publishers. Only the data changes from one vocabulary to the
  next: the merge table, the pre-tokenization, the normalization;
- **the score retained is the least favorable of the four.** It replaces
  the guessed margin with a measured margin: if four families of models
  fit, the fifth, unknown, has every chance of fitting. A new vocabulary
  gets added, the ceiling does not move.

**Verified, not assumed.** Each vocabulary was compared to its original
implementation, tiktoken 0.14.0 and tokenizers 0.23.3, on the five
reference specifications of §3: twenty counts, zero tokens of difference
(`tools/tokcount/check/compare.py`). A difference of one token would
have been a failure.

**What is not measured.** Anthropic's tokenizer, for lack of offline
access; the worst of the four stands in for it. The few strings that the
Qwen and DeepSeek vocabularies recognize as one block (`<think>`,
`<tool_call>`) are not recognized by the engine; a specification does
not contain them.

## 3. What the reference languages weigh

Measured on 2026-10-21 by `tools/tokcount/measure.sh`, sources pinned by
version and by hash, pdftotext 24.02.0 for the two PDFs, tags stripped
for the two HTML pages. The script replays everything.

| Specification | Source | o200k_base | cl100k_base | qwen3 | deepseek_v3 | **worst** |
|---|---|---:|---:|---:|---:|---:|
| Go 1.25 | `doc/go_spec.html` from the Go repository, tag `go1.25.0` | 55,389 | 55,279 | 56,240 | 56,519 | **56,519** |
| Zig 0.15.1 | language reference, single page, ziglang.org | 113,236 | 112,758 | 115,770 | 117,907 | **117,907** |
| Rust | *The Rust Reference*, repository `rust-lang/reference`, commit `3989fe9` (2026-10-20), `src/*.md` | 289,930 | 289,229 | 293,748 | 303,578 | **303,578** |
| C23 | working draft N3220, open-std.org, text of the PDF | 546,585 | 543,582 | 601,915 | 558,918 | **601,915** |
| C++23 | draft N4950 (2023-05-10), open-std.org, text of the PDF | 1,539,006 | 1,534,808 | 1,617,308 | 1,607,002 | **1,617,308** |

The two standards describe the language and the library in one piece.
Cut at the start of the "Library" clause, everything before it included
(foreword, table of contents):

| Language only | Clauses | o200k_base | cl100k_base | qwen3 | deepseek_v3 | **worst** |
|---|---|---:|---:|---:|---:|---:|
| C23 | 1 to 6 | 139,322 | 138,944 | 144,182 | 141,255 | **144,182** |
| C++23 | 1 to 15 | 373,149 | 373,314 | 380,613 | 380,718 | **380,718** |

What each text contains, to read the figures:

- **Go**: the language alone, without the library. It is the benchmark
  of concision: a complete language with garbage collector, interfaces
  and generics, in 56,000 tokens.
- **Zig**: the language with its compiled examples, numerous, the table
  of contents and the list of builtins. A large part of the text is
  code.
- **Rust**: the reference alone, without the *Book* or the documentation
  of `std`. It is not a complete specification, it says so itself.
- **C and C++**: the whole standard first, then the language clauses
  alone. The PDFs include index, annexes and footers, which pdftotext
  renders as they are.

The four vocabularies diverge from one another by 2% to 11% on the same
text; the worst is Qwen on the two PDFs, DeepSeek on the other three.
That is the gap the guessed margin would have had to cover.

**What this makes precise in the requirements.** §2 said, from memory:
"C fits, Go fits, Zig fits in one page; Rust does not, C++ even less."
Measured against a window of 200,000 tokens: Go, Zig and the language
clauses of C fit; the whole C standard, the Rust reference and C++ even
without its library do not fit. The intuition was right, the
requirements receive the figures (§2, dated).

## 4. The ceiling

The measurements above set the range: the complete specification of a
language without garbage collector, with grammar and examples, weighs
between Go and Rust. Shrimp removes from Rust half of what makes its
length, by decisions already taken: no macros, no `async`, no
inheritance, no destructor, no operator overloading, no inference, one
single way to do each thing. It adds what Rust does not have to specify:
linear resources, and examples at every entry.

**The ceiling is a fixed number, not a fraction of a window.** Windows
grow fast; a figure that passes in 2026 will pass with ease in a few
years, barring an industrial catastrophe (MaaaX, 2026-10-21). The figure
bounds the specification, not the model that reads it, and it must
leave, in a window of today, room for the program, the index of `std`
and the conversation.

**Ceiling: 100,000 tokens all included, on the worst of the four
vocabularies.** Decided by MaaaX on 2026-10-21, after measuring the Rust
reference stripped of what Shrimp refuses (inline assembly, macros,
code-generation and diagnostic attributes, destructors, coercions,
lifetime elision, glossary and index: 88,000 removed, 216,000 remaining):
"100,000 all included, or as close as possible". If the real text of
step 3 does not fit, the ceiling is revised as closely as possible by a
dated decision, never in passing.

- Twice Go, under Zig, under the language clauses of C, a third of the
  Rust reference. It is ambitious for a language that carries the
  guarantees G1 to G5, and it is deliberate: the ceiling is there to
  refuse.
- It leaves half of a 200,000-token window to the rest. Current windows
  range, from memory and without verification, from 128,000 to one
  million; the figure is designed for the smallest of the serious ones,
  not for the largest.
- It is measured on the real text at step 3, and it is revised by a
  decision of MaaaX, dated and motivated, never in passing
  (`docs/budget.md`, §3). The seven design documents of `docs/`, this
  one included, weigh 20,000 tokens on the worst vocabulary, to give a
  sense of scale.

Set aside: 60,000, the level of Go, which would not leave room for the
examples the Bible demands at every entry; 150,000, which passes in a
million but leaves 50,000 in 200,000, too little to write; the pair of a
150,000 ceiling and a 100,000 target, which a ceiling revisable by dated
decision makes useless.

## 5. A common trunk and sections: Book I as a set of skills

We do not need everything, all the time. A program that uses neither
`unsafe`, nor C interop, nor threads reads without their chapters. Book
I therefore takes the form of a **common trunk**, always loaded, and
**specialized sections**, loaded when the program needs them (MaaaX,
2026-10-21: "it looks like skills", and that is exactly it — a short
body always present, references opened on demand, one trigger per
reference).

Three rules, without which the form cheats:

1. **Each section is self-contained.** It does not change the meaning
   of what the trunk says: a program that does not use it reads and
   verifies with the trunk alone. That is G4, nothing hidden, applied to
   the specification itself.
2. **The trunk carries the index of the sections**, one line per section
   saying when to load it — "the program contains `unsafe` → section N".
   Without that line, the AI does not know what it is not loading.
3. **The ceiling stays on the whole**, trunk and sections. Otherwise it
   leaks: every costly feature would go into a section to escape it. The
   trunk receives its own figure, smaller, once the split exists.

Obvious candidates for a section: `unsafe`, C interop, concurrency. The
split, the list of sections and the figure for the trunk are set at
point 1.7, the form of the Bible; `tokcount` already counts per file and
in total, it has nothing to learn. The size of a typical session becomes
that of the trunk plus one or two sections, well under the ceiling.

## 6. Replaying

```
tools/tokcount/measure.sh                     # the five references, pinned sources
cargo run --release --manifest-path tools/tokcount/Cargo.toml -- count docs/*.md
```

The first needs `curl`, `pdftotext` and `cargo`; the second, `cargo`
alone, and it does not touch the network: the vocabularies are in the
repository. At step 3, it is the second that runs on Book I at every
version, and its result enters the Bible with the ceiling alongside.

## Sources

Re-read on 2026-10-21, SHA-256 hashes in `tools/tokcount/measure.sh`
and `tools/tokcount/vocab/README.md`.

- Go: https://raw.githubusercontent.com/golang/go/go1.25.0/doc/go_spec.html
- Zig: https://ziglang.org/documentation/0.15.1/
- Rust: https://github.com/rust-lang/reference, commit
  `3989fe91f1a601b51684a582ad17b2503f221787`
- C23: https://www.open-std.org/jtc1/sc22/wg14/www/docs/n3220.pdf
- C++23: https://www.open-std.org/jtc1/sc22/wg21/docs/papers/2023/n4950.pdf
- tiktoken 0.14.0, `tiktoken_ext/openai_public.py`, for the
  pre-tokenization patterns and the hashes of the two OpenAI vocabularies
- `tokenizer.json` of Qwen/Qwen3-8B and of deepseek-ai/DeepSeek-V3, for
  the merge tables, the patterns and the normalization
