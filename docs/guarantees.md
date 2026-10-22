# Non-negotiable guarantees — Shrimp

Decision of point 1.1 of the roadmap, 2026-10-21. The candidates are in
`docs/requirements.md`, §5; here, what is retained, what is set aside,
and why. The facts about other languages were re-read at the source on
the day of the decision; the sources are at the end of the document.

**Read and approved, Maxime MARCONATO (aka MaaaX^^, aka EPOCH84),
2026-10-21.** Any later change is dated and signed.

*Translated into English on 2026-10-22. The French text approved at commit
888832d remains the reference until this translation is re-read.*

## 1. The criterion

The number of guarantees is not fixed in advance. A property deserves
the status of guarantee if, and only if, it meets all three conditions:

1. **Mechanically verifiable** — by the compiler, or by a replayable
   test. Otherwise it is a wish.
2. **It forbids something precise and nameable** — a feature, a
   construct, a behavior. A guarantee that forbids nothing decides
   nothing.
3. **It does not follow from another** — otherwise it is only a
   corollary, and counting it doubles the pairs to arbitrate.

Why have few of them all the same: every pair of guarantees fights,
"non-negotiable" wears out as the list grows, and every guarantee is a
chapter of spec in a bounded context window.

A guarantee is a **veto**: any feature that contradicts one is refused,
however excellent elsewhere. What no guarantee refuses is not accepted
for all that: it goes on to the complexity budget (point 1.2).

## 2. The guarantees retained

Five. Each is written as a sentence testable on a program.

### G1 — No undefined behavior outside a named `unsafe` block

**Statement.** Every program accepted by the compiler has a behavior
defined by the specification, for every input, except inside blocks
marked `unsafe`, for which the programmer answers. Safe code cannot
trigger undefined behavior, even by calling correct `unsafe` code.

**Verification.** The compiler; and a reference interpreter that detects
every violation on the test suite (what Miri does for Rust).

**Forbidden.** Reading uninitialized memory, dereferencing an invalid
pointer, integer overflow with semantics left open, unchecked
out-of-bounds access, any "optimization" based on unspecified behavior.
Every operation has a defined result: a value, an error, or program
termination — never "anything at all".

**Cost.** An `unsafe` sub-language to specify (aliasing model included);
runtime checks where the type proves nothing (bounds, division), hence
code slower than C that checks nothing.

**Who has it.** Rust: "If all you do is write Safe Rust, you will never
have to worry about type-safety or memory-safety. You will never endure
a dangling pointer, a use-after-free, or any other kind of Undefined
Behavior" (Nomicon). Austral: "well-defined semantics for all
arithmetic operations". Zig does not have it: it uses "Illegal
Behavior" as a tool, checked or not depending on the build mode.

### G2 — Lifetimes decided at compile time

**Statement.** For every resource — memory, file, lock — the place where
it is released can be read in the source and is decided at compile
time. No garbage collector, no implicit reference counting, no
finalizer at an unknown moment.

**Verification.** The compiler, through the ownership model (step 3).
Replayable test: `core` runs without an allocator or a runtime; no
garbage collector symbol in a binary.

**Forbidden.** The garbage collector (Go, Java, OCaml, Swift for
automatic counting), finalizers, and any sharing of ownership that is
not written in a named type. Explicit sharing, `Rc`-style, remains
possible; since there is no destructor (G4), the way the last owner
gives the memory back is a point of step 3.

**Why it does not follow from G1.** Memory safety, for its part, does
follow from G1: a use after free is undefined behavior, hence excluded.
But G1 is also satisfied with a garbage collector; Java has almost no
undefined behavior. G2 fixes the *means*: no runtime. It is what makes
`core` possible on a microcontroller.

**Cost.** An ownership and borrowing model, hence correct programs
refused (measurement of the requirements, §8); this is where the
complexity of Rust lives.

**Who has it.** Rust (ownership and borrowing), Austral (linear types:
"Memory can be managed safely and without runtime overhead"), Hylo
(value semantics; project declared "still in their early stages").

### G3 — No data races

**Statement.** Two threads cannot access the same location without
synchronization if one of them writes. A program that attempted it does
not compile.

**Verification.** The compiler, through the typing of sharing between
threads (what `Send`/`Sync` do in Rust); replayable test on a corpus of
faulty concurrent programs that must be refused.

**Forbidden.** The mutable global variable without synchronization, the
thread that captures a shared mutable reference, any "C-style" thread
model where sharing is free.

**Why it does not follow from G1.** In Rust, a data race is undefined
behavior, so G1 excludes it. But that is not a necessity: Java *defines*
the behavior of programs with races (memory model, JLS §17: "When code
contains a data race, counterintuitive results are often possible"),
and so does Go. G1 alone would therefore allow a Shrimp where the race
is legal and specified. G3 says it does not compile.

**Cost.** A sharing model in the language — marker types, or actors
without shared memory (Erlang: "Threads of execution in Erlang share no
data") —, to be decided at step 3. No guarantee against deadlocks or
"general" races: Rust has none either ("Rust does not prevent general
race conditions").

### G4 — Nothing hidden: a function can be read within the function

**Statement.** Everything that executes, allocates or converts in a
function is named on the line where it happens, or follows from the
signatures alone of the functions and types it names. A single implicit
rule, written in the spec: the memory of a value is given back at the
closing brace. This rule never executes code written by a user: there is
no destructor.

**Verification.** Syntactic, hence by the compiler; measurement of the
requirements §8 "allocations and calls not visible in the source: zero".

**Forbidden.** Exceptions and any catchable non-local exit; implicit
conversions; macros that rewrite the syntax; operator overloading on
user types (`+` that allocates); costly implicit construction or copy;
allocation without a named allocator or allocating type. Program
termination on a failed check (bounds, division by zero) is not a
catchable exit: it is admitted by G4, and required by G1.

**No destructor: resources are consumed.** Two kinds of things die at
the brace, and they are not treated the same way.

- **Memory** — string, array, structure — is given back by an operation
  that is always identical, that cannot usefully fail and where there is
  nothing to read. The compiler emits it: this is the implicit rule of
  G4, and the only one.
- **A resource** — file, lock, connection — is given back by code that
  someone wrote, that can fail and that acts outside the program. This
  code never runs without being called.

A type that holds a resource is declared **to consume**. A value of this
type must be passed exactly once, on every path that leaves the scope,
to a function that consumes it; otherwise the program does not compile,
and the error names the value and the faulty exit. A structure that
contains a resource is in turn to consume. The `?` is only permitted if
no value to consume is alive. On program termination, nothing is
consumed: the operating system reclaims.

What this gives: G4 without exceptions; the close that fails is an
ordinary result, which G5 forces to be handled; double free and
forgetting do not compile, which a destructor does not detect. Price:
error paths are written by hand when a resource is alive, and the one
who writes pays. This is Austral's linear model, limited to resources.
Syntax of the declaration and sharing of a resource between owners:
step 3.

**Cost.** Longer code than in Rust or Python; no arithmetic on user
types through operators; a visible error-propagation syntax (Rust's `?`
is visible: it passes, except when a resource to consume is alive).

**Who has it.** Zig: "There is no hidden control flow, no hidden memory
allocations, no preprocessor, and no macros"; Austral: "No exceptions
(and no surprise control flow)", "No implicit function calls", "No
implicit type conversions". Rust does not have it: overloading, `Deref`,
conversions through `From`, macros.

### G5 — No forgotten case

**Statement.** Every value that can be an error or an absence has a type
that says so, and every alternative of a sum type — error, absence,
variant — is handled explicitly, or the program does not compile.

**Verification.** The compiler: exhaustiveness of matches, unconsumed
result refused (an error, not a warning).

**Forbidden.** `null` as a value of an ordinary type; the return code
that can be ignored (C); the `switch` without a default case that falls
through silently; the discarded `Result`.

**Why it does not follow from G4.** G4 excludes exceptions, hence
requires the error to be a value; but a value can be ignored. G5 says it
cannot be. C and Go have errors as values and no G5.

**`null`.** No ordinary type has a "nothing" value: absence is a
separate type, which the compiler forces to be handled before the value
is taken out of it. This is not one more guarantee — it follows from G5
—, but it is its most frequent case. Zero survives in two places: in raw
pointers, under `unsafe` and at the C boundary, where the passage to
safe code requires the test; and in the representation, where an option
of a reference can fit in one word, which only the compiler sees.

**Cost.** Code that handles cases the programmer "knows" to be
impossible; pressure on the design of sum types (step 3).

**Who has it.** Rust (in part: an ignored result is only a warning),
Austral ("algebraic data types … with exhaustiveness checking"), Zig
(error unions, exhaustive `switch`).

## 2 bis. The C boundary

The requirements impose C interoperability in both directions (§7). The
guarantees do not cross the boundary: they say what happens when it is
crossed.

| Guarantee | At the boundary |
|---|---|
| G1 | Every call into C, and every call from C into Shrimp, is `unsafe`: the compiler does not see the other side. Whoever writes the safe wrapper answers for its correctness — Rust's *soundness* rule. No stack unwinding through C: it does not exist (G4), a termination is a termination |
| G2 | Ownership is declared on crossing: a pointer received from C is borrowed or owned, and the type says which; a resource that C frees is not a type to consume on the Shrimp side |
| G3 | A C function is not shareable between threads by default; it becomes so by declaration, under `unsafe` |
| G4 | An external call is named, hence visible; a callback from C into Shrimp goes through a declared function type. Nothing changes |
| G5 | A C return code arrives as an integer; the wrapper converts it into a result or an absence, and it is to the wrapper that G5 applies. The raw boundary, for its part, remains C |

The boundary is therefore a place, not an exception: everything that is
`unsafe` is written there as such, and everything that comes out of it
is safe because someone guaranteed it in writing. Its specification —
ABI, representable types, strings, callbacks — is a chapter of the core
language (step 3).

## 3. The candidates set aside

| Candidate from the requirements | Fate | Reason |
|---|---|---|
| Memory safety without a garbage collector | Reformulated as G2 | Safety follows from G1 (fails criterion 3); what does not follow from it is "without a runtime", and that is G2 |
| Errors as values | Absorbed by G4 and G5 | "As values" follows from G4 (no exceptions); what remains, exhaustiveness, is G5 |
| Compatibility over time | Off the list | Governance commitment, not a property of a program; guaranteed by corpus and editions, point 1.2 |
| Bounded specification | Off the list | Same nature: a constraint on the project, measured in tokens at point 1.3 |
| Zero-cost abstraction | Refused as a guarantee | Forbids nothing nameable (criterion 2): one can always write slow code. It is a design principle, measured at point 1.8 against C and Rust |

## 4. The pairs

Ten pairs. Those that fight, and the trade-off decision:

| Pair | Tension | Decision |
|---|---|---|
| G1 × G2 | Without a runtime, safety requires an ownership model, which refuses correct programs | Accepted; this is the "correct programs refused" measurement, to be kept below Rust |
| G1 × G4 | Runtime checks are code that was not written | A check that terminates is not catchable: admitted. A check that costs shows up in the measurement, not in the source |
| G2 × G4 | Releasing without a runtime requires code at every end of life | Memory is given back by the compiler, a single rule with no user code; resources are consumed explicitly, the compiler enforces it |
| G3 × G4 | The typing of sharing is inferred | Nothing executes: static inference is not hidden in the sense of G4 |
| G4 × G5 | Handling every case lengthens the code; propagation must remain visible | A visible propagation operator, `?`-style, satisfies both |
| the five others | — | No tension found |

## 5. Verdicts

The guarantees run over known features. A verdict that hesitates would
be a badly written guarantee; "to the budget" is not a hesitation: it
means that no guarantee decides, and that point 1.2 will.

| Feature | Verdict | By |
|---|---|---|
| Exceptions | No | G4 |
| Garbage collector | No | G2 |
| Automatic reference counting (Swift) | No | G2 |
| Explicit `Rc`-style sharing | Yes in principle, mechanism at step 3 | G2 admits it, G4 constrains its release |
| User-written destructor | No | G4 |
| Memory released at the brace | Yes, the language's single rule | G4 admits it, G2 requires it |
| Type to consume (linear resource) | Yes, exactly once per path | G2, G4, G5 |
| `defer` | To the budget | Visible, but redundant with enforced consumption |
| `null` | No | G5 |
| Ignorable return code | No | G5 |
| Implicit conversion | No | G4 |
| Operator overloading on user types | No | G4 |
| Syntactic macros | No | G4 |
| Named `unsafe` block | Yes | G1 requires it to exist |
| Calling a C function | Yes, under `unsafe`, safe wrapper at the author's charge | G1, G5 |
| Stack unwinding through C | No | G4: nothing is catchable |
| Mutable global variable | Not without typed synchronization | G3 |
| Integer overflow | Defined behavior, which one at step 3 | G1 |
| Program termination on a failed check | Yes | G1, G4 |
| Monomorphized generics | Yes | None opposes it |
| Asynchrony (`async`/`await`) | To the budget | Visible, so G4 does not decide; cost at step 1.2 |
| Inheritance | To the budget | No guarantee decides |
| Local type inference | To the budget | Nothing executes; G4 does not decide |

## 6. Sources re-read on 2026-10-21

- Rust, *The Reference*, "Behavior considered undefined": the list
  includes "Data races", access to a "dangling" place, reading
  uninitialized integers and pointers.
- Rust, *The Rustonomicon*, "Meet Safe and Unsafe" and "Data Races and
  Race Conditions": quotations above.
- Zig, *Overview* (ziglang.org/learn/overview): "There is no hidden
  control flow, no hidden memory allocations, no preprocessor, and no
  macros"; "Zig uses Illegal Behavior as a razor sharp tool"; no claim
  of memory safety.
- Austral, *Features* (austral-lang.org/features): list of the
  anti-features quoted.
- Hylo (hylo-lang.org): "A Safe Systems Programming Language",
  "compiler and standard library are still in their early stages".
- Java, *JLS* SE 21, §17.4: definition of the data race, semantics of
  incorrectly synchronized programs.
- Erlang, *Concurrent Programming*: "Threads of execution in Erlang
  share no data".
- Go, *Go 1 and the Future of Go Programs*: the compatibility promise,
  with no date on the page (the "since 2012" of the requirements remains
  quoted from memory).
