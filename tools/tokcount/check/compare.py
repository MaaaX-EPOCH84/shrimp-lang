#!/usr/bin/env python3
# ================================================================
#        _____ __         _
#       / ___// /_  _____(_)___ ___  ____
#       \__ \/ __ \/ ___/ / __ `__ \/ __ \
#      ___/ / / / / /  / / / / / / / /_/ /
#     /____/_/ /_/_/  /_/_/ /_/ /_/ .___/
#                                /_/
#
# Shrimp — Small core, jumbo library.
# A programming language for two readers:
#      the human who reads, the AI who writes.
# ================================================================
# /tools/tokcount/check/compare.py
# ================================================================
# Checks tokcount against the reference implementations of its
# vocabularies, tiktoken and tokenizers, on the same texts.
# ================================================================
# Author: Maxime MARCONATO (aka MaaaX^^, aka EPOCH84)
# ================================================================
# License: MIT OR Apache-2.0, at your option — see LICENSE-MIT
# and LICENSE-APACHE.
# SPDX-License-Identifier: MIT OR Apache-2.0
# ================================================================

"""Checks tokcount against the reference implementations of its vocabularies.

Every text given on the command line is counted twice: by tokcount, and by
tiktoken (o200k_base, cl100k_base) or Hugging Face tokenizers (qwen3,
deepseek_v3) loaded from the original files. A single token of difference
is a failure: the point of the tool is to reproduce the reference counts
exactly, not approximately.

    python3 -m venv venv && venv/bin/pip install tiktoken==0.14.0 tokenizers==0.23.3
    venv/bin/python check/compare.py --qwen3 Qwen3-8B/tokenizer.json \
        --deepseek DeepSeek-V3/tokenizer.json corpus/*.txt corpus/*.md
"""

import argparse
import subprocess
import sys
from pathlib import Path

import tiktoken
from tokenizers import Tokenizer

CRATE = Path(__file__).resolve().parent.parent


def tokcount(files):
    out = subprocess.run(
        ["cargo", "run", "--release", "--quiet", "--manifest-path", CRATE / "Cargo.toml", "--", "count", *files],
        check=True,
        capture_output=True,
        text=True,
    ).stdout
    rows = [line.strip("|").split("|") for line in out.splitlines() if line.startswith("|")]
    names = [c.strip() for c in rows[0][1:-1]]
    counts = {}
    for row in rows[2:]:
        cells = [c.strip() for c in row]
        if cells[0] != "total":
            counts[cells[0]] = dict(zip(names, map(int, cells[1:-1])))
    return counts


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--qwen3", help="path to Qwen/Qwen3-8B tokenizer.json")
    parser.add_argument("--deepseek", help="path to deepseek-ai/DeepSeek-V3 tokenizer.json")
    parser.add_argument("files", nargs="+")
    args = parser.parse_args()

    references = {
        "o200k_base": tiktoken.get_encoding("o200k_base").encode_ordinary,
        "cl100k_base": tiktoken.get_encoding("cl100k_base").encode_ordinary,
    }
    for name, path in (("qwen3", args.qwen3), ("deepseek_v3", args.deepseek)):
        if path:
            tok = Tokenizer.from_file(path)
            references[name] = lambda text, tok=tok: tok.encode(text, add_special_tokens=False).ids

    ours = tokcount(args.files)
    failures = 0
    print(f"{'text':<28} {'vocabulary':<12} {'tokcount':>10} {'reference':>10}  verdict")
    for file in args.files:
        text = Path(file).read_text(encoding="utf-8")
        for name, encode in references.items():
            expected = len(encode(text))
            got = ours[file][name]
            verdict = "ok" if got == expected else f"MISMATCH ({got - expected:+d})"
            failures += got != expected
            print(f"{Path(file).name:<28} {name:<12} {got:>10} {expected:>10}  {verdict}")
    print(f"{failures} mismatch(es)")
    sys.exit(1 if failures else 0)


if __name__ == "__main__":
    main()
