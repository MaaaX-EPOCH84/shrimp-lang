# Requirements — Shrimp

What Shrimp must be, and how we will know that it is. Laid down on
2026-10-20, before any line of specification or code. The decisions
are in `AGENTS.md`; here is the substance.

**Read and approved, Maxime MARCONATO (aka MaaaX^^, aka EPOCH84), 2026-10-21.** Any later change
is dated and signed the same way.

*Translated into English on 2026-10-22. The French text approved at commit
888832d remains the reference until this translation is re-read.*

## 1. Thesis

The future of the programming interface is natural language, whatever
the language: to the human the ideas, to the AI the programming. The
artifact, for its part, stays formal. Running natural language has
already been tried — COBOL in 1959, SQL afterwards — and precision did
all the work, English none of it. So there must always be a formal layer
under the layer of intent: it is what runs, it is what gets verified, it
is the contract between the idea and the machine, and between today's AI
and tomorrow's.

Consequence: when the AI writes, the human's role shifts from writing to
reading. **Shrimp is optimized for reading and verification, not for
writing speed.** This is the reverse of forty years of language design,
which chased concision for the one who types.

## 2. The two readers

### The human

- Readable by someone who did not write the code, and who takes it back
  at the keyboard when the AI is down — "when the computer is down, there
  is still the ballpoint pen": the code must read well on paper.
- **Local**: a function is understood by reading the function. Nothing
  comes from elsewhere without being named — no hidden control flow, no
  hidden allocation, no implicit conversion, no macro that rewrites the
  syntax, no surprising operator overloading.
- **Small**: the language fits in one head. One way to do each thing.
- **Explicit rather than clever**: ownership is read in the signature;
  an error is a value; the forgotten case does not compile.
- Keywords and conventions readable without a dictionary — the line
  between that and the verbose (Ada) is to be found through use.

### The AI

What the AI asks for, phrased by the AI itself:

- **A large, stable corpus.** It programs well where it has read a lot
  and where nothing has moved. A new language has no corpus: two
  countermeasures, a syntax close to what it knows (Rust) so that what it
  has learned transfers, and a specification short enough to be re-read
  at every session.
- **A compiler that contradicts it.** Its mistakes are confident; the
  static checker is its first reviewer, before the human. Error messages
  are designed for an AI to act on: precise, localized, with the possible
  fix and its cost.
- **Nothing implicit** — same reasons as for the human: it gets lost in
  what it cannot read locally.
- **No undefined behavior**: it debugs what is not reproducible no better
  than the human does. Deterministic builds.
- **The specification of the core language fits in its context window.**
  A new criterion, specific to this era: if it re-reads the whole
  language reference at every session, it no longer gets the language
  wrong, only the problem. C fits, Go fits, Zig fits in one page; Rust
  does not, C++ even less. Measured in tokens, target to be set.
  *Measured on 2026-10-21, point 1.3, `docs/spec_size.md`:* against
  200,000 tokens, Go (56,519), Zig (117,907) and the language clauses of
  C (144,182) fit; the Rust reference (303,578) and C++ without its
  library (380,718) do not. Shrimp's ceiling: 100,000 all included.
- **The `std` reference readable by machine**: one signature, one line,
  one theme per entry, to verify instead of guessing.

What the AI is not: infallible. Faster than most humans at writing
everyday code; no better than the best at architecture, at knowing what
not to build, or at a race through five hundred thousand lines. The
safeguard is precisely that the human can read it over. Hence everything
above.

## 3. Ambition

**As reliable as Rust, much simpler to read.**

"Infinitely simpler" does not exist: Rust's borrow checker is the price
of "memory safety, without a garbage collector, without runtime checks".
Removing one of the three terms simplifies everything — Go pays with a
garbage collector, Zig pays with runtime checks and discipline, Ada/SPARK
pays with verbosity. Keeping all three and throwing away the complexity
is an open problem (Hylo attempts it through mutable value semantics).

The reversal that makes the ambition tenable: **Rust's difficulty is in
the writing, not in the reading.** Fighting the borrow checker falls to
the AI. What remains on the human's desk — heavy generics, async,
procedural macros — is an avoidable subset. Shrimp therefore starts from
Rust's semantics and attacks its three painful spots: the incompleteness
of the checker, compile times, the complexity budget. Target, phrased as
a bet: the guarantees of Rust, the simplicity of Zig, the compile times
of OCaml.

## 4. Faster than C

Possible, and already done by others, because speed comes from what the
compiler is allowed to assume: Fortran beats C on numerical work because
its arrays do not alias; an exclusive `&mut` in Rust is worth `restrict`
everywhere; a generic `sort` beats `qsort` because the comparator is
inlined; Halide and ISPC beat hand-written C because they express
parallelism. **The rigor that makes a language reliable is the same
rigor that makes it fast; undefined behavior is the enemy of both.**
Ceiling: nothing beats the best C with intrinsics; everything can beat
the C that actually gets written.

## 5. Candidate guarantees

Decided on 2026-10-21 in `docs/guarantees.md`: five retained on a
criterion, not on a number. Any feature that contradicts one of them is
refused, even if it is "the best" elsewhere. The table below is the
starting material, kept as it was.

| Candidate | Who has it | What it costs |
|---|---|---|
| No undefined behavior in safe code | Rust | An `unsafe` sublanguage to fence off |
| Memory safety without a garbage collector | Rust, Austral, Hylo | An ownership model, hence correct programs refused |
| No data races | Rust (`Send`/`Sync`), Erlang (by construction) | A sharing model in the language |
| Nothing hidden: no control flow, no allocation, no conversion | Zig | No exceptions, no macros, no free overloading |
| Errors as values, exhaustive cases | Rust, Zig | No exceptions |
| Compatibility: what compiles today compiles tomorrow | Go (since 2012), C | Narrow front door, editions |
| Specification bounded in size | Scheme, Go | Refusing useful features |
| Zero-cost abstraction | Rust, C++ | Monomorphization, hence compile time |

Not all of them combine: the choice is made, see `docs/guarantees.md`.

## 6. Non-goals

- Graphical user interface in `std` (Swing, `tkinter`: we know what that
  becomes). *Amended on 2026-10-21, point 1.2, MaaaX:* GUIs live in
  `x/`, as safe C wrappers per system, with no common abstraction;
  detail in `docs/budget.md`, §5.
- Competing with Rust's ecosystem: Shrimp speaks C at the boundary and
  can call what exists.
- Adoption by the world as a success criterion: a language for two,
  first. What kills personal languages — no ecosystem, nobody knows it —
  no longer holds when the AI regenerates the libraries from the spec
  and the spec fits in its context. Documentation (§9) prepares adoption
  without depending on it.
- Syntactic novelty: syntax is the least important and most disputed
  part; we keep that of others.
- "The best of every language": features do not add up, they interact
  (exceptions and ownership, garbage collector and destructors, macros
  and readability, inheritance and value semantics).

## 7. Constraints

- **C interoperability** mandatory, in both directions: every language
  must speak C.
- **Targets**, in order: Linux, then Windows, then macOS, then mobile,
  then the others — Raspberry Pi, ESP32, STM32 and the like, where
  `core` runs without an OS or allocation. Multi-system and
  multi-processor are goals; multi-generation (Z80, 6502, 68000) is a
  wish, kept if it costs the language no constraint, dropped otherwise.
  The detail — support levels, cross-compilation, WebAssembly, BSD — is
  set at point 1.6 of the roadmap.
- **A single tool**, a single chain, no choice of runner or formatter
  left to the user.

## 8. Measurements

Nothing to solder, but everything to measure. One number per
requirement, otherwise the requirement is an opinion.

| Requirement | Measurement | Reference |
|---|---|---|
| Short spec | Tokens of the core language spec, at the worst of four frozen vocabularies (`tools/tokcount`) | 100,000 all included, set on 2026-10-21 (`docs/spec_size.md`); Go 56,519, Zig 117,907, Rust 303,578 |
| Fast to compile | Seconds for a reference program, cold and incremental | Rust, Go, OCaml on the same program |
| Fast code | Execution time of the same programs | C (`-O2`), Rust (`--release`) |
| Checker not too strict | Correct programs refused on a given corpus | Rust on the same corpus |
| Readable | A human who did not write the code explains one of its functions unaided | MaaaX, on code written by the AI |
| Stable | Programs of the corpus that still compile after each release | 100% after 1.0 |
| Nothing hidden | Allocations and calls not visible in the source | Zero |

## 9. Documentation

Added on 2026-10-21. A language is documented the way it is specified:
in one block, and in a single source language.

- **A Bible**, the complete reference of the language — core language,
  `core`, `std`, tooling —, written in English, like everything that
  belongs to the language. Delivered online and as PDF, both generated
  from the same source.
- **Translated into the major languages**, from English, to ease
  adoption; the English version is authoritative.
- **Every entry of `core` and `std` carries a real, practical example**,
  commented didactically: not an isolated call, a program one could
  actually have written, which shows the entry in use. The examples
  compile and are tested at every release: an example that no longer
  compiles is a defect in the library.
- **The machine-readable index** (§2) is generated from the same sources
  as the Bible; nothing is written twice.

## 10. Editor tooling

Added on 2026-10-21 (MaaaX). Reading code also means reading it in an
editor.

- **An extension for VS Code and VSCodium**, the same one, published on
  both marketplaces (Marketplace and Open VSX), shipped with the language
  and not left to the community.
- **Syntax highlighting designed for reading**, not for decoration: what
  demands the reviewer's attention must leap to the eye. First, **the
  `unsafe` blocks**, which go unnoticed in the mass of code and are
  precisely the place where the guarantees stop: distinct background,
  border, or any other signal visible across a whole page. Then the
  types to consume and their consumption, the named allocations, the
  calls at the C boundary.
- **The highlighting comes from the compiler**, through a language
  server, to color what the code *is* and not what it looks like: a
  surface grammar does not know that a value is a resource.
- The same requirement for the printed version and the PDF of the Bible:
  what is flagged on screen is flagged on paper.
