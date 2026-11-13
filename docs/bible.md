# The shape of the Bible — Shrimp

Decision of point 1.7 of the roadmap, 2026-11-13. Requirements §9 asks
for one book — the complete reference of the language, in English,
online and as PDF from one source, an example at every entry, the
index generated from it — and says nothing of its shape. Here: how
many books, in what source format, with which conventions, where in
the repository, what renders it and when, and how it is translated.
Eight questions settled by MaaaX on 2026-11-13, on the assistant's
recommendations, in one answer. The facts about other tools were
re-read at the source the same day (§11); what was measured was
measured with `tools/tokcount` (§2), and the form was tried on a
witness (§10).

**Read and approved, Maxime MARCONATO (aka MaaaX^^, aka EPOCH84),
2026-11-13.** Any later change is dated and signed.

## 1. Three books and the appendices

| Part | Content | Written at | How the AI loads it |
|---|---|---|---|
| **Book I** | The core language: normative text, grammar, examples | Step 3 | The trunk always, a section when the program needs it (§5); under the ceiling of 100,000 tokens (`docs/spec_size.md`) |
| **Book II** | The library, `core` then `std`, by theme; every entry with its example (requirements §9) | Step 5, then completed as the library is written (steps 8, 9 and 13) | Through the index first, then one theme at a time |
| **Book III** | The tooling: `shrimp`, its verbs, the Unix tools shipped inside, the editor extension | Step 10 | One page per verb or tool, the one `--help` prints (`docs/tool.md` §6) |
| **Appendices** | The editions and what each one hides; the measurements of requirements §8; the index of `std` | Steps 4, 7 and 10 | On demand; outside the ceiling |

Book I is alone under the ceiling: it is the only one the AI re-reads
at every session. The edition appendix remains normative for the
earlier editions (`docs/budget.md` §3); the index is generated, never
written (`docs/architecture.md` §5).

**The incubator `x/` is not in the Bible.** A library that may still
break documents itself where it lives, in the same format and under the
same conventions; it enters Book II on the day it is promoted, and
"documentation at the level of `std`" is already one of the criteria of
that promotion (`docs/architecture.md` §4).

## 2. The source: Markdown, a closed subset

**The source is the Bible.** What the AI loads is the source file
itself, not a rendering of it, and what `tokcount` counts against the
ceiling is that file. The web pages and the PDF are renderings for the
human reader; they add nothing and hide nothing.

**The format is Markdown**, for four reasons, in this order:

1. **It reads raw.** When the tool is down the ballpoint pen remains
   (requirements §2): a Markdown file opened in any editor is already
   the book.
2. **It is rendered today**, by the site that hosts the repository,
   years before `shrimp doc` exists (§6).
3. **A closed subset can be parsed by hand.** `shrimp doc` is written
   with as few third-party crates as possible, because it will be
   rewritten in Shrimp (`docs/tool.md` §1); the format must be one that
   a few hundred lines read.
4. **It costs no more tokens than the others.** Measured, not supposed
   — the assistant expected Markdown to be the lightest; it is not, the
   three light formats tie:

| Format | o200k_base | cl100k_base | Qwen3 | DeepSeek-V3 | Worst |
|---|---:|---:|---:|---:|---:|
| Typst | 393 | 394 | 431 | 394 | 431 |
| AsciiDoc | 395 | 395 | 432 | 407 | 432 |
| Markdown | 397 | 397 | 434 | 395 | 434 |
| HTML | 568 | 558 | 595 | 577 | 595 |

One page of specification — two paragraphs, a table, a list, a grammar,
two programs, a note — written four times with the same text, counted
by `tools/tokcount` on 2026-11-13. One page, written by the assistant:
it says that tokens do not separate the light formats and that HTML
costs about 37% more; it says nothing finer.

**A closed subset, defined by the project.** Markdown has several ways
to write most things; the Bible admits one, on the reasoning of the
grid (`docs/budget.md`, Q4: one way only). The first list:

| Admitted | Refused |
|---|---|
| Headings with `#` | Underlined headings |
| Paragraphs, `*emphasis*`, `**strong**`, `` `code` `` | Raw HTML, in any form |
| Lists with `-` and with `1.` | Lists with `*` or `+` |
| Tables with `\|` | Indented code blocks |
| Fenced blocks, always labelled (§3) | A fenced block without a label |
| Links written in place, relative inside the Bible | Reference-style links, footnotes |
| A note as a quoted block | Images in Book I |

The list is closed at step 3, with the first chapter of Book I: that
is when a missing construct shows. From then on a construct enters by
a dated decision, and the tool refuses a file that steps outside the
subset.

## 3. The conventions, fixed from the start

The reference of Rust is written in Markdown and had to grow a
preprocessor of its own to become a specification (§11). The lesson is
taken before the first line: three conventions are part of the format.

**Every fenced block carries a label**, and the label says what the
tool does with the block. The first labels — the list is closed at
step 3:

| Label | What the block is | What the tool does |
|---|---|---|
| `shr` | A complete program | Compiles it; the release fails if it is refused |
| `shr rejected`, followed by a rule | A complete program that must be refused | Compiles it; the release fails if it is accepted, or refused for another rule |
| `ebnf` | A production of the grammar | Assembles the grammar of the language from these blocks and from nothing else |
| `text` | What a program prints, what a command shows | Nothing |

**What is compiled is what is read.** No hidden line, no prelude the
reader does not see, no included file: the block in the page is the
whole program. That is G4 applied to the examples, and it is why the
examples live in the text and not beside it — a file included from
elsewhere would be a text the ceiling counts and the AI does not read,
or reads and the ceiling does not count.

**Every normative rule carries a stable identifier.** The messages of
the compiler, the tests, the audits and the translations cite a rule,
not a page or a heading. An identifier is never renumbered and never
reused: what is true of a program — what compiles today compiles
tomorrow — is true of a reference to the specification. Its form is
fixed at step 3, and its cost in tokens is measured when the form is
chosen: it is paid under the ceiling.

## 4. The place in the repository

```text
bible/
  book1/              the core language
    trunk.md          always loaded; carries the index of the sections
    <section>.md      one file per section
  book2/              the library, one file per theme
  book3/              the tooling, one file per verb or tool
  appendices/
  translations/
    <language>/       a mirror of the four directories above
```

**`bible/` at the root, beside `docs/`.** `docs/` holds the decisions
of the project — why the language is what it is; `bible/` holds the
language. The directory is created at step 3 with its first file, not
before.

**In the same repository as the compiler.** A rule and the code that
enforces it change in the same commit, and the check script of
`docs/targets.md` §1 compiles every example of the Bible at every
commit. The English text is at the top of the tree and the
translations are under it: the layout says which one is authoritative.

**One file is one unit of loading.** A section of Book I, a theme of
Book II, a verb of Book III: the AI loads a file or does not, and
`tokcount` counts per file and in total without learning anything.

## 5. Book I: the form now, the split at step 3

`docs/spec_size.md` §5 sent three things here: the split into trunk and
sections, the list of the sections, the figure for the trunk. **This
point fixes the form and sends the list and the figure on to step 3**,
by a dated decision once the text exists. A split decided before the
text is a guess, and a figure for a trunk nobody has written is the
guessed margin refused at point 1.3. `docs/spec_size.md` §5 is amended
accordingly, dated and signed.

The form:

- **one file for the trunk, one file per section**;
- **the trunk opens on the index of the sections**, one line each: what
  the program must contain for the section to be needed, and the file
  to load;
- **the three rules of `docs/spec_size.md` §5 stand**: a section is
  self-contained, the trunk carries the index, the ceiling is on the
  whole;
- **the figure of the trunk is the count of its file**, the ceiling is
  the total of `bible/book1/`: one command gives both.

## 6. Rendering: nothing before `shrimp doc`

Until step 10 the Bible is read as Markdown, raw or as the hosting site
renders it. **No temporary chain**: a third-party renderer adopted at
step 3 would be thrown away at step 10, and in between it would define
the format in place of the subset — whatever it accepts would become
the Bible.

At step 10, `shrimp doc` produces the web pages and the PDF from the
same files (`docs/tool.md` §6). Requirements §10 — what is flagged on
screen is flagged on paper, the `unsafe` blocks first — is held there
by construction: the renderer is the tool that contains the compiler.
How a PDF is written without a third-party chain is step 10's
question.

Two consequences, said rather than hidden:

- **there is no PDF before step 10**, and no site of our own;
- **no example is compiled before step 7.** Book I is written and
  frozen at step 3, when no compiler exists: its examples are checked
  by reading only, and the first mechanical proof is the prototype's.
  The audit of step 11 — the prototype against the specification — is
  where every example of Book I meets a compiler for the first time.

## 7. Translation

The English text is authoritative (requirements §9). Four rules:

1. **Not before step 14, and book by book once frozen.** A text that
   still moves is translated twice.
2. **The prose only.** Every labelled block is copied byte for byte,
   comments included, and every rule identifier is kept: one set of
   examples to compile, and a reader of Shrimp reads English comments
   in every program anyway. The tool checks it — a translated file
   whose blocks or identifiers differ from the English ones is refused.
3. **Every translated file names the English text it comes from**: on
   its first line, visible, the path of the English file and its
   fingerprint. When the English file changes, the fingerprint no
   longer matches, the translation is shown with a banner saying it is
   behind — and it never blocks a release. A file not yet translated is
   served in English.
4. **A translation nobody has re-read says so**, on the same first
   line. Written by an AI and unread by a human who speaks the language
   is a status, not a defect; hiding it would be one.

A translation of Book I is never what the AI loads and is never counted
against the ceiling. Which languages, and in what order, is decided at
step 14.

## 8. Set aside

| Option | Why not |
|---|---|
| Two books, the tooling as a chapter of the library | `--help` loads one page per tool; the tooling has its own unit of loading and its own step |
| `x/` inside Book II | The Bible would describe what is allowed to break |
| Typst | Its HTML export is behind a feature flag — "Do not use this feature for production use cases" (§11); and a typesetting language is not rewritten in Shrimp |
| AsciiDoc | Richer than needed; its PDF converter is a Ruby program, and its grammar is too large to parse by hand |
| HTML written by hand, as the specification of Go is | About 37% more tokens on the witness page, and the worst to read raw |
| A format of our own | One more language to specify, and nothing renders it today |
| Markdown as a renderer defines it | Several ways to write each thing, and the renderer becomes the definition |
| A temporary renderer until step 10 | Thrown away at step 10; defines the format meanwhile |
| A rendering tool written in Rust at step 3 | A second tool to write and rewrite, for a book that reads raw |
| Examples as `.shr` files included in the text | The file the AI loads would not contain what it must read; hidden lines, by another door |
| Hidden lines in the examples | The reader does not see what compiles |
| The Bible in `docs/` | Mixes the decisions of the project with the reference of the language |
| The Bible in a repository of its own, as the reference of Rust is | A rule and its enforcement could no longer change in the same commit |
| The list of the sections and the figure of the trunk fixed now | Guessed before the text exists |
| Translating as the text is written | Translated twice; nothing is frozen before 1.0 |
| Translated comments in the examples | One set of examples per language to compile; and the comments of real programs are in English |
| Message catalogues, one entry per paragraph, as the documentation of Python uses | A third-party chain and a second file format; one fingerprint per file answers the same question |
| A late translation blocks the release | The release would wait on the least maintained language |
| Rules cited by section number | A number moves when a section is inserted |

## 9. Deferred, with the step that decides

| What | Step |
|---|---|
| The closed list of the Markdown subset and of the labels; the form of a rule identifier and its cost in tokens; the split of Book I, the list of its sections, the figure of the trunk | 3 |
| The edition appendix; the format of the index and of an entry of Book II | 4 |
| The map of the themes of Book II, one file each | 5 |
| The examples compiled at every commit by the check script; the first mechanical proof of Book I | 7 |
| `shrimp doc`: the web pages, the PDF, the check of the subset, the check of the translations, the banner; `--help` from Book III; the examples run in the page (`docs/targets.md` §2) | 10 |
| Every example of Book I against the prototype | 11 |
| The assembly, the generated index, the languages of translation and their order | 14 |

## 10. What was tried

A witness outside the repository, on 2026-11-13: a `bible/book1/` of
three files — a trunk with its index of sections, two sections —
carrying labelled blocks and rule identifiers in a placeholder form,
and one translated file. With `tools/tokcount` and the standard Unix
tools, nothing else:

- `tokcount count bible/book1/*.md` gives the count of each file and
  the total: the figure of the trunk and the figure under the ceiling
  come out of one command;
- ten lines of `awk` extract every labelled block to a file named
  after its label, the rejected program with the rule it names;
- the block of the translated file compares identical to the English
  one, and one byte changed in it is caught;
- the fingerprint recorded in the translated file matches, and stops
  matching after one sentence is added to the English file.

What this proves: the form is mechanical, and a tool written by hand
is enough. What it does not: nothing was compiled, there is no
compiler; and the witness recorded the fingerprint in an HTML comment,
which §2 refuses — hence the visible first line of §7.

## 11. Sources re-read on 2026-11-13

- https://github.com/rust-lang/reference — `book.toml`, branch
  `master`: an mdBook book, `[preprocessor.spec]` run from
  `tools/mdbook-spec`, `tools/grammar` watched beside it; the listing
  of `dev-guide/src` (`rules/`, `grammar.md`, `examples.md`,
  `tests.md`). The content of those pages was not read.
- https://rust-lang.github.io/mdBook/format/configuration/renderers.html
  — two built-in renderers, `html` and `markdown`; no PDF, only a
  printable page of the HTML output.
- https://rust-lang.github.io/mdBook/format/mdbook.html — `mdbook
  test`, `{{#include}}`, `rustdoc_include`, and the hidden lines: "lets
  you hide code lines by prepending them with a specific prefix".
- https://github.com/golang/go — `doc/go_spec.html`, branch `master`:
  one HTML file of 9,079 lines, the grammar in `<pre class="ebnf">`.
  That it is written by hand is quoted from memory.
- https://github.com/ziglang/zig — listing of `doc/`:
  `langref.html.in` and a directory `langref/`. The content of the
  directory was not read.
- https://typst.app/docs/reference/html/ — "The feature is still very
  incomplete and only available for experimentation behind a feature
  flag."; "Do not use this feature for production use cases." The
  version on the page is 0.15.1.
- https://docs.asciidoctor.org/pdf-converter/latest/ — "a native PDF
  converter for AsciiDoc", built on the Prawn gem, no intermediate
  format.
- https://djot.net/ — a lighter Markdown considered and not retained:
  "minor future changes to the syntax are expected".
- https://devguide.python.org/documentation/translations/translating/
  — "gettext PO file catalogs generated from the Python docs by
  Sphinx", one repository per language, untranslated messages falling
  back to English.
