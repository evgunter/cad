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
# TWO ENCLOSURES, and both are STRUCTURAL rather than per-line.
#
#   * A `cfg(debug_assertions)` ITEM encloses by BRACE DEPTH, so a use
#     after the item closes is outside it however close it looks.
#   * A `debug_assert!` encloses by STATEMENT, and the statement ends at
#     `;`, `{` or `}`. A per-line substring test gets this wrong in both
#     directions and the first version of this rewrite did:
#     `{ debug_assert!(a == a); eq_bits(a, b) }` passed — **and printed
#     the evidence-free sentence this gate was rewritten to stop
#     printing** — while a rustfmt-wrapped `debug_assert!(\n … \n);`
#     around a use fired.
#
# KNOWN GAP 1: `#[cfg(any(debug_assertions, …))]` is NOT read as a gate,
# because it is not one — such an item is compiled in whenever the other
# condition holds. Nor is `#[cfg(not(debug_assertions))]`, which is a
# release-only item. Both are conservative in the direction that fires.
# The `all(…)` form IS read as a gate, in any operand order: an earlier
# draft matched `all(debug_assertions, …)` and not the operands swapped,
# which is S56's order-sensitivity minted fresh in the PR that closes
# S125.
#
# KNOWN GAP 2: a `debug_assert!` whose argument list contains a `;` (a
# block expression) ends the statement early, so a use after that `;`
# reads as ungated. Cry-wolf, and no such site exists here.
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
    BEGIN {
      # A `debug_assert…!` macro, not a function whose name starts the
      # same way: the `!` is the whole distinction.
      DBG = "debug_assert[a-z_]*!"
    }
    {
      p1 = index($0, ":"); r = substr($0, p1 + 1); p2 = index(r, ":")
      if (p1 == 0 || p2 == 0) next
      f = substr($0, 1, p1 - 1); ln = substr(r, 1, p2 - 1)
      code = substr(r, p2 + 1)
      if (f != FNAME) { FNAME = f; depth = 0; gated = 0; seen = 0; stmt = "" }
      if (gated == 0 &&
          code ~ /#\[cfg\(([^]]*[(,][[:space:]]*)?debug_assertions[,)]/ &&
          code !~ /#\[cfg\([^]]*(any|not)\(/) {
        gated = 1; seen = 0; gdepth = depth
      }
      # Delimiter-wise, so that the statement a use sits in is the
      # statement the `debug_assert!` test asks about, and so that brace
      # depth moves at the brace rather than at the end of the line.
      while (1) {
        if (match(code, /[{};]/)) { cut = RSTART; piece = substr(code, 1, cut - 1) }
        else { cut = 0; piece = code }
        stmt = stmt " " piece
        if (piece ~ /bit_identity::|eq_bits/) {
          print "USE"
          if (gated == 0 && stmt !~ DBG)
            print "UNGATED " f ":" ln ":" piece
        }
        if (cut == 0) break
        d = substr(code, cut, 1)
        if (d == "{") { depth++; if (gated == 1) seen = 1 }
        else if (d == "}") {
          depth--
          if (gated == 1 && seen == 1 && depth <= gdepth) gated = 0
        } else if (gated == 1 && seen == 0) gated = 0
        stmt = ""
        code = substr(code, cut + 1)
      }
    }
  '
}

gate() {
  gate_require_file "$SUBJECT"
  local report uses ungated
  report=$(gate_rust_code "$SUBJECT" | debug_only_report)
  ungated=$(printf '%s\n' "$report" | gate_grep '^UNGATED ' | sed 's/^UNGATED //')
  uses=$(printf '%s\n' "$report" | gate_grep -c '^USE$')
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

# THE SENTENCE THIS GATE EXISTS TO STOP PRINTING. A `debug_assert!`
# earlier on the LINE is not an enclosure — the statement ended at the
# `;` — and the per-line substring test that read it as one passed this
# fixture while printing *"every bit-channel use … is inside a
# cfg(debug_assertions) item or a debug_assert!"*, which is verbatim the
# evidence-free sentence S63 recorded against the form this replaced.
plant_leak_after_debug_assert() {
  mkdir -p "$1/crates/topo/src"
  printf 'pub fn leak(a: f64, b: f64) -> bool { debug_assert!(a == a); eq_bits(a, b) }\n' \
    > "$1/$SUBJECT"
}

# The same enclosure the other way round: a use in a `debug_assert!`
# that rustfmt has wrapped over three lines is inside it, and the
# per-line test called it a violation.
plant_wrapped_debug_assert() {
  mkdir -p "$1/crates/topo/src"
  {
    printf 'pub fn ok(a: f64, b: f64) {\n'
    printf '    debug_assert!(\n'
    printf '        geom_core::bit_identity::eq_bits(&a, &b) == Some(true)\n'
    printf '    );\n'
    printf '}\n'
  } > "$1/$SUBJECT"
}

# `all(…)` IS a gate, in EITHER operand order. The first version of this
# rewrite read only `all(debug_assertions, …)` — S56's order-sensitivity,
# minted fresh in the PR that closes S125.
plant_all_cfg_swapped() {
  mkdir -p "$1/crates/topo/src"
  {
    printf '#[cfg(all(feature = "probe", debug_assertions))]\n'
    printf 'pub fn agree(a: f64, b: f64) -> bool { eq_bits(a, b) }\n'
  } > "$1/$SUBJECT"
}

# `not(debug_assertions)` is a RELEASE-only item, so a use inside it is
# a production use.
plant_not_cfg() {
  mkdir -p "$1/crates/topo/src"
  {
    printf '#[cfg(not(debug_assertions))]\n'
    printf 'pub fn agree(a: f64, b: f64) -> bool { eq_bits(a, b) }\n'
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
  gate_selftest_case "$want" plant_not_cfg
  gate_selftest_case "$want" plant_leak_after_debug_assert
  gate_selftest_case "the gate's subject is gone" plant_subject_gone
  gate_selftest_passes "a debug_assert!, prose, a string literal and a gated inner module" plant_permitted_shapes
  gate_selftest_passes "a rustfmt-wrapped debug_assert! around the use" plant_wrapped_debug_assert
  gate_selftest_passes "cfg(all(…)) with debug_assertions as the SECOND operand" plant_all_cfg_swapped
  printf '%s selftest OK: passes a clean fixture, a debug_assert! (wrapped or not), cfg(all(…)) in either operand order, prose/strings and a gated inner module; fires on a bare use, on a leak BESIDE a properly gated use, on a leak AFTER a debug_assert! on the same line, on a use after the gated item closes, on any(…) and not(debug_assertions) items, and on the subject file being gone\n' "$(gate_name)"
}

# The subject removed out from under the gate.
plant_subject_gone() { rm -f "$1/$SUBJECT"; }

gate_parse_args "$@"
gate_main
