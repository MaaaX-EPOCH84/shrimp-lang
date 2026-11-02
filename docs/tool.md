# The tool, the extension and the identity — Shrimp

Decision of point 1.5 of the roadmap, 2026-10-22 to 2026-11-01. What
`docs/architecture.md` §6 and `docs/name.md` proposed on 2026-10-20 is
settled here, one question at a time: the single tool and its verbs,
the file extension, the layout of a project, running from source, the
Unix tools shipped inside `shrimp`, the help, the mascot and the logo,
and the collisions checked. Every decision is MaaaX's, dated.

**Read and approved, Maxime MARCONATO (aka MaaaX^^, aka EPOCH84),
2026-11-01.** Any later change is dated and signed.

## 1. One tool, `shrimp`

**A single executable, `shrimp`, and nothing beside it.** No `shrimpc`,
no separate formatter, no separate package manager, no separate test
runner. Decided on 2026-10-20 (requirements §7), confirmed on
2026-10-22.

**Eight verbs**, fixed on 2026-10-22:

| Verb | What it does |
|---|---|
| `shrimp new` | Creates an empty project: manifest, `src/main.shr` |
| `shrimp check` | Checks a project — types, guarantees G1 to G5, every case covered, every resource consumed — without producing anything. The command the AI runs most, in a loop; several times faster than `build` |
| `shrimp build` | Produces the binary |
| `shrimp run` | Runs a program from its source, §4 |
| `shrimp test` | Runs the tests |
| `shrimp fmt` | Formats, one way only |
| `shrimp doc` | Renders the documentation — the Bible and the help of every verb, from one source, §6 |
| `shrimp index` | Generates the machine-readable index of `std` (architecture §5) |

Step 10 may add verbs; it may never remove one. What is not here:
dependency management (the mechanism waits for the three library tiers,
step 4; what is fixed is that it lives in `shrimp` too, never in a tool
of its own), the language server (requirements §10, step 10), and the
Unix tools, §5.

**`shrimp` itself carries as few third-party dependencies as possible**
(MaaaX, 2026-10-22). The reason is not purity: the tool will be
rewritten in Shrimp one day, and every third-party crate it leans on is
one more thing to rewrite. `tools/tokcount/` already lives by this rule
with three crates.

## 2. The extension: `.shr`

**`.shr`**, decided on 2026-10-22. Three letters like `.rs`, `.go` and
`.zig`; `.sh` is taken by the shell. `.shrimp` was the alternative,
readable without context but twice as long as any neighbour; it would
have won only if `.shr` had belonged to a widespread format. It does
not: `.shr` is used by `shar` shell archives (an old variant of
`.shar`), by Signal Hound spectrum sweeps and by a handful of dead
formats, none of which is a plausible neighbour in a source tree
(checked on 2026-10-23, `docs/name.md`).

## 3. The layout of a project

Decided on 2026-10-23. Names reserved now because `shrimp new` writes
them at step 10; their content waits for step 4.

| File | What |
|---|---|
| `shrimp.toml` | The manifest: name, version, targets, dependencies. Lower case, against the `Cargo.toml` capital: MaaaX's choice |
| `shrimp.lock` | The resolved versions, so that "what compiles today compiles tomorrow" also holds for dependencies |
| `src/` | The sources |
| `src/main.shr` | The entry point |

**TOML**, the format of Rust and Python: readable, typed, commentable,
one way to write each thing. YAML set aside (significant indentation,
`no` that becomes `false`); JSON set aside (no comments).

## 4. Running from source

**`shrimp run file.shr` compiles if needed, caches the binary and runs
it.** Decided on 2026-10-22, the `go run` model, with MaaaX's three
conditions:

- **No file next to the source, ever.** The cache lives outside the
  project, in the user's directory, and `shrimp` knows how to empty it.
- **No explicit step.** `build` exists for whoever wants the binary;
  `run` does not require it.
- **Same front end, same checks.** A script is an ordinary program with
  its entry function; nothing is relaxed for it. There is no
  "statements at file level" mode: that would be a dialect, and the AI
  would learn to write fast in the wrong one.

**The shebang** is a tolerance of the parser and an installation
convention, not a feature: a first line starting with `#!` is ignored.
It is the only line this adds to the spec. Which form —
`#!/usr/bin/env -S shrimp run`, which needs an `env` that splits
arguments, or `#!/usr/bin/env shrimp` with `shrimp` treating a first
argument ending in `.shr` as `run` — and the `.shr` file association on
Windows, where the kernel ignores shebangs, are fixed at step 10.

**Deferred to step 7, as a constraint on the backend:** the implicit
compilation must require nothing but `shrimp`, no linker and no C
toolchain to install, or "running from source" breaks on a bare Windows
machine. An interpreter over the compiler's intermediate representation
is one way to meet it, a backend with its own linker is another; the
choice belongs with the backend. **Measured at point 1.8:** the latency
between the keystroke and the first line of output, source cached and
source modified.

## 5. The Unix tools inside `shrimp`

The AI must find the same `grep`, `sed`, `find` and company whatever
the system, and they must answer the same: an AI programs well where
nothing moves (MaaaX, 2026-10-21). Shipped inside `shrimp`, never
assumed present on the machine. Form and base decided on 2026-10-22:

- **Sub-commands**, `shrimp grep`, `shrimp sed`, never a shell. A shell
  is a second language — grammar, pipes, quoting, exit codes — to
  specify, freeze and document, and it is exactly the kind of untyped
  logic Shrimp exists to replace.
- **Composition by options**, not by pipes: native recursion
  (`grep -r`, `find --exec`), globbing carried by the tool itself (as on
  Windows, where the shell expands nothing), `-o` for the output, and
  `shrimp xargs` for a list of files fed to a command. The machine's own
  shell can still pipe two sub-commands; the common cases should not
  need it.
- **Logic goes in a `.shr` file** run by `shrimp run`, §4 — ten typed,
  checked lines instead of a loop in a shell.
- **Written by hand, GNU semantics on a bounded subset**, measured
  against the GNU test suites. `uutils` and `ripgrep` set aside: dozens
  of transitive crates, two regular expression engines, and `ripgrep`
  is not `grep` and does not claim to be (its options and default
  output differ, which is its value and its defect here). The
  dependency rule of §1 decides.
- **Set aside:** delegating to the system tool when present and
  embedding one otherwise. It brings back exactly what this point
  removes: a BSD `sed` on macOS that does not answer like the GNU one.

The list of tools and the options of each — which options of `sed`,
which regular expression syntax, for that is where the real cost is —
are the work of step 10. One known trap for then: PowerShell transcodes
text that crosses a pipe between two native commands, with surprises on
UTF-8 (quoted from memory, to verify).

## 6. The help and the Bible, one source

**`shrimp <verb> --help` is the documentation of the verb**, and
`shrimp help <verb>` is its alias. The text of `shrimp grep --help` and
the tooling chapter of the Bible come out of the same file, exactly as
the index of `std` and Book II do (requirements §9: nothing is written
twice); `shrimp doc` renders the same text as HTML and PDF. For the AI,
`--help` is even the form it will load: one page per tool, on demand —
the self-contained section of `docs/spec_size.md` §5, applied to the
tool.

## 7. The identity

**The mascot is a mantis shrimp**, validated on 2026-10-23: the fastest
strike in the animal kingdom, the most complex eyes. **The drawings are
MaaaX's**, 2026-11-01, in `assets/`: the mascot, the icon, and the
small sizes redrawn because a whole mantis shrimp becomes a coloured
blot under 48 px — a compact shrimp at 32 and 24, a shrimp drawn pixel
by pixel at 16. Windows and macOS containers assembled from them; the
macOS one is not yet tested on a Mac. Inventory in `assets/README.md`.
Until a wordmark exists, the figlet logo of the source file headers
remains the only text logo.

## 8. Collisions

Checked on 2026-10-23, read-only, against the live registries; the
table is in `docs/name.md`. Nothing blocks the name. `shrimp-lang` is
free everywhere that matters — GitHub organization, GitLab, crates.io,
npm, the domains `shrimp-lang.org`, `.dev` and `.com` — and is the form
to use; `shrimp` alone is taken on every registry by placeholders or
unrelated projects. One homonym is a language: a toy on PyPI, without a
trace elsewhere. Reservations are MaaaX's own gesture, outside this
repository.

## 9. Set aside

| Option | Why not |
|---|---|
| `.shrimp` | Twice as long as any neighbour, for a readability the icon already gives; would have won only against a widespread `.shr` |
| `Shrimp.toml`, `Shrimp.lock` | The Cargo capital; MaaaX prefers lower case |
| YAML, JSON for the manifest | Significant indentation and `no` → `false`; no comments |
| A separate package manager, a separate `shrimpc` | One tool, one chain (requirements §7) |
| A shell inside `shrimp` | A second language to specify; what pipes give, options and `xargs` give without it |
| `uutils`, `ripgrep` as a base | The dependency rule of §1; `ripgrep` is not `grep` |
| System tools when present | Different answers per system, the thing being removed |
| A source-level interpreter for scripts | Two semantics for one language |
| A "statements at file level" script mode | A dialect |
| A mantis shrimp at every icon size | Unreadable under 48 px; redrawn instead |

## 10. Deferred, with the step that decides

| What | Step |
|---|---|
| Content of `shrimp.toml` and `shrimp.lock`, dependency mechanism, registry | 4 |
| Backend; `run` without any external toolchain | 7 |
| Latency of `run`, cached and modified | 1.8 |
| List of the Unix tools and their options; shebang form; Windows file association; further verbs | 10 |
| Language server, VS Code and VSCodium extension (requirements §10) | 10 |
| Wordmark; testing `shrimp.icns` on a Mac | when a Mac is at hand |
