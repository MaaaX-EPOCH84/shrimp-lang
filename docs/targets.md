# The targets and the boundary — Shrimp

Decision of point 1.6 of the roadmap, 2026-11-09. What requirements §7
laid down as an order — Linux, Windows, macOS, mobile, then the
others — receives here what the order lacked: what "supported" means,
level by level; which targets, at which level, from when; what that
imposes on the backend; what crosses the C boundary and in which form;
and what `core` may rely on when the machine is an ESP32-C3. Seven
questions settled one at a time by MaaaX on 2026-11-09. The facts about
other tools were re-read at the source the same day; the sources are in
§9, and what is quoted from memory says so.

**Read and approved, Maxime MARCONATO (aka MaaaX^^, aka EPOCH84),
2026-11-13.** Any later change is dated and signed.

## 1. Three levels of support

"Shrimp supports Windows" means nothing until it says what is promised.
Three levels, on the model of Rust's tiers ("guaranteed to work",
"guaranteed to build", "may or may not work"):

| Level | Promise | How it is held |
|---|---|---|
| **1** | It works. The whole test suite passes on the target at every commit; a regression blocks the release | The suite runs on the target itself |
| **2** | It builds. `core`, and `std` where the target has a system, compile for the target at every commit; the tests are not necessarily run | The build runs in continuous integration, from a level 1 host |
| **3** | It exists. The target is in the source; nobody promises it compiles today | Nothing automatic |

**A target changes level by a dated decision**, written in the table of
§2, never in passing. Going up requires the mechanism of the level to
be in place and green; going down is allowed, and is said.

**A host is a target on which `shrimp` itself runs.** Level 1 implies
host: one cannot run the suite where the tool does not run.

**One script holds the levels.** A single script in the repository runs
everything a level 1 demands — build, tests, lints, measurements —
and it is the only description of the check: the developer runs it
locally before pushing (a `pre-push` hook can do it), and the remote
continuous integration does nothing but call it. Two descriptions of
the same check drift apart; one does not. Its form belongs to the
measurement protocol (point 1.8), its installation to the prototype
(step 7); `shrimp test` takes over when it exists. The remote side also
gives what MaaaX's machine cannot: at the time of writing GitHub hosts
runners for Linux, Windows and macOS, on x64 and arm64 — and none for
any BSD.

## 2. The targets

The order is that of requirements §7 and does not change. The level is
the one each target starts at; the rank says in which order the levels
will be raised.

| Rank | Target | Form | Level at the start | Aim |
|---|---|---|---|---|
| 1 | Linux, x86-64 | Host; programs and libraries | **1**, from the prototype | — |
| 2 | Windows, x86-64 | Host; programs and libraries | 3 | 1 |
| 3 | macOS, ARM64 | Host; programs and libraries | 3 | 1 |
| 4 | Android, `arm64-v8a` | Libraries only, §4 | 3 | 2, then 1 |
| 5 | iOS, ARM64 | Libraries only, §4 | 3 | 2 |
| 6 | Raspberry Pi: Linux, ARM64 | Host; programs and libraries | 3 | 1 |
| 6 | ESP32-C3: bare RISC-V, RV32IMC | `core` only, §5 | 3 | 2 |
| 6 | STM32: bare ARM Cortex-M | `core` only | 3 | 2 |
| 6 | ESP32 with an Xtensa core | `core` only | not a target yet | depends on the backend, §3 |

Outside the order, because they are not ends in themselves:

| Target | Form | Level | Why it is there |
|---|---|---|---|
| WebAssembly, `wasm32` | `core` only at first | **2, from the prototype** | A `core` without a system that can be exercised on the developer's machine; later, the examples of the online Bible run in the page (requirements §9). Not level 1: the suite does not run in a browser at every commit |
| BSD | Programs and libraries | **3 as soon as Linux is level 1, then 2 as soon as the build can be automated** | Hygiene: `std` written against "a Unix", not against Linux. FreeBSD on x86-64 first. Never in the official order; level 1 only on the day a BSD user exists |

Decided by MaaaX on 2026-11-09: three levels and Linux x86-64 alone at
level 1; `wasm32` at level 2 from the prototype; BSD at level 3 then 2;
Android before iOS.

What the table does not say and leaves to the step that needs it:
which other architectures of a system follow the first one (Linux and
Windows on ARM64, macOS on x86-64, the 32-bit Android ABI), which
WebAssembly system interface carries `std` (step 9), which Cortex-M
families exactly (step 8). Each enters at level 3, by a dated line.
The Raspberry Pi of the table is the board that runs Linux; the
microcontrollers of the same brand belong to the bare ARM line (quoted
from memory: Cortex-M0+ cores).

For scale, Rust, after eleven years of releases, holds eight targets at
its tier 1: Linux and Windows on x86-64, 32-bit x86 and ARM64, and
macOS on ARM64 alone. FreeBSD and NetBSD on x86-64, Android, iOS,
`wasm32` and the bare RV32IMC and Cortex-M targets are at its tier 2;
OpenBSD, the ESP32 Xtensa cores, AVR, MSP430 and the 68000 at its
tier 3.

## 3. Cross-compilation from the first day

**Every target is produced from every level 1 host, with nothing
installed but `shrimp`.** Decided by MaaaX on 2026-11-09. It is the
constraint of `docs/tool.md` §4 — no linker and no C toolchain to
install — extended from the host to every target, and it is the only
way to reach machines that cannot compile for themselves: an ESP32-C3
has no compiler, a phone has no shell. Go and Zig work this way: "in
effect, you are always cross-compiling" (Go); "Zig builds for all
supported targets independently of the host", with "no separate cross
toolchain" (Zig).

This is **a constraint on the backend of step 7, not a choice of
backend.** What it does to the three candidates of architecture §6, as
they stand on 2026-11-09:

| Candidate | Against this constraint |
|---|---|
| Emitted C | Needs a C compiler and a linker for every target. Falls, unless `shrimp` ships one — which is the next line with a detour |
| Cranelift | Four backends: x86-64, ARM64, s390x and 64-bit RISC-V. Covers ranks 1 to 5 and the Raspberry Pi; covers neither the ESP32-C3 (32-bit RISC-V), nor Cortex-M, nor `wasm32` — Cranelift consumes WebAssembly, it does not emit it. Produces object code and no linker |
| LLVM | ARM, ARM64, RISC-V 32 and 64, WebAssembly and x86 among its regular targets; Xtensa and the 68000 among its experimental ones; neither Z80, nor 6502, nor uPD7801. Covers the whole table. Ships a linker (`lld`). Heavy, and slow to compile with |

Step 7 chooses, and may choose two — one backend for the speed of
`check` and `run` on the host, one for the breadth of targets. What it
may not do is choose a backend that makes a target of this table
depend on a toolchain installed beside `shrimp`.

**Where "nothing but `shrimp`" stops:** at the library. Signing an
application and packaging it for a store belong to the system's own
tools — Xcode on a Mac for iOS, Gradle for Android — and stay there.

## 4. The boundary

**C interoperability in both directions is confirmed** (requirements
§7). What each guarantee becomes at the crossing is already written and
approved, `docs/guarantees.md` §2 bis: every crossing is `unsafe`,
ownership is declared, nothing unwinds through C, a callback goes
through a declared function type. This point adds what a target makes
of it.

**Two forms of output, on every target that has a system:**

| Form | What `shrimp build` produces |
|---|---|
| A program | An executable with its entry function |
| A library | A static or dynamic library **and the C header that describes it**, generated from the exported functions, never written by hand |

The C ABI is that of the target system; the representable types,
strings and callbacks are a chapter of the core language (step 3).

**A library has no `main` and no life of its own**: it does nothing
until it is called. Its exported functions are an ordinary C façade —
flat structures, C strings, integer codes — and since nothing is
hidden (G4), there is no global state behind them: the caller opens a
context, passes it to every call, and closes it. On the Shrimp side the
context is a resource like any other; what the compiler cannot do is
make the other side close it, and the header says so.

**Mobile targets are libraries, and only libraries** (MaaaX,
2026-11-09). Nearly all of an application is written in Shrimp —
data, storage, network, logic — and compiled as a library; the screens
are written in the system's own language and are a shell that calls
it. It is the consequence of the amendment of `docs/budget.md` §5: no
user interface in `std`, no common abstraction. Nothing is displayed
from Shrimp.

| | Android | iOS |
|---|---|---|
| What is shipped | One dynamic library per ABI, at `lib/<abi>/lib<name>.so` inside the package; the installer keeps the one that fits the device | Static or dynamic libraries with their headers, one variant per platform — device and Simulator apart — in an XCFramework bundle, linked into the application |
| How it is called | The application runs on a virtual machine and reaches native code through JNI only: the library is loaded by `System.loadLibrary`, and each native method is either registered from `JNI_OnLoad` or found by a name the specification imposes. Strings are UTF-16 or "Modified UTF-8", objects are references to be released | Swift calls the C functions directly once a module map names the header; no layer in between |
| What it costs Shrimp | **A JNI layer on top of the C façade**, to write or to generate | Nothing more than the header |
| What stays outside | Packaging, by Gradle | Signing and packaging, on a Mac |

The four Android ABIs are `arm64-v8a`, `armeabi-v7a`, `x86` and
`x86_64`; the first is the target, the last serves the emulator.

The JNI layer and the project templates of each system are tooling and
library, not language: they live in `x/` (step 13), with a generator to
study then — Mozilla's UniFFI does it for Rust (quoted from memory,
to be read at the source at that step). Nothing here enters the core
language.

## 5. What `core` may rely on: the ESP32-C3

Architecture §2 says `core` is "what fits in a microcontroller". The
ESP32-C3 is the witness, because it is cheap, common and takes away
everything at once:

| | ESP32-C3 (datasheet v2.4) |
|---|---|
| Processor | One 32-bit RISC-V core, up to 160 MHz |
| Instruction set | RV32IMC: integers, multiplication and division, compressed instructions. **No floating point (no F), no atomic instructions (no A)** |
| Memory | 400 KB of SRAM, 16 KB of it for the cache; 384 KB of ROM |
| Storage | Flash in the package, optional, up to 8 MB |
| System | None that the program does not bring |

Hence the list of what `core` may **not** rely on, each line being
something a target of the table lacks:

1. **No system.** No file, no clock, no thread, no process, no
   argument, no standard output.
2. **No allocation.** Nothing in `core` asks for memory at run time.
3. **No floating-point unit.** A program that uses no floating-point
   number contains no floating-point code, no table, no support
   routine for it.
4. **No atomic instruction.** Nothing in `core` requires a
   compare-and-swap from the processor.
5. **No table it did not ask for.** The minimal formatting of `core`
   pulls in no Unicode data.
6. **A pointer has the size the target gives it**, 32 bits here; the
   language assumes neither 32 nor 64.
7. **Integers of every width are first-class**, `u8` and `u16`
   included, with no silent promotion — already required by G4.

Two consequences the witness makes visible and the spec will have to
write (step 3):

- **Stopping.** G1 turns an out-of-bounds access into a stop of the
  program. Without a system there is nobody to stop it: what "the
  program stops" does on a bare target is supplied by the program or
  by the support of the target, and the spec must say where.
- **The program that never returns.** A firmware is a loop without an
  end. The rule of `docs/guarantees.md` — every resource consumed
  exactly once on every path — must say what a path that never ends
  owes.

And one question for the library (step 4): **collections on a machine
that has memory but no system.** `core` allocates nothing, `std`
assumes a system; a program on an ESP32-C3 that brings its own
allocator falls between the two. Rust answers with a tier of its own
(`alloc`); whether Shrimp's three tiers need a fourth, or a part of
`std` that requires only an allocator, is decided with the tiers.

**What is measured** (point 1.8, then step 8): the size in flash of
the empty program and of a reference program on an ESP32-C3, beside
the same in C and in Rust. "`core` fits" will be that figure, not an
adjective.

## 6. The multi-generation wish

Z80, 6502, 68000 and, added by MaaaX on 2026-11-13, the NEC uPD7801:
a wish, "kept if it costs the language no constraint, dropped
otherwise" (requirements §7). Settled by MaaaX on 2026-11-09: **no
target, no level, and nothing enters the language for it.** What these
processors require is already owed to the ESP32-C3 and to G4 — the
seven lines of §5. What they would require beyond that is refused: a
code generator of our own (no candidate backend emits Z80, 6502 or
uPD7801 code), a memory model bent around a 256-byte stack, vocabulary
for zero pages and memory banks. The day a backend exists, one of them
enters the table at level 3, by a dated decision. The wish is honoured
by the discipline of `core`, not by a promise.

## 7. Set aside

| Option | Why not |
|---|---|
| Two levels, tested and built | No place for "exists, no promise", which is exactly the status a microcontroller needs for years |
| Several targets at level 1 from the start | A level 1 is a blocker; one cannot hold a blocker on a machine nobody runs |
| A remote workflow that describes its own steps | Two descriptions of one check; they drift apart |
| Cross-compilation later | Every bare target would wait for a native toolchain that does not exist; the tool would be designed without it and redone |
| WebAssembly at level 3 | Promises nothing and prepares nothing for the Bible; the cost at level 2 is a build |
| WebAssembly at level 1 | The suite in a browser at every commit |
| BSD in the official order, or at level 1 | No user, and no hosted runner; a blocker without a beneficiary |
| BSD off the list | Loses the one cheap benefit: a `std` that says Unix where it means Unix |
| Mobile applications written entirely in Shrimp | Requires the common user-interface abstraction refused in `docs/budget.md` §5, or two system frameworks wrapped whole |
| iOS before Android | No Mac at hand; Android is checked from Linux |
| A level, even 3, for Z80, 6502, 68000 and uPD7801 | A line in the table is a promise that the source contains the target; it does not |
| Choosing the backend here | Step 7; here only what it must satisfy |

## 8. Deferred, with the step that decides

| What | Step |
|---|---|
| The check script, its content and the measurements it runs; size of `core` on an ESP32-C3 against C and Rust | 1.8 |
| The C boundary in the spec: ABI, representable types, strings, callbacks, exported functions and the generated header; what stopping means on a bare target; resources on a path that never ends | 3 |
| Collections with an allocator and no system: a fourth tier or a part of `std` | 4 |
| The backend or backends, the linker inside `shrimp`, the first level 2 builds, the continuous integration installed | 7 |
| `core` on the bare targets: start-up, target support, exact Cortex-M families | 8 |
| `std` on WebAssembly and its system interface; `std` on Windows, macOS, BSD | 9 |
| `shrimp build --target`, the names of the targets on the command line, the generated header as a verb or an option | 10 |
| The JNI layer, the project templates for Android and iOS, a binding generator | 13 |

## 9. Sources re-read on 2026-11-09

- https://doc.rust-lang.org/rustc/platform-support.html — the three
  tiers and their wording; the eight tier 1 targets; the tier of each
  target named in §2.
- https://github.com/bytecodealliance/wasmtime — `cranelift/README.md`
  ("Cranelift has four backends: x86-64, aarch64 (aka ARM64), s390x
  (aka IBM Z) and riscv64") and `cranelift/codegen/src/isa/mod.rs`,
  branch `main`.
- https://github.com/llvm/llvm-project — `llvm/CMakeLists.txt`, branch
  `main`: `LLVM_ALL_TARGETS` and `LLVM_ALL_EXPERIMENTAL_TARGETS` (ARC,
  CSKY, M68k, Xtensa); listing of `llvm/lib/Target` (neither Z80, nor
  6502, nor uPD7801) and of the root (`lld`).
- https://www.espressif.com/sites/default/files/documentation/esp32-c3_datasheet_en.pdf
  — ESP32-C3 Series Datasheet, version 2.4: processor, RV32IMC, memory.
- https://docs.github.com/en/actions/reference/runners/github-hosted-runners
  — hosted runners: Linux, Windows, macOS, x64 and arm64; no BSD.
- https://developer.android.com/ndk/guides/abis — the four ABIs, the
  path `lib/<abi>/lib<name>.so`, extraction at install time.
- https://developer.android.com/training/articles/perf-jni —
  `System.loadLibrary`, `RegisterNatives` from `JNI_OnLoad` or lookup
  by name, UTF-16 and Modified UTF-8.
- https://developer.apple.com/documentation/xcode/creating-a-multi-platform-binary-framework-bundle
  — what an XCFramework contains, `-library` and `-headers`, signing.
- https://www.swift.org/documentation/articles/wrapping-c-cpp-library-in-swift.html
  — "Swift can't import header files directly": the module map, then
  the C functions called as they are.
- https://go.dev/doc/install/source — `GOOS` and `GOARCH`, "in effect,
  you are always cross-compiling".
- https://ziglang.org/learn/overview/ — "Cross-compiling is a
  first-class use case".

Not re-read, and marked as such in the text: the cores of the
Raspberry Pi microcontrollers, UniFFI. Not verified by execution:
nothing here has been compiled for any target; the first proof is the
prototype's.
