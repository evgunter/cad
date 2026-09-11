#!/usr/bin/env bash
# Build the digest text from a raw dir produced by r1m4-digest.sh /
# r1m4-runs.sh. Patterns match ANYWHERE in a line because --nocapture
# glues test stdout to the harness's "test name ... " prefix.
set -euo pipefail
RAW="$1"; OUT="$2"
: > "$OUT"
for BAND in default 1e-6 1e-12; do
  echo "== band $BAND mesh ==" >> "$OUT"
  grep -oE 'test [A-Za-z0-9_:]+ \.\.\. (ok|ignored)' "$RAW/mesh.$BAND.txt" | sort >> "$OUT"
  grep -oE 'HASH [a-z_0-9.]+ d=[0-9.]+: pos [0-9a-f]{16} tri [0-9a-f]{16} bnd [0-9a-f]{16}' "$RAW/mesh.$BAND.txt" | sort >> "$OUT"
  grep -oE 'Z5 HASH [0-9a-f]{16} positions=[0-9]+' "$RAW/mesh.$BAND.txt" | sort >> "$OUT"
  grep -oE '[a-z_0-9.]+ d=[0-9.]+ n=[0-9]+ t=[0-9]+ => [0-9a-f]{16}' "$RAW/mesh.$BAND.txt" | sort >> "$OUT"
  grep -oE 'test result: ok\. [0-9]+ passed; [0-9]+ failed; [0-9]+ ignored' "$RAW/mesh.$BAND.txt" >> "$OUT"
  echo "== band $BAND poleguard ==" >> "$OUT"
  grep -oE 'test [A-Za-z0-9_:]+ \.\.\. (ok|ignored)' "$RAW/pg.$BAND.txt" | sort >> "$OUT"
  grep -oE 'test result: ok\. [0-9]+ passed; [0-9]+ failed; [0-9]+ ignored' "$RAW/pg.$BAND.txt" >> "$OUT"
  echo "== band $BAND budget ==" >> "$OUT"
  grep -oE 'test [A-Za-z0-9_:]+ \.\.\. (ok|ignored)' "$RAW/budget.$BAND.txt" | sort >> "$OUT"
  grep -oE 'test result: ok\. [0-9]+ passed; [0-9]+ failed; [0-9]+ ignored' "$RAW/budget.$BAND.txt" >> "$OUT"
done
echo "digest lines: $(wc -l < "$OUT")"
md5sum "$OUT"
