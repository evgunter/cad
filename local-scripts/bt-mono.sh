#!/usr/bin/env bash
# Temporary investigation helper (NOT for merge).
# Monomorphization census of an already-built rlib: for each generic function,
# how many distinct instantiations were codegen'd INTO THIS CRATE and how many
# bytes of machine code they total. Groups instantiations by erasing generic
# type arguments from the demangled name.
#   usage: bt-mono.sh <crate_underscored> [top-N] [profile]
cd "$(dirname "$0")/.."
prefix="$1"; top="${2:-25}"; prof="${3:-debug}"
f=$(ls target/$prof/deps/lib${prefix}-*.rlib 2>/dev/null | head -1)
[ -z "$f" ] && { echo "no rlib for $prefix in target/$prof"; exit 1; }
echo "## $f"
nm -S --defined-only "$f" 2>/dev/null | grep -E ' [Tt] ' | awk '{print strtonum("0x" $2) "\t" $4}' \
  | rustfilt > /tmp/bt-mono-$prefix.txt
echo "text symbols: $(wc -l < /tmp/bt-mono-$prefix.txt)   text bytes: $(cut -f1 /tmp/bt-mono-$prefix.txt | paste -sd+ | bc)"
echo
echo "top $top by total bytes (generic args erased):  bytes  copies  name"
sed -E 's/::h[0-9a-f]{16}$//' /tmp/bt-mono-$prefix.txt \
  | awk -F'\t' '{ n=$2;
      for(i=0;i<6;i++) gsub(/<[^<>]*>/,"<>",n);
      b[n]+=$1; c[n]++ } END { for (k in b) printf "%9d %6d  %s\n", b[k], c[k], k }' \
  | sort -rn | head -"$top"
echo
echo "top 15 by COPY COUNT:"
sed -E 's/::h[0-9a-f]{16}$//' /tmp/bt-mono-$prefix.txt \
  | awk -F'\t' '{ n=$2;
      for(i=0;i<6;i++) gsub(/<[^<>]*>/,"<>",n);
      b[n]+=$1; c[n]++ } END { for (k in b) printf "%6d %9d  %s\n", c[k], b[k], k }' \
  | sort -rn | head -15
