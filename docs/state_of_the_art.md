# State of the art — what we read before we write

To be filled in at step 2 with a fixed grid: for each language, what it
does for each of the guarantees retained, and what that costs it.
Below, the starting list and the reason for each entry. The mechanisms
are cited from memory: to be re-read at the source before they enter
a decision.

## Languages

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

## Mechanisms and projects to follow

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

## Books and texts

- Pierce, *Types and Programming Languages* — the mathematically
  understood core on which everything that has succeeded in the last
  twenty years is built.
- Nystrom, *Crafting Interpreters* — the complete pipeline in practice.
- Cooper and Torczon, *Engineering a Compiler*; Aho, Lam, Sethi, Ullman,
  *Compilers* — the dragon book, for the backend.
- Dijkstra, *On the foolishness of "natural language programming"*
  (EWD667, 1978) — why the artifact stays formal.
