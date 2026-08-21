#!/usr/bin/env bash
# bit-identity-debug-only.sh — topo/source.rs must stay debug-only.
# ONE home; ci.yml's "bit-identity debug-only guard (topo/source.rs)"
# step and local-scripts/ci-local.sh's discipline row both call this
# file.
#
# source.rs must stay debug-only: its bit-channel calls may only appear
# inside `cfg(debug_assertions)` items, or inside a `debug_assert!`
# (which is itself compiled out of release). THIS GATE IS THAT FILE'S
# ONLY CONTROL — `bit-identity-consumer.sh` excludes it wholesale — so
# what it can and cannot see is the whole guarantee.
#
# IT CORRELATES, and it did not used to. The inherited form counted two
# things and related neither: uses of the channel, and occurrences of
# `cfg(debug_assertions)`, failing only when there were uses and no
# occurrences at all. ONE gate anywhere in the file licensed any number
# of ungated production uses — and the gate then PRINTED that the file
# "gates its N uses behind cfg(debug_assertions)", a sentence it had no
# evidence for. It also counted uses in COMMENTS, so the number it
# announced was not the number of calls either. Each use is now placed
# against the item that encloses it.
#
# THE SUBJECT MUST EXIST. This gate names one file, and the inherited
# form did not check for it: on a missing `crates/topo/src/source.rs`
# both counts were the empty string, `[ "" -gt 0 ]` raised "integer
# expression expected", `&&` read that as false, and the gate exited 0 —
# GREEN exactly when its subject had moved out from under it.
# `gate_require_file` turns that case into a loud failure.
#
# KNOWN GAP: `#[cfg(any(debug_assertions, …))]` is NOT read as a gate,
# because it is not one — such an item is compiled in whenever the other
# condition holds. Conservative in the direction that fires.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

SUBJECT=crates/topo/src/source.rs
GATE_SCAN_NOUN='bit-channel use'

# Emits `USE` for every bit-channel call and `UNGATED` for every one of
# them not enclosed by a `cfg(debug_assertions)` item. Enclosure is
# brace depth over the code-only text, so a use is placed against the
# item it is in rather than against the file it is in.
debug_only_report() {
  awk '
    {
      p1 = index($0, ":"); r = substr($0, p1 + 1); p2 = index(r, ":")
      if (p1 == 0 || p2 == 0) next
      f = substr($0, 1, p1 - 1); ln = substr(r, 1, p2 - 1)
      code = substr(r, p2 + 1)
      o = code; opens = 0; closes = 0
      if (index(o, "{") > 0) opens = gsub(/\{/, "{", o)
      if (index(o, "}") > 0) closes = gsub(/\}/, "}", o)
      if (gated == 0 && code ~ /#\[cfg\((all\()?debug_assertions[,)]/) {
        gated = 1; seen = 0; gdepth = depth
      }
      if (code ~ /bit_identity::|eq_bits/) {
        print "USE"
        if (gated == 0 && code !~ /debug_assert/)
          print "UNGATED " f ":" ln ":" code
      }
      if (gated == 1) {
        if (opens > 0) seen = 1
        depth += opens - closes
        if (seen == 1 && depth <= gdepth) gated = 0
        else if (seen == 0 && index(code, ";") > 0) gated = 0
      } else depth += opens - closes
    }
  '
}

gate() {
  gate_require_file "$SUBJECT"
  local report uses ungated
  report=$(gate_rust_code "$SUBJECT" | debug_only_report)
  ungated=$(printf '%s\n' "$report" | grep '^UNGATED ' | sed 's/^UNGATED //' || true)
  uses=$(printf '%s\n' "$report" | grep -c '^USE$' || true)
  GATE_SCAN_FILES=$uses
  if [ -n "$ungated" ]; then
    printf '%s\n' "$ungated"
    gate_error "$SUBJECT uses the bit channel above outside any cfg(debug_assertions) item — the retirement allows a debug assertion only. One gated use elsewhere in the file does not cover these"
    exit 1
  fi
  gate_ok "every bit-channel use in $SUBJECT is inside a cfg(debug_assertions) item or a debug_assert!"
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

# THE CASE THE COUNTING FORM PASSED, and the reason this gate was
# rewritten: one properly gated use, and a production leak beside it.
plant_one_gated_one_leaked() {
  mkdir -p "$1/crates/topo/src"
  {
    printf '#[cfg(debug_assertions)]\n'
    printf 'pub fn agree(a: f64, b: f64) -> bool { eq_bits(a, b) }\n'
    printf 'pub fn production_leak(a: f64, b: f64) -> bool {\n'
    printf '    geom_core::bit_identity::eq_bits(&a, &b) == Some(true)\n'
    printf '}\n'
  } > "$1/$SUBJECT"
}

# The gated item ENDS, and the next use is outside it. Depth, not
# proximity.
plant_after_the_gated_item() {
  mkdir -p "$1/crates/topo/src"
  {
    printf '#[cfg(debug_assertions)]\n'
    printf 'pub fn agree(a: f64, b: f64) -> bool {\n'
    printf '    eq_bits(a, b)\n'
    printf '}\n'
    printf 'pub fn later(a: f64, b: f64) -> bool { eq_bits(a, b) }\n'
  } > "$1/$SUBJECT"
}

# `any(debug_assertions, …)` is not a debug-only gate.
plant_any_cfg() {
  mkdir -p "$1/crates/topo/src"
  {
    printf '#[cfg(any(debug_assertions, feature = "probe"))]\n'
    printf 'pub fn agree(a: f64, b: f64) -> bool { eq_bits(a, b) }\n'
  } > "$1/$SUBJECT"
}

# THE NEAR MISSES. A use inside a `debug_assert!` is debug-only by
# construction; a use named in prose is not a use at all, and counting
# one is how this gate announced a number that was not the number of
# calls.
plant_permitted_shapes() {
  mkdir -p "$1/crates/topo/src"
  {
    printf '/// The one bit-identity call site: eq_bits, cfg(debug_assertions)-gated.\n'
    printf '// bit_identity::eq_bits is named here and used nowhere.\n'
    printf '/*\n * Nor is eq_bits used inside this block comment.\n */\n'
    printf 'pub const NOTE: &str = "eq_bits";\n'
    printf 'pub fn checked(a: f64, b: f64) {\n'
    printf '    debug_assert!(geom_core::bit_identity::eq_bits(&a, &b) == Some(true));\n'
    printf '}\n'
    printf '#[cfg(debug_assertions)]\n'
    printf 'mod inner {\n'
    printf '    pub fn agree(a: f64, b: f64) -> bool { super::eq_bits(a, b) }\n'
    printf '}\n'
  } > "$1/$SUBJECT"
}

gate_selftest() {
  local want="outside any cfg(debug_assertions) item"
  gate_selftest_clean
  gate_selftest_case "$want" plant
  gate_selftest_case "$want" plant_one_gated_one_leaked
  gate_selftest_case "$want" plant_after_the_gated_item
  gate_selftest_case "$want" plant_any_cfg
  gate_selftest_case "the gate's subject is gone" plant_subject_gone
  gate_selftest_passes "a debug_assert!, prose, a string literal and a gated inner module" plant_permitted_shapes
  printf '%s selftest OK: passes a clean fixture, a debug_assert!, prose/strings and a gated inner module; fires on a bare use, on a leak BESIDE a properly gated use, on a use after the gated item closes, on an any(debug_assertions, …) item, and on the subject file being gone\n' "$(gate_name)"
}

# The subject removed out from under the gate.
plant_subject_gone() { rm -f "$1/$SUBJECT"; }

gate_parse_args "$@"
gate_main
