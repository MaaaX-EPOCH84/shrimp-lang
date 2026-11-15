# The measurement protocol — Shrimp

Decision of point 1.8 of the roadmap, 2026-11-14. Requirements §8 gives
one measurement per requirement and says nothing of how it is taken.
Here: what a figure is, on which programs, against which compilers, on
which machine, by which script — for each line of that table, and for
the three measurements the earlier points sent here (the latency of
`shrimp run`, the size of `core` on an ESP32-C3, the check script).
Eight questions settled by MaaaX on 2026-11-14, on the assistant's
recommendations, in one answer. Nothing of Shrimp can be measured yet:
there is no compiler. What could be tried was tried on C, Rust, Go and
Python, on the machine at hand (§13); the facts about other tools were
re-read at the source the same day (§14).

**Read and approved, Maxime MARCONATO (aka MaaaX^^, aka EPOCH84),
2026-11-14.** Any later change is dated and signed.

## 1. The lines of requirements §8, and two more

| Requirement | What is measured | Against | First figure |
|---|---|---|---|
| Short spec | Tokens of Book I, worst of four frozen vocabularies | The ceiling of 100,000 (`docs/spec_size.md`) | Step 3 |
| Fast to compile | Seconds, cold and incremental, on a generated program of three sizes and on one real program (§5) | Rust, Go, OCaml | Step 7 |
| Fast to run from source | Milliseconds from the command to the first byte of output, source cached and source modified (§5) | `go run`, `python3` | Step 7 |
| Fast code | Time and instruction count of six kernels and one real program (§3, §4) | C at `-O2` under two compilers, Rust at `--release` | Step 7 |
| Checker not too strict | Correct programs refused, on a corpus that only grows (§6) | Rust on the same corpus | Steps 7 and 12 |
| Readable | Outputs predicted right by a reader with no tool (§7) | No other language | Step 12 |
| Nothing hidden | Calls and allocations of the generated code that no word of the source carries (§8) | Zero | Step 7 |
| Stable | Programs that still compile after each release (§9) | 100% after 1.0 | Step 7 |
| `core` fits | Bytes of flash of two programs on an ESP32-C3 (§9) | C and Rust | Step 8 |

**No threshold is set here.** The references of requirements §8 stand
alone until Shrimp has figures of its own: the first ones come with the
prototype (step 7), and a threshold enters by a dated decision at the
audit of step 11, once there is something to hold it against. A
threshold written today would be the guessed margin refused at point
1.3.

## 2. What a figure is

**A figure is a ratio, never a duration.** Shrimp over C, Shrimp over
Rust: the two programs measured in the same session, on the same
machine. There is no reference machine. Seconds taken on one machine
are never set beside seconds taken on another, and a duration appears
in a result only next to the one it is divided by.

**A session**, for timed measurements:

1. every program is built, run once, and **its output compared byte for
   byte** with the others; a program whose output differs is not timed;
2. **eleven passes, interleaved**: one run of each program, then the
   next pass, so that a drift of the machine falls on all of them;
3. the figure of a program is the **median** of its eleven passes;
4. **the session is run twice**; it is valid if, for every program, the
   two medians differ by less than 2%. Otherwise both are thrown away
   and the machine is looked at.

The 2% is not guessed: on the machine at hand, a laptop, three sessions
gave medians within 1% of each other while single passes spread over
4 to 10% (§13). The median holds where one pass does not.

**The instruction count is the second figure**, beside the time: the
instructions executed in user mode, counted by `perf stat`. It is
expected to move far less than time from one run to the next, and so to
show a change of one percent that time cannot. **Expected, not
verified**: on the machine at hand the counter is refused to an
ordinary user (§13). Its spread is the first thing measured when it is
opened, and it stays a second figure until then.

**What a result records**, in a text file beside the figures: the date
and the commit; the processor, the number of cores, the memory, the
kernel, the frequency governor; the exact version of every compiler and
the flags it was given; the medians, the two sessions, the ratios. A
figure whose file lacks one of these is not a figure.

**Remote runners check, they do not measure.** A shared machine gives
durations nobody can reproduce. The check script (§10) verifies there
that the reference programs build and print the same output; the
figures that are published come from a session.

## 3. The reference programs

Six kernels written by the project, and one real program.

| Kernel | What it does | What it isolates (requirements §4) |
|---|---|---|
| `axpy` | Adds a scaled array to another, repeatedly, through a function that receives both | What the compiler may assume of two arrays that cannot alias |
| `sort` | Sorts integers with one sort written out, generic over the comparison | A comparison the compiler can see and inline, against one passed by address |
| `words` | Counts the distinct words of a generated text in an open-addressing table written out | Hashing, byte strings, the cost of checked indexing |
| `tree` | Builds, walks and gives back a binary tree, node by node | Allocation, and memory given back at the closing brace without a destructor |
| `parse` | Reads decimal numbers and separators out of a byte buffer | Branches and bounds on bytes |
| `bytes` | A 16-bit checksum and a saturating sum over `u8` values | Small integers with no silent promotion; needs no allocation, hence `core` alone |

**The real program is the BPE engine of `tools/tokcount`**: it exists,
it does a real job on real data — a frozen vocabulary, a text —, and
it is already due to be rewritten in Shrimp (point 1.3 of the
roadmap). It is compared with its Rust original only: writing it a
third time in C would be a project of its own.

**How they are written**, in every language:

- **as the language is actually written**: no intrinsics, no inline
  assembly, no compiler-specific attribute or pragma, no `unsafe` in
  Rust, no `restrict` in C;
- **the same algorithm, written out**: a kernel calls no library
  collection and no library sort — the library is measured by the real
  program, the language by the kernels;
- **one thread**;
- **the input is generated inside the program**, by the same generator
  with the same seed in every language, and the program prints one line
  — a checksum of its result. That line is what §2 compares;
- **sized to run for about a second** in C on the machine of the
  session: long enough for the start of the process, about a
  millisecond (§13), to vanish.

One exception, said: **`axpy` has a second C version, with `restrict`**,
reported beside the first. The first is the C that gets written; the
second is the ceiling requirements §4 names — "nothing beats the best
C" — and the distance between the two is what the kernel is there to
show.

The kernels are specified here and written at step 7, with the
prototype that will compile the Shrimp versions. Their sizes, the
generator and its seed are fixed with the code.

**Kernels are stand-ins, and known as such.** The Benchmarks Game
itself quotes the objection — kernels, toy programs and synthetic
benchmarks "are discredited today", because a compiler can be made to
shine on them (§14). They are kept for what they are good at: each one
isolates one assumption, is read in a minute, and is the same program
in every language. The real program stands beside them, and the
programs of the trial (step 12) take over as soon as they exist.

## 4. Fast code

Requirements §8 names the references: C at `-O2`, Rust at `--release`.
They are kept as written, with one addition: **the C versions are
compiled by two compilers, GCC and Clang.** Rust comes out of LLVM, as
Clang does; against GCC alone, the figure would measure the backend as
much as the language. The witness of §13 shows it on one kernel: the
same C source runs in 0.79 s under GCC and 0.51 s under Clang, and the
Rust version ties with the second.

Nothing else is passed: no `-march=native`, no link-time optimisation,
no profile, no fast-math. Checks stay on wherever the safe language
puts them. The asymmetry of requirements §8 is kept and said: `-O2` for
C, and what `--release` means for Rust, which is its level 3 (§14).

Three ratios per program: Shrimp over C under GCC, over C under Clang,
over Rust — in time and in instructions.

## 5. Fast to compile, and to run from source

**A real program is too small to time a compiler.** The whole of
`tools/tokcount` is 924 lines of Rust; compiling it measures the start
of the compiler. Hence two programs:

- **a generated program, at three sizes** — 1,000, 10,000 and 100,000
  lines. A generator written by the project emits, from one seed, the
  same program in each language: functions of a few fixed shapes that
  call each other. Three sizes give a slope, lines per second, and show
  whether the time grows in a straight line; one size would give a
  point;
- **the real program** of §3, against Rust only, for the same reason
  as there.

| State | What it means |
|---|---|
| Cold | The build cache is empty; the compiler itself has already run once, and that first run is not counted |
| Incremental, body | The body of one function has changed; nothing that depends on its signature has |
| Incremental, signature | The signature of one function has changed, and its callers with it |

Each state in each mode the language has: the mode a developer loops in
and the optimised one — `shrimp check` against `cargo check`, a plain
build against `cargo build`, `go build` and `ocamlopt`, an optimised
build against `cargo build --release`. Compile times are durations of
one to a hundred seconds: the session of §2 applies, with fewer passes
where a pass takes a minute.

**The latency of `shrimp run`** (`docs/tool.md` §4) is the time from
the command to the first byte the program writes. Two programs — one
line of output, and the real program — in two states: the source
unchanged since the last run, and one character of it changed. Two
references, the two ways a script is run from source today: `go run`,
which compiles, and `python3`, which interprets; and a floor, the same
one-line program compiled in C. On the machine at hand, for one line of
output (§13):

| | Unchanged | Modified |
|---|---:|---:|
| `go run` 1.22.2 | 157 ms | 167 ms |
| `python3` 3.12.3 | 15 ms | 15 ms |
| C, already compiled | 0.9 ms | — |

## 6. Checker not too strict: the corpus

**What is counted**: the programs of a corpus that are correct and that
the compiler refuses. Shrimp's count beside Rust's, on the same corpus.

**An entry** is one program, and holds:

1. one sentence saying what the program does, and the name of the
   pattern it carries;
2. a fixed input and the output expected;
3. **the natural form in C**, which prints that output when compiled
   with the address and undefined-behaviour sanitizers;
4. **the natural form in Rust** — the same algorithm in safe code, with
   no `unsafe`, no copy added and no restructuring to please the
   checker — and the verdict of a `rustc` whose version the result
   records: accepted, or refused with the error code;
5. from step 7, the natural form in Shrimp and the verdict of `shrimp`.

**"Correct" means what item 3 can show**: the algorithm, run on the
input, has no undefined behaviour that the sanitizers see and prints
the expected output. It is a test, not a proof, and it is said.

**Admission**:

- the C form passes item 3;
- **one entry per pattern**: a second program that is refused for the
  same reason as an existing one does not enter. Without that rule the
  count is whatever the author of the corpus wants it to be;
- an entry enters because **some** checker refuses it or is expected
  to: Rust's, the linear types of Austral, Shrimp's own. The corpus is
  seeded from the cases the Rust project documents itself, so Rust's
  count starts high by construction: **the count compares two checkers
  on one corpus, it ranks nothing in the absolute**;
- **every correct program that Shrimp refuses during the trial**
  (step 12) enters, whether Rust accepts it or not. That is what keeps
  the corpus from flattering its author.

**The corpus only grows.** An entry is never removed and never
rewritten; a checker that improves changes a verdict, not the corpus.
A form that works around the refusal may be kept beside the natural
one, for what it costs to read; it is not counted.

The rule is fixed here. The first entries are written at step 2, where
each guarantee is read with its cost; Shrimp is counted at steps 7 and
12. One entry was built as a witness (§13).

## 7. Readable

Requirements §8: "a human who did not write the code explains one of
its functions unaided". Made mechanical, at step 12:

1. **the functions are drawn by a script**, from one seed, among the
   functions of the trial programs that have a test. Neither the AI nor
   the reader chooses them;
2. for each one, the reader receives **the text of the function, the
   signatures of what it names** — G4 promises they are enough
   (`docs/guarantees.md` §2) — **and one input**: that of the first test
   of the function, in the order of the file;
3. **the reader is MaaaX** (`docs/budget.md` §4), who did not write the
   code. Plain text, no editor help, no execution, no AI: the ballpoint
   pen of requirements §2;
4. he writes **the output for that input**. The program is then run:
   the answer is right or wrong, and nobody judges it;
5. **the figure is the number of right answers over the number of
   functions**, with the time spent on each noted beside.

How many functions are drawn is fixed at step 12, and not below ten.

**No other language is put beside.** The same functions in Rust or C
would measure what the reader already knows of Rust and C. The figure
is read alone, from one trial to the next.

## 8. Nothing hidden

Requirements §8: "allocations and calls not visible in the source",
reference zero. G4 says what visible means (`docs/guarantees.md` §2).

**The compiler gives the list.** For a program, on request: every call
and every allocation in the code it generated, each with the word of
the source that carries it — the position of a call, an operator, an
index. **The figure is the number of entries with no word.** The form
of the request is fixed at step 7, with the compiler.

Three kinds of entry need a rule, or zero is out of reach on some
targets and meaningless on the others:

| Entry | Rule |
|---|---|
| Memory given back at a closing brace | Listed apart, with the brace. It is the one implicit rule of G4; the list shows it is the only one |
| A routine the target needs for an operator — a division on a processor without a divider, floating point on one without a floating-point unit, which is the case of the ESP32-C3 | Carried by the operator written in the source. The routines form a closed list in the specification; a call to anything outside it has no word |
| The stop of a runtime check — an index out of bounds, an overflow | Carried by the index or the operator that requires the check (`docs/guarantees.md` §4, G1 × G4) |

The cost of these entries is not hidden by being carried: it shows in
the instruction count of §4, which is where G1 × G4 sent it.

## 9. Stable, and the size of `core`

**Stable.** The corpus is everything that compiles: the examples of the
Bible, the reference programs of §3, the corpus of §6, the programs of
the trial. At each release every one of them is compiled again; a block
labelled as rejected must still be rejected, for the same rule
(`docs/bible.md` §3). After 1.0 the figure is 100% or the release does
not go out. Before 1.0 a program may stop compiling, and each one that
does is listed with the date and the reason: the list is the price of
every change of mind, kept where it can be counted.

**The size of `core`** (`docs/targets.md` §5). Two programs built for
the ESP32-C3: the empty one, and the kernel `bytes` of §3, which needs
`core` alone. The figure is the flash they occupy — code and
initialised data of the linked file, read with `size` — beside the same
two programs in C at `-Os` and in Rust at `opt-level = "s"`. No board
is needed: nothing is run. First figure at step 8, when `core` exists
in Shrimp.

## 10. The check script

`docs/targets.md` §1 asks for one script that is the only description
of the check. It is **`ci.sh`, at the root**, and it exists from this
point on, with what can already be checked:

| Step | What it runs |
|---|---|
| The tests of `tools/tokcount` | `cargo test` |
| Its lints | `cargo clippy` with warnings as errors, `cargo fmt --check` |
| The shell scripts | `bash -n` and `shellcheck` on every one |
| The Python scripts | Their syntax |
| The documents | Every `docs/<name>.md` mentioned exists; every "§N" names a numbered section of the document it points at |

Four rules of form:

- **every step runs**, even after a failure, and the script ends on the
  list of those that failed: one run shows everything that is wrong;
- **a missing tool is an error**, never a skipped step. A check that
  skips in silence is a check that drifts;
- **no argument, no mode**: what runs locally is what runs remotely;
- **it writes nothing in the tree** but the build directory of
  `cargo`, which Git ignores.

**It grows with the project**: the count of Book I against the ceiling
at step 3; at step 7 the compiler's own tests, the examples of the
Bible (`docs/bible.md` §4), the verdicts of the corpus, the count of
§8, and the reference programs built and compared on their output.
**Timed sessions are not in it**: a check that takes ten minutes is not
run, and a duration on a shared runner is not a figure (§2). They have
a script of their own, written at step 7 with the programs.

Installed at step 7, as `docs/targets.md` §1 has it: a `pre-push` hook
and a remote workflow, both of which do nothing but call it. Until then
it is run by hand before each commit. `shrimp test` takes over when it
exists.

## 11. Set aside

| Option | Why not |
|---|---|
| The document alone, the script later | The same checks were being replayed by hand at every commit; the first run of the script found three references no reading had caught (§13) |
| The reference programs written in C and Rust now | Nothing is coded before step 7; their Shrimp versions need the prototype anyway |
| The programs of the Benchmarks Game | The fastest C, C++ and Rust entries are flagged there for "possible hand-written vector instructions or "unsafe"", and single-thread and multi-thread entries sit in one table (§14): they measure the tuning, not the language as it is written |
| One real program and no kernel | A real program does not say which assumption won or lost |
| Kernels and no real program | Stand-ins only (§3) |
| C compiled by GCC alone | Measures the backend as much as the language (§4) |
| The real program rewritten in C, Go and OCaml | Three ports of an engine, to compare one program |
| A reference machine | There is one machine, a laptop; and a figure tied to a machine dies with it |
| Durations published as such | Not comparable from one machine to another |
| A remote runner as the place of measurement | Shared, hence not reproducible |
| The lowest of the passes instead of the median | It answers "how fast can it go", and one lucky pass decides; the median moved by less than 1% between sessions (§13) |
| A third-party benchmarking tool | The session is thirty lines of shell (§13), and what the project measures with is rewritten in Shrimp one day (`docs/tool.md` §1) |
| Thresholds fixed now | Guessed before the first figure |
| Counting refusals on the reference programs only | Written by an author who knows the checker and walks around its walls |
| The test suite of `rustc` as the corpus | Tests of one checker's rules, not correct programs a checker gets wrong |
| Several entries for one pattern | The count becomes a choice of the author |
| A comparison language for "readable" | Measures the reader's fluency in that language |
| A survey, several readers | A language for two, first (`docs/budget.md` §5) |
| Target routines counted as hidden calls | Zero would be out of reach on a processor without a floating-point unit, or every division there would have to be written as a call |
| Timed measurements inside the check script | Too slow to be run at each commit, and no figure on a shared runner |

## 12. Deferred, with the step that decides

| What | Step |
|---|---|
| The first entries of the corpus, with each guarantee and its cost | 2 |
| The closed list of the routines a target may need for an operator (§8) | 3 |
| The count of Book I in the check script | 3 |
| The code of the kernels, their sizes, the generator and its seed; the generator of the compile-time program; the session script; where the result files live; the form of the request of §8; the check script installed, locally and remotely | 7 |
| The first figures: compile time, latency, speed, corpus, nothing hidden, stable | 7 |
| The instruction counter opened on the measuring machine, and its spread measured | 7 |
| The size of `core` on the ESP32-C3 | 8 |
| The thresholds, by dated decision | 11 |
| "Readable": the number of functions, the draw, the sitting; the corpus counted again with the refusals of the trial | 12 |
| The measurements as an appendix of the Bible (`docs/bible.md` §1) | 14 |

## 13. What was tried

On 2026-11-14, on the machine at hand: an Intel Core i7-6700HQ, four
cores and eight threads, 15 GiB, Linux 6.8.0, a laptop on the
`powersave` governor, in ordinary use — a load average of 0.9. GCC
13.3.0, Clang 18.1.3, `rustc` 1.97.1, Go 1.22.2, Python 3.12.3.
Everything below was run outside the repository except the check
script.

**The session of §2.** One kernel in the manner of `sort` — a heapsort
over 3,000,000 integers, the comparison passed by address in C and as a
generic parameter in Rust — and a session script of thirty lines of
shell. The three programs print the same checksum. Three sessions of
eleven interleaved passes, medians in milliseconds:

| | Session 1 | Session 2 | Session 3 | Spread of the passes |
|---|---:|---:|---:|---:|
| C, GCC `-O2` | 790.5 | 793.3 | 786.1 | 4.0 to 10.2% |
| C, Clang `-O2` | 509.8 | 505.9 | 509.0 | 5.2 to 6.6% |
| Rust, level 3 | 507.7 | 507.1 | 508.8 | 4.3 to 5.9% |

The medians of one program stay within 1% of each other; Rust over C
is 0.64 under GCC and 1.00 under Clang, in all three sessions. What
this proves: the method gives a ratio that repeats, on a laptop. What
it does not: one kernel, one machine; and it does not say why Clang
closes the gap — the kernels of step 7 are there for that.

**The latency of §5.** A program of one line of output, eleven runs in
each state, the time taken from the command to the end of the process —
which, for one line, is the first byte to within the exit. The figures
are in §5. `go run` costs about a sixth of a second whether the source
changed or not.

**One entry of the corpus of §6.** "Find the slot of a key in a
growable array, or append one, and return it": the natural form in C
prints the expected output under the address and undefined-behaviour
sanitizers; the natural form in Rust is refused by `rustc` 1.97.1 with
`E0499`. It is the shape of problem case #3 of the RFC that introduced
non-lexical lifetimes, whose own example, `get_default`, is refused
with the same code by the same compiler (§14).

**The check script of §10**, run on the repository and on a copy with
nine faults planted one at a time: a section number that does not exist
in another document, one in the same document, a document that does not
exist, an unquoted variable in a script, a shell syntax error, a Python
syntax error, a failing test, unformatted and unused Rust, a missing
tool. Each was caught, and the clean copy passes. The document step
gives the same result under three implementations of `awk`.

**What its first run found**: three references that pointed nowhere —
"§8" twice in `docs/budget.md` and "§7" once in `docs/guarantees.md`,
each meaning a section of the requirements and naming no document.
They now name it.

**What was not tried**: the instruction count — `perf stat` is refused
to an ordinary user on this machine, where `kernel.perf_event_paranoid`
is 4, and opening it is a change of system setting; the compile-time
measurement, whose generator does not exist; the size on an ESP32-C3,
for which no toolchain is installed; OCaml, which is not installed
either. And nothing of Shrimp.

## 14. Sources re-read on 2026-11-14

- https://benchmarksgame-team.pages.debian.net/benchmarksgame/ —
  version 25.03 of the pages. "Why toy programs?": the quotation of
  *Computer Architecture: A Quantitative Approach* on kernels, toy
  programs and synthetic benchmarks, "All three are discredited today".
  The n-body table: C gcc #9, C++ g++ #0 and Rust #9, the three
  fastest, carry the mark "possible hand-written vector instructions or
  "unsafe""; "Some of these programs are single-thread and some are
  multi-thread". The source of C gcc #9 was read: it is written with
  `__m256d` and the `_mm256_` intrinsics. "How programs are measured":
  "the lowest elapsed time from 12 BenchExec measurements". Only the
  n-body programs were looked at.
- https://github.com/rust-lang/rfcs — `text/2094-nll.md`: four problem
  cases; "Problem case #3: conditional control flow across functions",
  with `get_default`, "just as before, it will not compile". That the
  compiler still refuses it is not quoted: it was run (§13).
- https://github.com/torvalds/linux —
  `Documentation/admin-guide/sysctl/kernel.rst`,
  `perf_event_paranoid`: "The default value is 2"; at 1 and above,
  "Disallow CPU event access by users without CAP_PERFMON". The table
  stops at 2; the value 4 found on the machine is not in it, and that
  it comes from the distribution is quoted from memory.
- https://gcc.gnu.org/onlinedocs/gccint/Soft-float-library-routines.html
  — "The software floating point library is used on machines which do
  not have hardware support for floating point"; `__mulsf3`. And
  `Integer-library-routines.html`: "used on platforms that don't
  provide hardware support for arithmetic operations on some modes";
  `__udivsi3`.
- https://gcc.gnu.org/onlinedocs/gcc/Optimize-Options.html — `-Os`:
  "Optimize for size. -Os enables all -O2 optimizations" but a few.
- https://doc.rust-lang.org/cargo/reference/profiles.html — the
  `release` profile has `opt-level = 3`; `"s"`: "optimize for binary
  size"; `"z"`: "optimize for binary size, but also turn off loop
  vectorization".
- https://github.com/golang/go — `src/cmd/go/alldocs.go`: "Run compiles
  and runs the named main Go package."
- Not re-read: OCaml, which requirements §8 names as a reference and of
  which nothing is claimed here; the ESP32-C3, whose facts are those of
  `docs/targets.md` §5.
