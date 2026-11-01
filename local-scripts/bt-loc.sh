#!/usr/bin/env bash
# Temporary investigation helper (NOT for merge).
cd "$(dirname "$0")/.."
for c in crates/*/; do
  name=$(basename "$c")
  s=$(find "$c/src" -name '*.rs' 2>/dev/null -print0 | xargs -0 cat 2>/dev/null | wc -l)
  t=$(find "$c/tests" -name '*.rs' 2>/dev/null -print0 | xargs -0 cat 2>/dev/null | wc -l)
  printf '%-14s src=%7d tests=%7d\n' "$name" "$s" "$t"
done
