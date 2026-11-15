#!/usr/bin/env bash
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
# /ci.sh
# ================================================================
# The one check of the repository: runs every step, reports each
# one, fails if any failed. Local and remote runs call this file.
# ================================================================
# Author: Maxime MARCONATO (aka MaaaX^^, aka EPOCH84)
# ================================================================
# License: MIT OR Apache-2.0, at your option — see LICENSE-MIT
# and LICENSE-APACHE.
# SPDX-License-Identifier: MIT OR Apache-2.0
# ================================================================

# The single description of the check (docs/targets.md §1,
# docs/measurements.md §10). Every step runs even after a failure,
# so that one run shows everything that is wrong.
#
# Needs bash, git, awk, cargo with clippy and rustfmt, shellcheck
# and python3. A missing tool is an error, never a skipped step: a
# check that skips in silence is a check that drifts.
#
#   ./ci.sh
#
# Exit status: 0 every step passed, 1 at least one failed, 2 a tool
# is missing or the script was called with arguments.

set -euo pipefail

if [ "$#" -ne 0 ]; then
    printf 'usage: %s\n' "$0" >&2
    exit 2
fi

root=$(cd "$(dirname "$0")" && pwd)
cd "$root"

for tool in git awk cargo shellcheck python3; do
    if ! command -v "$tool" > /dev/null; then
        printf 'ci.sh: %s is required and was not found\n' "$tool" >&2
        exit 2
    fi
done

tokcount=tools/tokcount/Cargo.toml
failed=()

# Runs one step and records its name if it fails. The step runs in a
# subshell outside any condition: bash ignores `set -e` in the whole
# body of a function called from an `if` or before `||`, and a step
# would then pass on the status of its last command alone.
step() {
    local name=$1
    shift
    printf '== %s\n' "$name"
    local status=0
    set +e
    (
        set -e
        "$@"
    )
    status=$?
    set -e
    if [ "$status" -ne 0 ]; then
        failed+=("$name")
        printf 'FAILED: %s\n' "$name"
    fi
}

# Prints, NUL-separated, the files of the work tree matching a
# pattern: tracked ones and new ones alike, ignored ones left out, so
# that a file is checked before it is added and not only after.
files_matching() {
    git ls-files -z --cached --others --exclude-standard -- "$1"
}

tokcount_tests() {
    cargo test --quiet --locked --manifest-path "$tokcount"
}

tokcount_lints() {
    cargo clippy --quiet --locked --all-targets --manifest-path "$tokcount" -- -D warnings
    cargo fmt --check --manifest-path "$tokcount"
}

shell_sources() {
    local -a scripts
    mapfile -d '' scripts < <(files_matching '*.sh')
    local script
    for script in "${scripts[@]}"; do
        bash -n "$script"
        shellcheck "$script"
    done
    printf '%d shell scripts\n' "${#scripts[@]}"
}

# Syntax only: the scripts import packages that are not required to
# be installed here. Parsed in memory, so nothing is written beside
# the sources.
python_sources() {
    local -a scripts
    mapfile -d '' scripts < <(files_matching '*.py')
    python3 -c '
import ast
import sys

for path in sys.argv[1:]:
    with open(path, encoding="utf-8") as source:
        ast.parse(source.read(), path)
' "${scripts[@]}"
    printf '%d python scripts\n' "${#scripts[@]}"
}

# Checks what the documents say of each other: every `docs/<name>.md`
# they mention exists, and every "§N" names a numbered section
# ("## N. Title") of the document it points at.
#
# A "§N" points at the document named just before it — `docs/tool.md`
# §4, requirements §8, the requirements (§2) —, at the same document
# as the "§" before it in a list — §1, §2 and §6 —, and otherwise at
# the document it is written in. Outside docs/, a "§N" that names no
# document has no owner and is not checked. Two forms point outside
# the repository and are not checked either: a dotted number, §17.4,
# which no document here uses, and a number after an acronym, JLS §17.
#
# The files are read twice: once to learn which sections exist, once
# to check the references, a reference being free to wrap over two
# lines.
doc_references() {
    local -a documents
    mapfile -d '' documents < <(files_matching '*.md')
    LC_ALL=C awk '
        function report(file, message) {
            printf "%s: %s\n", file, message
            bad = 1
        }

        function check(file, text,    rest, path, offset, token, number, before, word, owner, last) {
            gsub(/[ \t]+/, " ", text)

            rest = text
            while (match(rest, /docs\/[a-z_]+\.md/)) {
                path = substr(rest, RSTART, RLENGTH)
                if (!(path in exists)) {
                    report(file, path " does not exist")
                }
                rest = substr(rest, RSTART + RLENGTH)
            }

            offset = 0
            last = ""
            while (match(substr(text, offset + 1), /§[0-9]+/)) {
                token = substr(text, offset + RSTART, RLENGTH)
                before = substr(text, 1, offset + RSTART - 1)
                offset = offset + RSTART + RLENGTH - 1
                number = token
                sub(/^[^0-9]+/, "", number)
                if (substr(text, offset + 1) ~ /^\.[0-9]/ || before ~ /[A-Z][A-Z]+ $/) {
                    last = ""
                    continue
                }

                owner = ""
                if (match(before, /`(docs\/)?[a-z_]+\.md`,? $/)) {
                    owner = substr(before, RSTART, RLENGTH)
                    gsub(/[`, ]/, "", owner)
                    if (owner !~ /^docs\//) {
                        owner = "docs/" owner
                    }
                } else if (match(before, /[A-Za-z_]+,? \(?$/)) {
                    word = tolower(substr(before, RSTART, RLENGTH))
                    gsub(/[, (]/, "", word)
                    if (("docs/" word ".md") in exists) {
                        owner = "docs/" word ".md"
                    }
                }
                if (owner == "" && before ~ /§[0-9]+(, and |, | and )$/) {
                    owner = last
                }
                if (owner == "" && file ~ /^docs\//) {
                    owner = file
                }
                last = owner

                if (owner != "" && !((owner, number) in section)) {
                    report(file, token " names no section of " owner)
                }
            }
        }

        pass == 1 {
            exists[FILENAME] = 1
            if (match($0, /^## [0-9]+\. /)) {
                number = substr($0, 4, RLENGTH - 5)
                section[FILENAME, number] = 1
            }
            next
        }

        FNR == 1 && current != "" {
            check(current, text)
            text = ""
        }

        {
            current = FILENAME
            line = $0
            sub(/^(> ?)+/, "", line)
            text = text " " line
        }

        END {
            if (current != "") {
                check(current, text)
            }
            exit bad
        }
    ' pass=1 "${documents[@]}" pass=2 "${documents[@]}"
    printf '%d documents\n' "${#documents[@]}"
}

step 'tokcount: tests' tokcount_tests
step 'tokcount: clippy and rustfmt' tokcount_lints
step 'shell scripts: syntax and shellcheck' shell_sources
step 'python scripts: syntax' python_sources
step 'documents: paths and section numbers' doc_references

printf '\n'
if [ "${#failed[@]}" -ne 0 ]; then
    printf 'ci.sh: %d step(s) failed:\n' "${#failed[@]}" >&2
    printf '  %s\n' "${failed[@]}" >&2
    exit 1
fi
printf 'ci.sh: every step passed\n'
