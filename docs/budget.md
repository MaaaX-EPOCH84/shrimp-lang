# Complexity budget and non-goals — Shrimp

Decision of point 1.2 of the roadmap, 2026-10-21. The guarantees
(`docs/guarantees.md`) say what is refused outright; here, how
everything else is decided — what no guarantee settles — and what the
project declares it does not pursue. The facts about other languages
were re-read at the source on the day of the decision; the sources are
at the end of the document.

**Read and approved, Maxime MARCONATO (aka MaaaX^^, aka EPOCH84),
2026-10-21.** Any later change is dated and signed.

*Translated into English on 2026-10-22. The French text approved at commit
888832d remains the reference until this translation is re-read.*

Four trade-off decisions taken by MaaaX before writing, on 2026-10-21:
the budget is a grid for admitting and a ceiling for refusing; the four
features sent back by 1.1 are decided here; the non-goals remain five;
the GUI receives an amendment.

## 1. What the budget is

A feature that no guarantee refuses is not admitted for all that. It
costs, and it costs in a bounded currency: **the specification of the
core language must fit in the context window of an AI** (requirements
§8, figure at point 1.3). Each feature spends there its rules, its edge
cases and its interactions with the features already admitted.

Two properties make the budget a one-way door:

- **Admitting is irreversible.** What compiles today compiles
  tomorrow; an edition hides, it does not remove
  (`docs/architecture.md`, §4). An admitted feature stays in the
  language forever, even deprecated.
- **Refusing is reversible.** Nothing prevents admitting later a
  feature refused today, if a witness program shows that it is missing.

**When in doubt, refuse.** The asymmetry dictates the rule.

The budget decides in two stages: a **grid**, which says whether a
feature pays its way; a **ceiling**, which says whether there is room
left. The grid applies from today; the ceiling awaits its figure.

## 2. The grid: "every feature pays its way"

Seven questions, in this order. **The first "no" stops**: the feature
is refused, and the question that refused it is archived with it. An
admitted feature has answered all seven, and its answers are archived
too, because they are the spec in embryo.

| # | Question | Answer that passes |
|---|---|---|
| Q0 | Does a guarantee refuse it? | No. Already done in 1.1 for the known features; redone for any new feature |
| Q1 | Can it live in a library? | No. If it can, that is where it goes — `std` or `x/` — and it costs the core language nothing |
| Q2 | Which measurement of requirements §8 does it serve, or which guarantee requires it? | At least one, named. A feature that advances no measurement pays nothing |
| Q3 | Is the witness program more readable with it than without? | Yes, side by side: the same program written with only the features already admitted, and with the candidate. If the "without" reads just as well, the candidate is redundant |
| Q4 | Does it duplicate a way of doing things already admitted? | No, or it replaces it entirely. One single way of doing each thing |
| Q5 | Which admitted features does it interact with, and does each interaction have its rule? | All listed, all settled. An interaction without a rule is a refusal: it is the exact place where "best of each" languages break (requirements §6) |
| Q6 | What does it cost in spec tokens? | An estimate on entry, the measurement on the real text at step 3; under the ceiling, or the feature waits |

Three possible verdicts: **Admitted**, **Refused** (with the question),
**Postponed** (grid passed, ceiling not yet set or spec text not yet
written; the feature does not enter as long as the measurement is
missing).

**Who applies it.** The AI prepares the case: it writes the seven
answers and the witness program. MaaaX decides, dates and signs. A
feature does not enter the core language without its archived grid.

**What the grid does not judge.** Syntax, which is copied (requirements
§6), and the speed of the generated code, which is measured (point
1.8). The grid judges what a feature costs to read and to specify.

## 3. The ceiling

The ceiling is a number of tokens, that of the core language spec, set
at point 1.3 with the tokenizer and the script that measure it. *Set on
2026-10-21, `docs/spec_size.md`, approved by MaaaX:* 100,000 tokens all
included, at the worst of four frozen vocabularies, counted by
`tools/tokcount`. The rules that follow remain; the "until then" is
lifted:

- the grid alone decides, and every "Admitted" verdict given before 1.3
  is **subject to the ceiling**;
- a feature admitted by the grid but which would exceed the ceiling is
  **Refused**, or the ceiling is raised by a decision of MaaaX, dated
  and motivated. It is never raised in passing;
- what an edition hides leaves the current text of the spec and goes to
  an appendix, outside the ceiling: the AI writing in the current
  edition does not have to load it. The appendix remains normative for
  earlier editions.

Before version 1.0, nothing is frozen: the grid applies, the ceiling is
measured, the verdicts are revised at the audit (steps 6, 11, 15). From
1.0 on, the door is one-way.

## 4. The four features sent back by point 1.1

### `defer` — Refused (Q4)

**What it is.** Go: "A 'defer' statement invokes a function whose
execution is deferred to the moment the surrounding function returns".
Zig: "Executes an expression unconditionally at scope exit", in reverse
order.

**The grid.** Q0: visible, no guarantee settles it (1.1). Q1: no, it is
control flow. Q2: it would serve "readable" by bringing the release
closer to the acquisition. Q3: the witness program is a function that
opens a file, performs three operations that can fail, and closes it.
With `defer`, the close is written at the top and executed at the
bottom: the reader must keep a deferred action in mind, which G4
tolerates but does not like. Without `defer`, the close is an explicit
consumption that the compiler enforces on every exit path (G2 × G4
trade-off decision of point 1.1). **Q4: it duplicates the enforced
consumption.** Two ways to release a resource, one checked by typing,
the other by a placement convention. Refused.

**What the refusal leaves to step 3.** The need behind `defer` is
real: with a `?`-style propagation operator, a live resource must be
consumed before each propagation, and writing it on every line is
unreadable. Constraint for the design of resources: **consumption on
all exit paths must be writable once, visibly, without a destructor.**
The mechanism — resource block, or other — belongs to the spec, under
G4.

### `async`/`await` — Refused (Q5)

**What it is.** Rust: "Use `async` in front of `fn`, `closure`, or a
`block` to turn the marked code into a `Future`. As such the code will
not be run immediately, but will only be evaluated when the returned
future is `.await`ed"; "Futures alone are *inert*; they must be
*actively* `poll`ed for the underlying computation to make progress".
The executor is not provided: "Rust lets you choose one depending on
your requirements, rather than providing one".

**The grid.** Q0: visible, not settled (1.1). Q1: the executor, yes;
the transformation of a function into a state machine, no — it is in
the compiler. Q2: no measurement of §8; what it serves is the writing
of servers, which `std` must allow (network, HTTP) but which threads
and G3 allow too. Q3: the witness program — a server serving ten
connections — reads better in synchronous threads: each connection is
an ordinary function. Q4: it duplicates the thread-based concurrency
model, and **it duplicates `std`**: every blocking function calls for
its asynchronous twin (Rust has `std::fs` and `tokio::fs`), which
contradicts "one single convention everywhere". That would already be a
refusal. **Q5: an interaction without a rule.** A future abandoned
before its end holds resources; in Rust, cancellation is a `Drop`.
Shrimp has no destructor (1.1): a canceled future is an unconsumed
resource, and no rule says who consumes it. Refused.

**What the refusal leaves.** Shrimp's concurrency is that of G3:
threads, and a typed sharing model to be set at step 3. If a witness
program one day shows that a threaded server fails a measurement of
§8, the feature goes through the grid again with a rule for
cancellation — or lives in a library in `x/`, without syntax.

### Inheritance — Refused (Q5)

**What it is.** Rust has none: "There is no way to define a struct
that inherits the parent struct's fields and method implementations
without using a macro." Its two uses, according to the same text:
reusing code, and using a child type where the parent type is expected.
Go neither: "Object-oriented programming, at least in the best-known
languages, involves too much discussion of the relationships between
types".

**The grid.** Q0: not settled (1.1). Q1: no. Q2: "readable", for
reuse. Q3: the witness program — three shapes with an area — reads just
as well as a closed sum type, and better when one must know what a
value can be. Q4: reuse is already served by composition and generics
(admitted in 1.1); substitution will be served by the polymorphism
mechanism of step 3. **Q5: two interactions without a rule.** With G5:
a hierarchy is open, a child can be added elsewhere, so a match over
the hierarchy is never exhaustive — G5 cannot apply. With value
semantics and G2: a child copied into a parent loses its fields (C++'s
*slicing*), or else everything goes by reference, which imposes
allocation where G4 wants to see it written. Refused.

**What the refusal leaves.** Closed sum types, composition, and an
interface-based polymorphism whose form — static through generics,
dynamic through a declared type — is set at step 3 under G4.

### Local type inference — Refused (Q3)

**What it is.** Rust: "The compiler can usually infer what type we
want to use based on the value and how we use it. In cases when many
types are possible … we must add a type annotation". With a bound: "In
function signatures, you *must* declare the type of each parameter. …
Requiring type annotations in function definitions means the compiler
almost never needs you to use them elsewhere".

**The grid.** Q0: nothing executes, G4 does not settle it (1.1) — the
verdict of `guarantees.md` holds, it is not G4 that refuses. Q1: no.
Q2: "readable", by removing the annotations from local variables.
**Q3: the witness program reads better with the types written out.**
The reference reader for the "readable" measurement (requirements §8)
is MaaaX, and his verdict, on 2026-10-21: an inferred type is implicit
code at reading time; a written type leaves no doubt. Without tools,
without an IDE, with a ballpoint pen, `let n = f(x)` forces one to go
and read `f`; `let n: u32 = f(x)` reads in place. Refused.

**What the refusal costs, and to whom.** Writing every type is long
and sometimes heavy — a nested generic type on a `let`. That cost is at
writing time, hence on the AI: it is exactly the reversal of
requirements §3, the difficulty moves to the one who writes, reading
stays simple. The compiler, for its part, checks instead of guessing: a
wrong annotation is an error, not an inference.

**What the refusal leaves to step 3.** Shrimp is strict: every
binding — local variable, parameter, return, field, constant, closure
parameter — carries its written type. It remains for the spec to set
how a literal gets its own (annotation of the binding, suffix, or both)
without silent promotion, and to make sure that types stay short to
write: named aliases, not inference.

## 5. The non-goals

The five of requirements §6, re-read one by one. No sixth: the
guarantee verdicts (no macros, no operator overloading, no exceptions)
are motivated refusals, archived in `guarantees.md`, not non-goals.

| Non-goal | Outcome | Reason |
|---|---|---|
| Graphical user interface in `std` | **Amended** | "Not in `std`" did not say where. Decided: GUIs live in `x/`, in the form of safe C wrappers **per system** — one for each native toolkit —, **with no common abstraction** above them. Shrimp unifies nothing: each system exposes its own. It is precisely the common abstraction (Swing, `tkinter`) that the non-goal targeted, and it remains refused |
| Competing with Rust's ecosystem | Confirmed | The C boundary is a chapter of the core language (1.1, §2 bis); it is what makes existing code callable |
| Adoption by the world as a success criterion | Confirmed | A language for two, first; the "readable" measurement is made with MaaaX, not with a survey |
| Syntactic novelty | Confirmed | The grid does not judge syntax (§2); it is copied |
| "The best of each language" | Confirmed, and tooled | Q5 is its mechanical translation: an interaction without a rule refuses the feature |

The amendment is carried over into requirements §6, dated and signed.

## 6. Sources re-read on 2026-10-21

- Go, *The Go Programming Language Specification*, "Defer
  statements": "A 'defer' statement invokes a function whose
  execution is deferred to the moment the surrounding function returns,
  either because the surrounding function executed a return statement,
  reached the end of its function body, or because the corresponding
  goroutine is panicking."
- Zig, *Language Reference* (master), "defer": "Executes an
  expression unconditionally at scope exit."; "Defer expressions are
  evaluated in reverse order."
- Rust, `std` documentation, `async` keyword: "Use `async` in
  front of `fn`, `closure`, or a `block` to turn the marked code into a
  `Future`. As such the code will not be run immediately, but will only
  be evaluated when the returned future is `.await`ed."
- Rust, `std::future::Future`: "Futures alone are *inert*; they must
  be *actively* `poll`ed for the underlying computation to make
  progress".
- Rust, *Asynchronous Programming in Rust*, "async and await":
  "Rust lets you choose one depending on your requirements, rather
  than providing one."; "You'll need to include the Tokio crate as
  a dependency in your Cargo.toml".
- Rust, *The Rust Programming Language*, "Characteristics of
  Object-Oriented Languages": "If a language must have inheritance
  to be object oriented, then Rust is not such a language. There is no
  way to define a struct that inherits the parent struct's fields and
  method implementations without using a macro."; "Inheritance is
  often at risk of sharing more code than necessary."
- Go, *FAQ*, "Why is there no type inheritance?": "Object-oriented
  programming, at least in the best-known languages, involves too much
  discussion of the relationships between types, relationships that
  often could be derived automatically. Go takes a different approach."
- Rust, *The Rust Programming Language*, "Data Types": "The
  compiler can usually infer what type we want to use based on the
  value and how we use it. In cases when many types are possible … we
  must add a type annotation". "Functions": "In function
  signatures, you *must* declare the type of each parameter. This is a
  deliberate decision in Rust's design: Requiring type annotations in
  function definitions means the compiler almost never needs you to use
  them elsewhere in the code to figure out what type you mean."
- Rust, *The Rust Edition Guide*, "What are editions?": "crates in
  one edition **must** seamlessly interoperate with those compiled with
  other editions"; "Editions do not split the ecosystem". This is the
  model for the exit door of §3.
