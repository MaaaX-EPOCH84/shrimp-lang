# License — Shrimp

Decision of point 1.4 of the roadmap, 2026-10-22. The repository was
private until now: without a license, a public repository is "all rights
reserved". Here: the license chosen, what it covers, why, and what was
set aside.

**Read and approved, Maxime MARCONATO (aka MaaaX^^, aka EPOCH84),
2026-10-22.** Any later change is dated and signed.

*Translated into English on 2026-10-22. The French text approved at commit
888832d remains the reference until this translation is re-read.*

## 1. Decision

**Dual license, MIT or Apache-2.0 at the recipient's choice**, like Rust.
Decided by MaaaX on 2026-10-22 between the two options open since
2026-10-20, GPLv3 and MIT/Apache-2.0.

- `LICENSE-MIT`: the reference MIT text (SPDX), holder "2026
  MaaaX-EPOCH84 and the Shrimp contributors".
- `LICENSE-APACHE`: the reference Apache-2.0 text, a byte-for-byte copy
  of `apache.org/licenses/LICENSE-2.0.txt` (SHA-256
  `cfc7749b…523d30`, identical to Debian's
  `/usr/share/common-licenses/Apache-2.0`).
- `THIRD_PARTY.md`: the four frozen vocabularies of `tools/tokcount/`,
  redistributed under their own license (MIT for OpenAI and DeepSeek,
  Apache-2.0 for Qwen).
- `Cargo.toml` of `tokcount`: `license = "MIT OR Apache-2.0"`.
- Every source file carries at its head the license notice and the line
  `SPDX-License-Identifier: MIT OR Apache-2.0`, in the repository's
  common header (model: `tools/tokcount/src/main.rs`).

## 2. Scope

**One license for everything**: compiler, `core`, `std`, tooling, the
design documents in `docs/`, and the Bible to come. Decided by MaaaX on
2026-10-22. Rust does the same: `rust-lang/book` and
`rust-lang/reference` carry the same two files as the compiler
(verified on 2026-10-22 on the repositories). A separate documentation
license (CC BY 4.0) would have been one more rule to explain, for no
gain: both texts already cover "software and associated documentation
files".

The holder names the author without closing the door: "MaaaX-EPOCH84 and
the Shrimp contributors". No assignment of rights is asked of
contributors; each keeps their own, like Rust (`COPYRIGHT`).

## 3. Why not GPLv3

The decisive argument is about the language, not the compiler. `core` and
`std` are linked into every Shrimp program. Under GPLv3, every compiled
program would be a derivative work of the library, hence GPL, barring a
written exception. GCC has lived with such an exception from the start:
the *GCC Runtime Library Exception*, version 3.1 of 31 March 2009, which
allows propagation of target code linked to `libgcc` and company under
other terms (verified on 2026-10-22 on gnu.org). That is one more legal
text to write, to maintain and to get understood, the opposite of
"nothing hidden".

For a compiler alone, GPLv3 would have been defensible; it is the choice
of `foregrampm`. But Shrimp is not a tool, it is a language: what matters
is what the programs written with it can become, and the answer must be
"anything".

## 4. Why both and not just one

- **Apache-2.0** brings the explicit patent grant (§3) and the
  termination clause in case of a patent infringement suit. MIT says
  nothing about it.
- **MIT** remains compatible with code under GPLv2 only, which
  Apache-2.0 is not according to the FSF. The Linux kernel is under GPLv2.
- Leaving the choice to the recipient costs one more file and not one
  more sentence in the language. Rust has proven it, and the Cargo
  ecosystem records it in a single SPDX expression,
  `MIT OR Apache-2.0`, which `tokcount` now carries.

## 5. Set aside

- **GPLv3 alone**: §3.
- **MIT alone or Apache-2.0 alone**: §4.
- **Separate documentation license**: §2.
- **MPL-2.0**: per-file copyleft, a defensible middle ground for a
  library, but Shrimp's interest is not in retaining fixes, it is in
  being read and verified; and it is a third family to explain.
- **Rust-style `COPYRIGHT` file**: pointless as long as the repository
  has a single author and four third-party files; `THIRD_PARTY.md` is
  enough, to be revisited if REUSE tooling becomes necessary.

## 6. Immediate consequence

The GitHub repository goes public as soon as this point is committed and
pushed, with at the head of `README.md` the warning that the project is
under design and that nothing is implemented (MaaaX,
2026-10-22).

## Sources

- https://github.com/rust-lang/rust — `README.md` §License,
  `LICENSE-MIT`, `LICENSE-APACHE`, `COPYRIGHT`, read on 2026-10-22.
- https://github.com/rust-lang/book and
  https://github.com/rust-lang/reference — roots listed on
  2026-10-22, `LICENSE-MIT` and `LICENSE-APACHE` present.
- https://www.apache.org/licenses/LICENSE-2.0.txt — downloaded on
  2026-10-22, fingerprint above.
- https://spdx.org/licenses/MIT.txt — downloaded on 2026-10-22.
- https://www.gnu.org/licenses/gcc-exception-3.1.en.html — read on
  2026-10-22.
- https://www.gnu.org/licenses/license-list.en.html#apache2 — read on
  2026-10-22: "not compatible with GPL version 2", because of the
  patent termination and indemnification clauses.
