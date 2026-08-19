#!/usr/bin/env bash
# bit-identity-debug-only.sh — topo/source.rs must stay debug-only.
# ONE home; ci.yml's "bit-identity debug-only guard (topo/source.rs)"
# step and local-scripts/ci-local.sh's discipline row both call this
# file.
#
# source.rs must stay debug-only: its bit-channel calls may only
# appear inside cfg(debug_assertions) items (grep-level guard:
# the file declares the gate above each use).
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

gate() {
  local uses gates
  uses=$(grep -cE 'bit_identity::|eq_bits' crates/topo/src/source.rs || true)
  gates=$(grep -c 'cfg(debug_assertions)' crates/topo/src/source.rs || true)
  if [ "$uses" -gt 0 ] && [ "$gates" -eq 0 ]; then
    gate_error "topo/src/source.rs uses the bit channel without cfg(debug_assertions) gating — the retirement allows a debug assertion only"
    exit 1
  fi
}

plant() {
  mkdir -p "$1/crates/topo/src"
  printf 'pub fn agree(a: f64, b: f64) -> bool { eq_bits(a, b) }\n' \
    > "$1/crates/topo/src/source.rs"
}

gate_parse_args "$@"
gate_main "without cfg(debug_assertions) gating" plant
