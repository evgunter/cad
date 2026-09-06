#!/usr/bin/env bash
# bit-identity-debug-only.sh — the debug-only subjects stay debug-only.
# ONE home; ci.yml's `discipline` job and local-scripts/ci-local.sh's
# discipline row both call this file by name.
#
# A SUBJECT LIST, NOT A FILE. `SUBJECTS` carries one row per file that
# owes a symbol to `cfg(debug_assertions)`: the path, the spellings
# whose uses are gated, and the text the fixtures write.
# The enclosure analysis below is the part worth getting right, so it
# exists exactly once and a second debug-only mechanism is a row here
# rather than a second gate with a second reader.
#
#   * `crates/topo/src/source.rs` — the bit channel. Its calls may only
#     appear inside `cfg(debug_assertions)` items, or inside a
#     `debug_assert!` (which is itself compiled out of release). THIS
#     GATE IS THAT FILE'S ONLY CONTROL — `bit-identity-consumer.sh`
#     excludes it wholesale — so what it can and cannot see is the whole
#     guarantee.
#   * `crates/editor-core/src/product.rs` — the gather counter: the
#     `GATHERS` cell, the increment in `product_recorded`, and
#     `gathers_on_this_thread`. A fourth site without the attribute, or
#     the attribute dropped from one of the three, compiles and passes
#     every test.
#   * `crates/mesh/src/curved.rs` — the sphere/torus lane's
#     identified-vertex census: `identified_ids` re-derives the set of
#     mesh ids the boundary walk placed at two UV locations, and
#     `overused_identified_edge` re-derives, over the emitted triangles,
#     the fan edge that census forbids.
#   * `crates/mesh/src/tessellate.rs` — `unpaired_chord_segment`, the
#     re-derivation over the assembled mesh of the chord segment used by
#     other than two face triangles.
#   * `crates/mesh/src/walk.rs` — `overused_identified_edge_in`, the one
#     home of the identified-vertex fan census both surface lanes call.
#   * `crates/mesh/src/trimmed.rs` — the trimmed lane's call of that same
#     census. A SUBJECT IS A FILE, so a symbol whose uses cross files
#     owes a row per file that uses it; a row covering the definition
#     alone pins the attribute on the definition and nothing about the
#     call sites.
#
# WHAT IS PINNED IS THE SOURCE SHAPE. This workspace's
# `[profile.release]` sets `debug-assertions = true` until publish, so a
# `cfg(debug_assertions)` item is not "absent from a release build"
# here. What the attribute buys is that cargo's OWN release defaults
# strip it — which is what a consumer building these crates normally
# gets — and that the day the stanza comes out, nothing has to change.
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
# EVERY SUBJECT MUST EXIST, and every row is proved present before any
# row is scanned, so a subject that moved cannot hide behind the ones
# that stayed. The inherited form checked for none of it: on a missing
# `crates/topo/src/source.rs` both counts were the empty string,
# `[ "" -gt 0 ]` raised "integer expression expected", `&&` read that as
# false, and the gate exited 0 — GREEN exactly when its subject had
# moved out from under it. `gate_require_file` turns that case into a
# loud failure.
#
# TWO ENCLOSURES, and both are STRUCTURAL rather than per-line.
#
#   * A `cfg(debug_assertions)` ITEM encloses by BRACE DEPTH, so a use
#     after the item closes is outside it however close it looks.
#   * A `cfg(debug_assertions)` ITEM ENDS AT A `;` ONLY WHERE A `;` CAN
#     END ONE: at round/square bracket depth zero, which is a `use`, a
#     `type` alias or an attribute in statement position. A `;` inside
#     the item's SIGNATURE — an array type `[(T, T); N]`, a const-generic
#     default — ends nothing, and reading it as an item end reports a
#     correctly gated `fn` ungated. Angle brackets are NOT counted: a `;`
#     reaches the inside of a `<…>` only through a `[…]` or a `{…}`,
#     which are, and reading `<`/`>` as brackets mistakes every
#     comparison for one.
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
#
# KNOWN GAP 3: a symbol is matched at identifier boundaries, so a longer
# name that merely contains it is not a use. What the reader cannot tell
# apart is a WHOLE-identifier collision — the same spelling naming
# something that is not the mechanism. No row has one; a row that
# would is one this reader cannot serve.
#
# KNOWN GAP 4: a rustfmt-wrapped attribute — `#[cfg(` and
# `debug_assertions` and `)]` on three lines — is not read as a gate,
# because the attribute matcher reads one line, so a use inside that
# item fires. Cry-wolf, and no such site exists here.
#
# KNOWN GAP 5: a `{ … }` const-generic default in a gated signature
# marks the item entered at that brace, so the item reads as closed at
# the matching `}` and a use in the real body fires. Cry-wolf again, in
# the same direction.
#
# KNOWN GAP 6: an attribute in STATEMENT position over a multi-line call
# whose ARGUMENTS contain a brace — a struct literal or an `if` arm
# handed to the call — is not servable at all. The first `{` the reader
# meets is inside the still-open `(`, which is the positive desync the
# body-brace arm refuses to tolerate, so every such site is reported and
# the gate reds. That is the safe direction and it is also a CEILING on
# the subject list: a mechanism whose sites take that shape cannot be a
# row here until the reader can say where a statement-position item
# ends. `crates/topo/src/euler.rs`'s `ArenaDelta` — the per-operator
# arena shift `assert_euler_postcondition` checks — is the live one:
# twelve of its sites take that shape, in five of the eight `topo` files
# that name it.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

# PATH, the symbol spellings, a bare SYM the fixtures name, one EXPR
# that uses it, and the noun the diagnosis calls the mechanism by. Only
# the noun may carry spaces, and it is last for that reason.
#
# The symbol field is a `|`-SEPARATED LIST of spellings, each matched at
# identifier boundaries, and not a general ERE: the reader splits on
# `|`, so a `|` inside a group would be read as a separator. It reaches
# awk through `-v`, which processes backslash escapes, so `foo\.bar`
# would arrive as `foo.bar` — a spelling needing a literal backslash
# cannot be written here.
SUBJECTS=(
  'crates/topo/src/source.rs bit_identity::|eq_bits eq_bits eq_bits(a,b) the bit channel'
  'crates/editor-core/src/product.rs GATHERS|gathers_on_this_thread GATHERS GATHERS.with(get) the debug-only gather counter'
  'crates/mesh/src/curved.rs identified_ids|overused_identified_edge|overused_identified_edge_in identified_ids identified_ids(a) the identified-vertex census the sphere/torus emit pass re-derives'
  'crates/mesh/src/tessellate.rs unpaired_chord_segment unpaired_chord_segment unpaired_chord_segment(a) the chord-segment pairing census'
  'crates/mesh/src/walk.rs overused_identified_edge_in overused_identified_edge_in overused_identified_edge_in(a) the shared identified-vertex fan census'
  'crates/mesh/src/trimmed.rs overused_identified_edge_in overused_identified_edge_in overused_identified_edge_in(a) the trimmed lane call of that fan census'
)
GATE_SCAN_NOUN='debug-only symbol use'

# Emits `USE` for every use of PATTERN and `UNGATED` for every one of
# them not enclosed by a `cfg(debug_assertions)` item. Enclosure is
# brace depth over the code-only text, so a use is placed against the
# item it is in rather than against the file it is in.
debug_only_report() {
  awk -v PAT="$1" '
    BEGIN {
      # A `debug_assert…!` macro, not a function whose name starts the
      # same way: the `!` is the whole distinction.
      DBG = "debug_assert[a-z_]*!"
      # ONE SPELLING AT A TIME, each anchored on an identifier boundary
      # at whichever of its ends is an identifier character. Unanchored,
      # `eq_bits` matches `neq_bits` and `GATHERS` matches `PREGATHERS`,
      # and a use the mechanism never made is counted and placed.
      n = split(PAT, ALT, /\|/)
      for (i = 1; i <= n; i++) {
        lead = (ALT[i] ~ /^[A-Za-z0-9_]/) ? "(^|[^A-Za-z0-9_])" : ""
        tail = (ALT[i] ~ /[A-Za-z0-9_]$/) ? "([^A-Za-z0-9_]|$)" : ""
        ALT[i] = lead "(" ALT[i] ")" tail
      }
    }
    # A READER THAT LOST ITS PLACE HAS DECIDED NOTHING. Reported once
    # per gated item, so one desync cannot bury the rest of the report.
    function report_desync(where, why) {
      print "DESYNC " where ": " why
      dsync = 1
    }
    {
      p1 = index($0, ":"); r = substr($0, p1 + 1); p2 = index(r, ":")
      if (p1 == 0 || p2 == 0) next
      f = substr($0, 1, p1 - 1); ln = substr(r, 1, p2 - 1)
      code = substr(r, p2 + 1)
      if (f != FNAME) {
        if (FNAME != "" && gated == 1 && seen == 0 && bdepth > 0 && dsync == 0)
          report_desync(FNAME ":" gln, "the file ended with brackets still open after this cfg(debug_assertions) attribute")
        FNAME = f; depth = 0; gated = 0; seen = 0; stmt = ""; bdepth = 0
        dsync = 0
      }
      if (gated == 0 &&
          code ~ /#\[cfg\(([^]]*[(,][[:space:]]*)?debug_assertions[,)]/ &&
          code !~ /#\[cfg\([^]]*(any|not)\(/) {
        gated = 1; seen = 0; gdepth = depth; bdepth = 0; dsync = 0; gln = ln
      }
      # Delimiter-wise, so that the statement a use sits in is the
      # statement the `debug_assert!` test asks about, and so that brace
      # depth moves at the brace rather than at the end of the line.
      while (1) {
        if (match(code, /[{};]/)) { cut = RSTART; piece = substr(code, 1, cut - 1) }
        else { cut = 0; piece = code }
        stmt = stmt " " piece
        hit = 0
        for (i = 1; i <= n; i++) if (piece ~ ALT[i]) { hit = 1; break }
        if (hit == 1) {
          print "USE " f ":" ln
          if (gated == 0 && stmt !~ DBG)
            print "UNGATED " f ":" ln ":" piece
        }
        # Round and square brackets are counted, never cut on: they end
        # no statement, and the depth they carry is what separates a `;`
        # inside the signature of an item from the `;` that ends it.
        #
        # WHAT THE COUNT GUARANTEES, in each direction it can be wrong.
        # A NEGATIVE desync — more closers than openers — keeps `<= 0`
        # true, so an item ends early and a use after it FIRES, which is
        # the safe direction. A POSITIVE one would make the item never
        # end and every later use read as gated, silently, so it is not
        # tolerated: brackets still open at the body brace is reported
        # as a reader desync and reds the gate.
        t = piece; bdepth += gsub(/[[(]/, "", t)
        t = piece; bdepth -= gsub(/[])]/, "", t)
        if (cut == 0) break
        d = substr(code, cut, 1)
        if (d == "{") {
          depth++
          if (gated == 1 && seen == 0) {
            if (bdepth <= 0) seen = 1
            else if (dsync == 0)
              report_desync(f ":" ln, "brackets are still open at the body brace of the cfg(debug_assertions) item at line " gln ", so the reader cannot say where that item ends")
          }
        }
        else if (d == "}") {
          depth--
          if (gated == 1 && seen == 1 && depth <= gdepth) gated = 0
        } else if (gated == 1 && seen == 0 && bdepth <= 0) gated = 0
        stmt = ""
        code = substr(code, cut + 1)
      }
    }
    END {
      if (FNAME != "" && gated == 1 && seen == 0 && bdepth > 0 && dsync == 0)
        report_desync(FNAME ":" gln, "the file ended with brackets still open after this cfg(debug_assertions) attribute")
    }
  '
}

gate() {
  local row path pat sym expr noun
  local report ungated desynced uses total=0 proved= failures=0
  # EVERY subject is proved present before ANY is scanned, so a subject
  # that moved out from under the gate cannot be masked by a clean scan
  # of the ones that stayed. `gate_require_file` ends the gate at the
  # FIRST missing subject, so two missing subjects name one.
  for row in "${SUBJECTS[@]}"; do
    read -r path pat sym expr noun <<< "$row"
    gate_require_file "$path"
  done
  for row in "${SUBJECTS[@]}"; do
    read -r path pat sym expr noun <<< "$row"
    report=$(gate_rust_code "$path" | debug_only_report "$pat")
    desynced=$(printf '%s\n' "$report" | gate_grep '^DESYNC ' | sed 's/^DESYNC //')
    if [ -n "$desynced" ]; then
      printf '%s\n' "$desynced"
      # The rule `gate_grep` holds for a matcher that could not run,
      # applied to a reader that could not place what it read: the
      # marker keeps `gate_ok` from printing green over it.
      : >> "$GATE_MATCHER_FAILED"
      gate_error "$path: the reader lost bracket depth (above), so it cannot place a use against the item that encloses it — what it did not place is unknown, which is not a pass"
      failures=$((failures + 1))
      continue
    fi
    ungated=$(printf '%s\n' "$report" | gate_grep '^UNGATED ' | sed 's/^UNGATED //')
    uses=$(printf '%s\n' "$report" | gate_grep -c '^USE ')
    total=$((total + uses))
    if [ -n "$ungated" ]; then
      printf '%s\n' "$ungated"
      gate_error "$path uses $noun above outside any cfg(debug_assertions) item — a debug assertion is the only other place it may stand. One gated use elsewhere in the file does not cover these"
      failures=$((failures + 1))
      continue
    fi
    proved="$proved${proved:+ and of }$noun in $path"
  done
  # Every subject is SCANNED before the gate fails, so one red run names
  # every subject that has an ungated use rather than one per run.
  [ "$failures" -eq 0 ] || exit 1
  GATE_SCAN_FILES=$total
  gate_ok "every use of $proved is inside a cfg(debug_assertions) item or a debug_assert!"
}

# --- THE FIXTURES ------------------------------------------------------
#
# Every planter takes the subject row it writes — its PATH, a bare SYM
# the row's pattern matches, and EXPR, one use of that symbol — and then
# the fixture root, so each case runs once per subject rather than once
# for the subject the matcher was written against. Fixture text is read
# as text and never compiled: it carries the SHAPE the reader decides
# on, not a type-correct program.
plant_source() {
  local path=$1 root=$2
  shift 2
  mkdir -p "$root/$(dirname "$path")"
  printf '%s\n' "$@" > "$root/$path"
}

# The clean tree is every subject, gated.
gate_plant_clean() {
  local row path pat sym expr noun
  for row in "${SUBJECTS[@]}"; do
    read -r path pat sym expr noun <<< "$row"
    plant_source "$path" "$1" \
      '#[cfg(debug_assertions)]' \
      "pub fn agree(a: f64, b: f64) -> bool { $expr }"
  done
}

plant() {
  local path=$1 expr=$3 root=$4
  plant_source "$path" "$root" "pub fn agree(a: f64, b: f64) -> bool { $expr }"
}

# THE CASE THE COUNTING FORM PASSED, and the reason this gate was
# rewritten: one properly gated use, and a production leak beside it.
plant_one_gated_one_leaked() {
  local path=$1 expr=$3 root=$4
  plant_source "$path" "$root" \
    '#[cfg(debug_assertions)]' \
    "pub fn agree(a: f64, b: f64) -> bool { $expr }" \
    'pub fn production_leak(a: f64, b: f64) -> bool {' \
    "    $expr == Some(true)" \
    '}'
}

# The gated item ENDS, and the next use is outside it. Depth, not
# proximity.
plant_after_the_gated_item() {
  local path=$1 expr=$3 root=$4
  plant_source "$path" "$root" \
    '#[cfg(debug_assertions)]' \
    'pub fn agree(a: f64, b: f64) -> bool {' \
    "    $expr" \
    '}' \
    "pub fn later(a: f64, b: f64) -> bool { $expr }"
}

# A `;` INSIDE THE SIGNATURE IS NOT THE ITEM END. An array-typed
# parameter carries one before the body brace, and the use inside the
# item is gated.
plant_semicolon_in_signature() {
  local path=$1 expr=$3 root=$4
  plant_source "$path" "$root" \
    '#[cfg(debug_assertions)]' \
    'pub fn agree<const N: usize>(pairs: [(f64, f64); N]) -> bool {' \
    "    let _ = pairs; $expr" \
    '}'
}

# THE SAME QUESTION FROM THE OTHER SIDE, so that reading a `;` at
# bracket depth zero as an item end stays REQUIRED: a `use` under the
# attribute is an item that ends at its `;`, and the use below it is
# outside the gate.
plant_after_the_gated_use() {
  local path=$1 sym=$2 expr=$3 root=$4
  plant_source "$path" "$root" \
    '#[cfg(debug_assertions)]' \
    "use crate::inner::$sym;" \
    "pub fn later(a: f64, b: f64) -> bool { $expr }"
}

# THE SENTENCE THIS GATE EXISTS TO STOP PRINTING. A `debug_assert!`
# earlier on the LINE is not an enclosure — the statement ended at the
# `;` — and the per-line substring test that read it as one passed this
# fixture while printing *"every use of … is inside a
# cfg(debug_assertions) item or a debug_assert!"*, which is verbatim the
# evidence-free sentence S63 recorded against the form this replaced.
plant_leak_after_debug_assert() {
  local path=$1 expr=$3 root=$4
  plant_source "$path" "$root" \
    "pub fn leak(a: f64, b: f64) -> bool { debug_assert!(a == a); $expr }"
}

# The same enclosure the other way round: a use in a `debug_assert!`
# that rustfmt has wrapped over three lines is inside it, and the
# per-line test called it a violation.
plant_wrapped_debug_assert() {
  local path=$1 expr=$3 root=$4
  plant_source "$path" "$root" \
    'pub fn ok(a: f64, b: f64) {' \
    '    debug_assert!(' \
    "        $expr == Some(true)" \
    '    );' \
    '}'
}

# `all(…)` IS a gate, in EITHER operand order. The first version of this
# rewrite read only `all(debug_assertions, …)` — S56's order-sensitivity,
# minted fresh in the PR that closes S125.
plant_all_cfg_swapped() {
  local path=$1 expr=$3 root=$4
  plant_source "$path" "$root" \
    '#[cfg(all(feature = "probe", debug_assertions))]' \
    "pub fn agree(a: f64, b: f64) -> bool { $expr }"
}

# `not(debug_assertions)` is a RELEASE-only item, so a use inside it is
# a production use.
plant_not_cfg() {
  local path=$1 expr=$3 root=$4
  plant_source "$path" "$root" \
    '#[cfg(not(debug_assertions))]' \
    "pub fn agree(a: f64, b: f64) -> bool { $expr }"
}

# `any(debug_assertions, …)` is not a debug-only gate.
plant_any_cfg() {
  local path=$1 expr=$3 root=$4
  plant_source "$path" "$root" \
    '#[cfg(any(debug_assertions, feature = "probe"))]' \
    "pub fn agree(a: f64, b: f64) -> bool { $expr }"
}

# THE NEAR MISSES. A use inside a `debug_assert!` is debug-only by
# construction; a use named in prose is not a use at all, and counting
# one is how this gate announced a number that was not the number of
# calls.
plant_permitted_shapes() {
  local path=$1 sym=$2 expr=$3 root=$4
  plant_source "$path" "$root" \
    "/// The one call site: $sym, cfg(debug_assertions)-gated." \
    "// $sym is named here and used nowhere." \
    "/*" \
    " * Nor is $sym used inside this block comment." \
    " */" \
    "pub const NOTE: &str = \"$sym\";" \
    'pub fn checked(a: f64, b: f64) {' \
    "    debug_assert!($expr == Some(true));" \
    '}' \
    '#[cfg(debug_assertions)]' \
    'mod inner {' \
    "    pub fn agree(a: f64, b: f64) -> bool { super::$expr }" \
    '}'
}

# A LOST BRACKET DEPTH IS REPORTED, NOT PASSED. `lib.sh`'s reader does
# not nest block comments, so the first `*/` closes the outer one and
# `/* outer /* inner */ ( */` leaves a stray `(` in the code view.
# Bracket depth then never returns to zero, the item never reads as
# entered, and every use to the end of the file would read as gated.
plant_desync_nested_block_comment() {
  local path=$1 expr=$3 root=$4
  plant_source "$path" "$root" \
    '#[cfg(debug_assertions)]' \
    '/* outer /* inner */ ( */' \
    "pub fn agree(a: f64, b: f64) -> bool { $expr }" \
    "pub fn leak(a: f64, b: f64) -> bool { $expr }"
}

# THE SAME LOSS WITH NO BRACE AFTER IT. The file ends inside the stray
# bracket, so the body-brace arm is never reached and the end of the
# file is where the reader has to say so. Every use here is gated, so
# only the desync can red this fixture.
plant_desync_at_end_of_file() {
  local path=$1 expr=$3 root=$4
  plant_source "$path" "$root" \
    '#[cfg(debug_assertions)]' \
    "pub fn agree(a: f64, b: f64) -> bool { $expr }" \
    '#[cfg(debug_assertions)]' \
    '/* outer /* inner */ ( */'
}

# THE NEAR MISS AT THE IDENTIFIER BOUNDARY: a longer name that merely
# CONTAINS the symbol is not a use of it.
plant_near_miss_identifier() {
  local path=$1 sym=$2 root=$4
  plant_source "$path" "$root" \
    "pub fn near(a: f64, b: f64) -> bool { pre${sym}_post(a, b) }"
}

# The subject removed out from under the gate — one row of the list, so
# the other subjects are clean and only the missing one can fail it.
plant_subject_gone() { rm -f "$2/$1"; }

gate_selftest() {
  local want="outside any cfg(debug_assertions) item"
  gate_selftest_clean
  # A `grep` that cannot run is the failure this gate cannot see for
  # itself: it produces no hits, and no hits is what a clean tree
  # produces. Proved here rather than asserted, because before
  # `gate_grep` this exact fixture printed OK and exited 0.
  gate_selftest_without_tool grep "it is grep saying it could not search"
  local row path pat sym expr noun
  for row in "${SUBJECTS[@]}"; do
    read -r path pat sym expr noun <<< "$row"
    gate_selftest_case "$want" plant "$path" "$sym" "$expr"
    gate_selftest_case "$want" plant_one_gated_one_leaked "$path" "$sym" "$expr"
    gate_selftest_case "$want" plant_after_the_gated_item "$path" "$sym" "$expr"
    gate_selftest_case "$want" plant_after_the_gated_use "$path" "$sym" "$expr"
    gate_selftest_case "$want" plant_any_cfg "$path" "$sym" "$expr"
    gate_selftest_case "$want" plant_not_cfg "$path" "$sym" "$expr"
    gate_selftest_case "$want" plant_leak_after_debug_assert "$path" "$sym" "$expr"
    gate_selftest_case "the reader lost bracket depth" \
      plant_desync_nested_block_comment "$path" "$sym" "$expr"
    gate_selftest_case "the reader lost bracket depth" \
      plant_desync_at_end_of_file "$path" "$sym" "$expr"
    gate_selftest_case "the gate's subject is gone" plant_subject_gone "$path"
    gate_selftest_passes "a debug_assert!, prose, a string literal and a gated inner module" \
      plant_permitted_shapes "$path" "$sym" "$expr"
    gate_selftest_passes "a rustfmt-wrapped debug_assert! around the use" \
      plant_wrapped_debug_assert "$path" "$sym" "$expr"
    gate_selftest_passes "cfg(all(…)) with debug_assertions as the SECOND operand" \
      plant_all_cfg_swapped "$path" "$sym" "$expr"
    gate_selftest_passes 'a `;` inside a gated signature' \
      plant_semicolon_in_signature "$path" "$sym" "$expr"
    gate_selftest_passes "a longer identifier that merely contains the symbol" \
      plant_near_miss_identifier "$path" "$sym" "$expr"
  done
  printf '%s selftest OK, over %s subjects, each proved on its own: enclosure by brace depth and by `debug_assert!` statement (rustfmt-wrapped or not); `all(…)` gates in either operand order while `any(…)` and `not(…)` do not; prose, string literals and a longer identifier that merely contains the symbol are not uses; an item ends at a `;` only at bracket depth zero; and a lost bracket depth, a missing subject and a `grep` that cannot run are each a loud failure\n' \
    "$(gate_name)" "${#SUBJECTS[@]}"
}

gate_parse_args "$@"
gate_main
