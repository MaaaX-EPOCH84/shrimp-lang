# Architecture — Shrimp

Proposed on 2026-10-20, to be validated step by step. Two objects: a
frozen core language, a library in three tiers.

## 1. The core language

The core language is what can never be changed again. Small, frozen
early, specified in a text that fits in an AI's context window, with a
reference implementation and a test suite that is authoritative.

It contains this, and nothing else:
- the **ownership and aliasing model** — the one thing that cannot be
  grafted on afterwards (C++ has been trying for fifteen years);
- the **error model**: values, exhaustiveness, propagation;
- the **memory model**: what is guaranteed between threads;
- the **module system** and visibility;
- the **type system**: sums, products, generics or `comptime`, traits or
  interfaces — to be decided;
- the **minimal syntax**, borrowed, not invented;
- the **boundary**: `unsafe` if it exists, and the C ABI.

The standard library, however huge, is not part of it.

## 2. The three tiers

| Tier | Contents | Rule |
|---|---|---|
| `core` | No OS, no allocation: basic types, slices, options and results, minimal formatting | What fits in a microcontroller; what a Z80 could aim for |
| `std` | Everything 90% of programs need: text and Unicode, collections, math, time, files, networking and HTTP, JSON, TOML, CSV, XML, hashing and TLS, regular expressions, compression, processes, arguments, tests | One convention everywhere. Written in Shrimp. Precompiled, linked on demand: a big `std` costs a small binary nothing |
| `x/` | Official incubator: same tooling, same governance, allowed to break | The waiting line into `std`; what is never promoted gets archived |

Models: `std` in the style of Go (a production server with no
dependencies), consistency in the style of WLangage (one prefix per
family, the same parameter order everywhere — here, modules by theme,
the modern form of the same gesture), incubator in the style of
`golang.org/x/` (`context`, `slices`, `maps`, `log/slog` were born
there), antechamber in the style of Boost → C++.

Counter-models: PHP (huge and inconsistent: `strpos(haystack, needle)`
but `array_search(needle, haystack)`), Python (`urllib` so painful that
everyone installs `requests`; nineteen dead modules removed in 3.13),
Java (`java.util.Date` broken since day one and still there).

## 3. The single convention of `std`

To be written at step 4, before the first function. At a minimum:
- naming: one word per concept, the same everywhere;
- parameter order: the subject first, always;
- one error type per family, propagated the same way;
- documentation: one line per entry, which fits in the index.

## 4. Promotion from `x/` to `std`

Criteria, all required:
1. API unchanged for several years;
2. a single obvious design — no promotion while two schools are fighting
   (Rust's async executors);
3. maintainers who hand over the keys;
4. no dependency outside `std`;
5. tests and documentation at the level of `std`;
6. **rewrite to the conventions of `std` on entry**: this is where
   "well made" becomes "consistent".

Exit door: **editions**. Nothing is ever removed; an edition hides what
is deprecated. Go never removes anything (hence its severity at the
door), Python removes painfully, editions are the compromise.

## 5. The index

`std` ships with its machine-readable reference: one entry per function,
type or constant — module, name, signature, one line, theme, edition of
first appearance. A file the AI loads to check a signature instead of
inventing it, and which the tooling generates from the sources, never by
hand.

## 6. Tooling

A single executable, `shrimp`: `build`, `run`, `test`, `fmt`, `doc`,
`index`. No separate `shrimpc`, no choice of formatter. Source
extension: `.shr` or `.shrimp` (`.sh` is taken).

First compiler: in Rust, with a backend chosen to get quickly to a
program that runs — emitted C, Cranelift or LLVM. Self-hosting later,
when the language deserves it.
