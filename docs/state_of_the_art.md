# State of the art — what we read before we write

Step 2 of the roadmap: for each language, what it does for each of the
five guarantees retained (`docs/guarantees.md` §2), and what that costs
it. Point 2.1, 2026-11-15, fixes the form before anything is read: the
grid and what a cell holds (§1), the rule of re-reading (§2), the list
of languages (§3), the place of the text and how it is approved (§6).
Seven questions settled by MaaaX on 2026-11-15, on the assistant's
recommendations, in one answer. The grid is empty: every cell says
`not read`, and points 2.2 to 2.6 fill it, lot by lot.

**§1, §2, §3, §6 and §10 read and approved, Maxime MARCONATO (aka
MaaaX^^, aka EPOCH84), 2026-11-15.** Any later change is dated and
signed. The lots of §5 are approved one by one, each at its point.

## 1. The grid

One row per language, one column per guarantee:

| | Guarantee (`docs/guarantees.md` §2) |
|---|---|
| G1 | No undefined behavior outside a named `unsafe` block |
| G2 | Lifetimes decided at compile time, no garbage collector |
| G3 | No data races |
| G4 | Nothing hidden: a function can be read within the function |
| G5 | No forgotten case |

**A cell holds four things**: a status, the mechanism in one line, the
cost in one line, the source.

**The status is one of four words**, and nothing else:

| Status | What it means |
|---|---|
| `guaranteed` | The compiler refuses the program that breaks the guarantee, in the language as it is written outside a named escape |
| `checked at run time` | The program that breaks it is stopped or reported when it runs, by a check the implementation puts there by default |
| `by convention` | The language gives the means and imposes nothing: discipline, an optional tool, a mode that is not the default |
| `absent` | The language has no means, or chose the opposite — a garbage collector, for G2 |

A fifth word, `not read`, marks a cell nobody has filled yet. It is the
only thing an unfilled cell may contain, and a lot is done when none of
its cells says it.

**A status is never qualified.** A language that holds a guarantee in
part takes the status of the weakest part, and the mechanism line says
which part is stronger. "Mostly guaranteed" is the adjective this grid
exists to refuse.

**The cost names the measurement that pays.** It opens on one or more
of these words, then a colon and one line that says how:

| Word | The measurement of requirements §8 |
|---|---|
| `spec` | Short spec |
| `compile time` | Fast to compile |
| `speed` | Fast code |
| `refusals` | Checker not too strict |
| `reading` | Readable |
| `stability` | Stable |
| `hidden` | Nothing hidden |
| `none` | The source read shows no cost on any of them |
| `not found` | A cost exists and no measurement of requirements §8 carries it; the line says which |

For a status `absent`, the cost is what the language pays instead:
requirements §3 already says that Go pays with a garbage collector and
Zig with run-time checks and discipline. That line is what the row is
read for.

**The grid is checked by the check script** (`docs/measurements.md`
§10): every language of §3 has its row in §4 and its five cells in §5,
every status is one of the five words, every cost opens on a word of
the table, a filled cell has its mechanism, its cost and its source, an
unfilled one has nothing, and the row of §4 says what the cells of §5
say.

## 2. The rule of re-reading

The mechanisms of §7 to §9 were written from memory. Nothing enters
the grid that way.

1. **Primary sources only**: the reference or the specification of the
   language, its official documentation, a proposal accepted by its
   project, a text signed by its authors. Anything else serves to find
   the primary source, and is not cited.
2. **Every source is dated**: the address, the version of the language
   or of the document, the day it was read. Each language of §5 ends
   on the list of its sources.
3. **A quotation where the fact surprises**, or contradicts what §7
   expected: the reader must be able to check the sentence, not the
   summary.
4. **Nothing from memory.** A cell that was not re-read stays
   `not read`. A language whose primary source cannot be read says so
   in place of its cells.
5. **A verdict of a compiler is run, not quoted**, whenever the
   compiler is at hand, and the cell names the version. What was run
   says "run"; what was read says "read".
6. **`absent` is established, not assumed**: by the language's own
   statement, or by saying what was searched and where. Not finding is
   first a measure of the search.

## 3. The languages

Eighteen languages, in five lots. **The list is frozen**: a language
enters or leaves by a dated decision.

| Language | Lot | Why it is read |
|---|---|---|
| Rust | 2.2 | The base: ownership, borrowing, `Send` and `Sync`, sum types, editions |
| Zig | 2.3 | Nothing hidden as a doctrine; safety by run-time checks and discipline |
| Austral | 2.3 | Linear types, a specification kept small on purpose |
| Hylo | 2.3 | Mutable value semantics: the guarantees without references in the language |
| Vale | 2.3 | Generational references: memory safety by cheap run-time checks |
| Mojo | 2.3 | Ownership on a Python syntax |
| Ada | 2.4 | Written to be read and maintained for thirty years by others |
| SPARK | 2.4 | The subset of Ada that proves the absence of run-time errors |
| Swift | 2.4 | Started simple, became enormous: the counter-example of the budget |
| Go | 2.5 | A short specification, a compatibility promise kept, a garbage collector |
| OCaml | 2.5 | Algebraic types and very short compile times, with a garbage collector |
| Erlang | 2.5 | No shared memory: actors and messages |
| Pony | 2.5 | Data races excluded at compile time by capabilities on references |
| C | 2.6 | The ABI every language must speak; the reference of the measurements |
| C++ | 2.6 | Destructors and RAII, the model refused at point 1.1 |
| Fortran | 2.6 | Faster than C on arrays because they do not alias |
| Halide | 2.6 | Beats hand-written C by separating the algorithm from its schedule |
| ISPC | 2.6 | Beats hand-written C by expressing data parallelism |

The reasons are those of §7, in a line; they are expectations, and the
lot checks them.

**Two additions to the starting list**, decided with the form: C++, for
the destructor that point 1.1 refused and for the recipe the budget was
written against; Pony, for one more way to exclude data races at
compile time. **Ada and SPARK have a row each**: one checks at run time
what the other proves, and one row could not hold both statuses.
**Halide and ISPC have a row each** for the same reason of form.

**WLangage and PHP are not in this grid.** They are read for the
library — the single convention, and its counter-model — and have
nothing to say of G1 to G5. They belong to point 2.7, with a grid of
their own whose form is fixed there.

**The backends are not read here.** `docs/targets.md` §3 re-read
Cranelift and LLVM against the constraints of the targets; the choice
and what it still needs to read belong to step 7.

## 4. The grid, at a glance

| Language | G1 | G2 | G3 | G4 | G5 |
|---|---|---|---|---|---|
| Rust | not read | not read | not read | not read | not read |
| Zig | not read | not read | not read | not read | not read |
| Austral | not read | not read | not read | not read | not read |
| Hylo | not read | not read | not read | not read | not read |
| Vale | not read | not read | not read | not read | not read |
| Mojo | not read | not read | not read | not read | not read |
| Ada | not read | not read | not read | not read | not read |
| SPARK | not read | not read | not read | not read | not read |
| Swift | not read | not read | not read | not read | not read |
| Go | not read | not read | not read | not read | not read |
| OCaml | not read | not read | not read | not read | not read |
| Erlang | not read | not read | not read | not read | not read |
| Pony | not read | not read | not read | not read | not read |
| C | not read | not read | not read | not read | not read |
| C++ | not read | not read | not read | not read | not read |
| Fortran | not read | not read | not read | not read | not read |
| Halide | not read | not read | not read | not read | not read |
| ISPC | not read | not read | not read | not read | not read |

## 5. The cells, language by language

### Rust

| Guarantee | Status | Mechanism | Cost | Source |
|---|---|---|---|---|
| G1 | not read | | | |
| G2 | not read | | | |
| G3 | not read | | | |
| G4 | not read | | | |
| G5 | not read | | | |

### Zig

| Guarantee | Status | Mechanism | Cost | Source |
|---|---|---|---|---|
| G1 | not read | | | |
| G2 | not read | | | |
| G3 | not read | | | |
| G4 | not read | | | |
| G5 | not read | | | |

### Austral

| Guarantee | Status | Mechanism | Cost | Source |
|---|---|---|---|---|
| G1 | not read | | | |
| G2 | not read | | | |
| G3 | not read | | | |
| G4 | not read | | | |
| G5 | not read | | | |

### Hylo

| Guarantee | Status | Mechanism | Cost | Source |
|---|---|---|---|---|
| G1 | not read | | | |
| G2 | not read | | | |
| G3 | not read | | | |
| G4 | not read | | | |
| G5 | not read | | | |

### Vale

| Guarantee | Status | Mechanism | Cost | Source |
|---|---|---|---|---|
| G1 | not read | | | |
| G2 | not read | | | |
| G3 | not read | | | |
| G4 | not read | | | |
| G5 | not read | | | |

### Mojo

| Guarantee | Status | Mechanism | Cost | Source |
|---|---|---|---|---|
| G1 | not read | | | |
| G2 | not read | | | |
| G3 | not read | | | |
| G4 | not read | | | |
| G5 | not read | | | |

### Ada

| Guarantee | Status | Mechanism | Cost | Source |
|---|---|---|---|---|
| G1 | not read | | | |
| G2 | not read | | | |
| G3 | not read | | | |
| G4 | not read | | | |
| G5 | not read | | | |

### SPARK

| Guarantee | Status | Mechanism | Cost | Source |
|---|---|---|---|---|
| G1 | not read | | | |
| G2 | not read | | | |
| G3 | not read | | | |
| G4 | not read | | | |
| G5 | not read | | | |

### Swift

| Guarantee | Status | Mechanism | Cost | Source |
|---|---|---|---|---|
| G1 | not read | | | |
| G2 | not read | | | |
| G3 | not read | | | |
| G4 | not read | | | |
| G5 | not read | | | |

### Go

| Guarantee | Status | Mechanism | Cost | Source |
|---|---|---|---|---|
| G1 | not read | | | |
| G2 | not read | | | |
| G3 | not read | | | |
| G4 | not read | | | |
| G5 | not read | | | |

### OCaml

| Guarantee | Status | Mechanism | Cost | Source |
|---|---|---|---|---|
| G1 | not read | | | |
| G2 | not read | | | |
| G3 | not read | | | |
| G4 | not read | | | |
| G5 | not read | | | |

### Erlang

| Guarantee | Status | Mechanism | Cost | Source |
|---|---|---|---|---|
| G1 | not read | | | |
| G2 | not read | | | |
| G3 | not read | | | |
| G4 | not read | | | |
| G5 | not read | | | |

### Pony

| Guarantee | Status | Mechanism | Cost | Source |
|---|---|---|---|---|
| G1 | not read | | | |
| G2 | not read | | | |
| G3 | not read | | | |
| G4 | not read | | | |
| G5 | not read | | | |

### C

| Guarantee | Status | Mechanism | Cost | Source |
|---|---|---|---|---|
| G1 | not read | | | |
| G2 | not read | | | |
| G3 | not read | | | |
| G4 | not read | | | |
| G5 | not read | | | |

### C++

| Guarantee | Status | Mechanism | Cost | Source |
|---|---|---|---|---|
| G1 | not read | | | |
| G2 | not read | | | |
| G3 | not read | | | |
| G4 | not read | | | |
| G5 | not read | | | |

### Fortran

| Guarantee | Status | Mechanism | Cost | Source |
|---|---|---|---|---|
| G1 | not read | | | |
| G2 | not read | | | |
| G3 | not read | | | |
| G4 | not read | | | |
| G5 | not read | | | |

### Halide

| Guarantee | Status | Mechanism | Cost | Source |
|---|---|---|---|---|
| G1 | not read | | | |
| G2 | not read | | | |
| G3 | not read | | | |
| G4 | not read | | | |
| G5 | not read | | | |

### ISPC

| Guarantee | Status | Mechanism | Cost | Source |
|---|---|---|---|---|
| G1 | not read | | | |
| G2 | not read | | | |
| G3 | not read | | | |
| G4 | not read | | | |
| G5 | not read | | | |

## 6. The place of the text, and its approval

**One document, this one.** The grid is read across languages; a file
per language would hide the columns.

**Approved lot by lot.** Each of points 2.2 to 2.6 fills its languages
and is read and approved at its closing, with a dated line under the
heading of each language. Point 2.9 then reads the columns and writes
what the grid says to step 3. Eighteen languages approved in one
sitting would be approved unread.

**The starting list stays** in §7 to §9 as it was written on
2026-10-20, from memory, with the two rows added on 2026-11-15: it is
the record of what was expected. Point 2.9 sets it beside what was
found.

**`docs/guarantees.md` is not touched.** Its "Who has it" lines were
re-read on 2026-10-21 and stand as approved; the grid takes them up
and extends them.

## 7. The starting list, a priori

Written on 2026-10-20, before any re-reading: the reason for each
entry, and what was expected of it. Cited from memory.

| Language | Why read it | What we take from it, a priori | What we avoid |
|---|---|---|---|
| **Rust** | The base. Ownership, borrowing, `Send`/`Sync`, sum types, `Result`/`?`, traits, editions, cargo | The guarantees and their proven implementation | Compile times, async, procedural macros, unreadable signatures |
| **Zig** | "Nothing hidden": no hidden control flow or allocation, explicit allocators, `comptime` in place of generics and macros, errors and optionals as values; reference in one page | The doctrine of the explicit, `comptime` | Not safe by construction; not at 1.0, breaks with every version |
| **Go** | Spec readable in an afternoon, compatibility promise kept since 2012, complete `std`, `x/` incubator | Library governance, stability | The garbage collector, `nil`, races not forbidden |
| **Ada / SPARK** | Designed to be read and maintained for thirty years by others; SPARK proves the absence of run-time errors, and borrowed an ownership model from Rust for its pointers | Readability, contracts, proof | Verbosity, the ecosystem |
| **Hylo** | Mutable value semantics, zero references in the language: the attempt to keep Rust's guarantees while throwing away the complexity | The model, if it holds | Research, not stable |
| **Austral** | Linear types, tiny specification | Spec size as a discipline | Niche |
| **Vale** | Generational references: memory safety through cheap run-time checks | One point on the safety/cost grid | Research |
| **Mojo** | Ownership on a Python syntax, by the author of LLVM and Swift | What it simplified from Rust | Proprietary, in motion |
| **Swift** | Started simple, became enormous; strict Rust-style concurrency in Swift 6 | The counter-example for the complexity budget | — |
| **OCaml** | Algebraic types, inference, very short compile times; the first Rust compiler was written in it | Compile speed as a goal | The garbage collector |
| **Erlang** | No shared memory: actors and messages, forty years in telecom | The simplest concurrency model that is reliable | Not a systems language |
| **C** | The industry's ABI: every language has to talk to it | The boundary | Everything else |
| **Fortran** | Faster than C on numerical computing because its arrays do not alias | The proof that rigor makes speed | — |
| **Halide, ISPC** | Beat hand-written C by expressing parallelism | What a language can say that C cannot | Restricted domain |
| **WLangage** | Enormous and consistent library, one prefix per family | The single convention | Proprietary, a platform rather than a language |
| **PHP** | Enormous and inconsistent library | — | The inconsistency |
| **C++** | Destructors and RAII, the model point 1.1 refused; the language that took the best of every other | What a destructor costs the reader, seen where it was invented | The recipe, and the size it led to |
| **Pony** | Data races excluded at compile time by capabilities on references, with actors | One more answer to G3 | A garbage collector per actor |

## 8. Mechanisms and projects to follow

- **Polonius**, new formulation of Rust's borrow checker;
  **Tree Borrows**, aliasing model for `unsafe`; **Verus**,
  verification of Rust programs; **Dafny**, **Lean**, for what proof
  costs and yields.
- **Backends**: LLVM (Rust, Swift, Zig, Clang), Cranelift (simple,
  compiles fast), QBE (tiny, used by Hare), emitted C (Nim; cfront at
  the origin of C++).
- **Incubation**: `golang.org/x/`, Boost → C++, `hashbrown` and
  `once_cell` → Rust's `std`, PEP 594 (removal of Python's dead
  batteries).

## 9. Books and texts

- Pierce, *Types and Programming Languages* — the mathematically
  understood core on which everything that has succeeded in the last
  twenty years is built.
- Nystrom, *Crafting Interpreters* — the complete pipeline in practice.
- Cooper and Torczon, *Engineering a Compiler*; Aho, Lam, Sethi, Ullman,
  *Compilers* — the dragon book, for the backend.
- Dijkstra, *On the foolishness of "natural language programming"*
  (EWD667, 1978) — why the artifact stays formal.

## 10. Set aside

| Option | Why not |
|---|---|
| One file per language | The grid is read by column as much as by row |
| A status in free words | "Mostly", "largely", "in practice": the grid would say what its author feels |
| A qualified status, "guaranteed in part" | Every language holds every guarantee in part; the weakest part is the one a program meets |
| A cost in free words | A cost that names no measurement cannot be set beside Shrimp's |
| Secondary sources | They repeat each other, and the one they repeat is the primary source |
| Cells filled from memory and corrected later | The plausible is what passes every re-reading |
| The whole grid approved at point 2.9 | Too much to read in one sitting |
| WLangage and PHP in the grid of the guarantees | Ten cells saying `absent`, for two languages read for their library |
| Ada and SPARK on one row | One row, two statuses |
| The backends as a lot of this step | Already re-read where they constrain something (`docs/targets.md` §3); chosen at step 7 |
| A grid nothing checks | Fixed at the first language, drifting at the third |
