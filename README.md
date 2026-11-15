# Shrimp

> **Under design — nothing is implemented, nothing is usable.** This
> repository is public to be read, not to be installed.

A programming language for two readers: the human who has the ideas
and reads, and the AI who writes. The compiler arbitrates.

**State: design. Nothing is implemented.** First stone laid on
20 October 2026, with no commitment on timing.

The ambition in one line: Rust's guarantees, much simpler to read.
*Jumbo shrimp*: a tiny core language, a huge library.

- `docs/requirements.md` — what Shrimp must be, and how it will be measured
- `docs/architecture.md` — frozen core language, three library tiers, incubator
- `docs/guarantees.md` — the five non-negotiable guarantees, and the candidates set aside
- `docs/budget.md` — the "every feature must pay its way" grid, the ceiling, the non-goals
- `docs/spec_size.md` — the token ceiling of the core language spec, and what C, C++, Go, Zig and Rust weigh
- `docs/state_of_the_art.md` — what is read before a line is written: eighteen languages against the five guarantees, a fixed grid, each cell re-read at the source
- `docs/name.md` — the name, and the ones set aside
- `docs/license.md` — why MIT or Apache-2.0, and not GPLv3
- `docs/tool.md` — the single tool `shrimp` and its eight verbs, `.shr`, running from source, the Unix tools inside, the mascot
- `docs/targets.md` — three levels of support, the targets, cross-compilation from the first day, the C boundary, `core` on an ESP32-C3
- `docs/bible.md` — the shape of the reference: three books, Markdown in a closed subset, examples compiled from the text, translation from English
- `docs/measurements.md` — how each figure is taken: reference programs, compilers, sessions, the corpus of refused programs, the check script
- `assets/` — the mascot, the logo and the icons, drawn by MaaaX
- `tools/tokcount/` — the token counter: four frozen vocabularies, one engine, the worst score
- `ci.sh` — the one check of the repository, run before each commit

## License

MIT or Apache-2.0, at your option: `LICENSE-MIT`, `LICENSE-APACHE`.
One license for everything, code and documents. Redistributed
third-party files: `THIRD_PARTY.md`.
