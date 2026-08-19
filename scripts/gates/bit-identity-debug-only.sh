#!/usr/bin/env bash
# bit-identity-debug-only.sh — topo/source.rs must stay debug-only.
# ONE home; ci.yml's "bit-identity debug-only guard (topo/source.rs)"
# step and local-scripts/ci-local.sh's discipline row both call this
# file.
#
# source.rs must stay debug-only: its bit-channel calls may only
# appear inside cfg(debug_assertions) items (grep-level guard:
# the file declares the gate above each use).
#
# THE SUBJECT MUST EXIST. This gate names one file, and the inherited
# form did not check for it: on a missing `crates/topo/src/source.rs`
# both `grep -c` calls exit 2 printing nothing, both counts are the
# empty string, `[ "" -gt 0 ]` raises "integer expression expected" and
# returns 2, `&&` reads that as false, and the gate exits 0 — GREEN
# exactly when its subject had moved out from under it. `set -e` does
# not catch it because the test is an `if` condition. `gate_require_file`
# turns that case into a loud failure; the counts can no longer be empty,
# so the arithmetic is total.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

SUBJECT=crates/topo/src/source.rs

gate() {
  gate_require_file "$SUBJECT"
  local uses gates
  uses=$(grep -cE 'bit_identity::|eq_bits' "$SUBJECT" || true)
  gates=$(grep -c 'cfg(debug_assertions)' "$SUBJECT" || true)
  if [ "$uses" -gt 0 ] && [ "$gates" -eq 0 ]; then
    gate_error "topo/src/source.rs uses the bit channel without cfg(debug_assertions) gating — the retirement allows a debug assertion only"
    exit 1
  fi
  gate_ok "topo/src/source.rs gates its $uses bit-channel use(s) behind cfg(debug_assertions)"
}

# This gate's subject is one file, not `crates/*/src`.
gate_plant_clean() {
  mkdir -p "$1/crates/topo/src"
  printf '#[cfg(debug_assertions)]\npub fn agree(a: f64, b: f64) -> bool { eq_bits(a, b) }\n' \
    > "$1/$SUBJECT"
}

plant() {
  mkdir -p "$1/crates/topo/src"
  printf 'pub fn agree(a: f64, b: f64) -> bool { eq_bits(a, b) }\n' > "$1/$SUBJECT"
}

gate_parse_args "$@"
gate_main "without cfg(debug_assertions) gating" plant
