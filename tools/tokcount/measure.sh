#!/usr/bin/env bash
# Replays the measurement recorded in docs/spec_taille.md: fetches the
# reference specifications at pinned versions, checks their hashes,
# extracts their text and counts them under every vocabulary.
#
# Needs curl, sha256sum, tar, pdftotext (poppler) and cargo. The PDF
# extraction is the one step done by a tool outside this crate; its
# version is printed with the results.
#
#   tools/tokcount/measure.sh [WORK_DIR]

set -euo pipefail

here=$(cd "$(dirname "$0")" && pwd)
work=${1:-$here/target/corpus}
mkdir -p "$work"

fetch() {
    local name=$1 url=$2 sha256=$3
    [ -f "$work/$name" ] || curl -sSL -o "$work/$name" "$url"
    echo "$sha256  $work/$name" | sha256sum --check --quiet
}

fetch n3220.pdf \
    https://www.open-std.org/jtc1/sc22/wg14/www/docs/n3220.pdf \
    bccd974cd9d0227c282c9557e12e29c90df115b403ea4269f58d69661acfc113
fetch n4950.pdf \
    https://www.open-std.org/jtc1/sc22/wg21/docs/papers/2023/n4950.pdf \
    2a2fdee7eb4271263451468ff689d6fd3bbb377d7c6e45375796badfb60d50a2
fetch go_spec.html \
    https://raw.githubusercontent.com/golang/go/go1.25.0/doc/go_spec.html \
    42a4ff1c01efe8dc23eca4378b8fb03d76e9f2f079dab01298ffd74d8b0e3149
fetch zig_langref.html \
    https://ziglang.org/documentation/0.15.1/ \
    55843fd8297f8e7ed1b280ecd8b5a2cfcb2a801cfdbba2ffd61f07603b1853e9
rust_ref=3989fe91f1a601b51684a582ad17b2503f221787
fetch rust_reference.tar.gz \
    https://github.com/rust-lang/reference/archive/$rust_ref.tar.gz \
    512e4fb66c690450cd6551c04640105bc7ff45a1d2b88019c6604668b5c6d83c

cargo build --release --quiet --manifest-path "$here/Cargo.toml"
tokcount=$here/target/release/tokcount

pdftotext -enc UTF-8 "$work/n3220.pdf" "$work/c23_n3220.txt"
pdftotext -enc UTF-8 "$work/n4950.pdf" "$work/cpp23_n4950.txt"
"$tokcount" strip-html "$work/go_spec.html" > "$work/go_spec.txt"
"$tokcount" strip-html "$work/zig_langref.html" > "$work/zig_langref.txt"
tar xzf "$work/rust_reference.tar.gz" -C "$work"
find "$work/reference-$rust_ref/src" -name '*.md' | LC_ALL=C sort | xargs cat > "$work/rust_reference.md"

echo "pdftotext: $(pdftotext -v 2>&1 | head -n 1)"
echo
cd "$work"
"$tokcount" count c23_n3220.txt cpp23_n4950.txt go_spec.txt zig_langref.txt rust_reference.md
