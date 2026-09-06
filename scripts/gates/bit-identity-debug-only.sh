#!/usr/bin/env bash
# bit-identity-debug-only.sh — the debug-only subjects stay debug-only.
# ONE home; ci.yml's `discipline` job and local-scripts/ci-local.sh's
# discipline row both call this file by name.
#
# A SUBJECT LIST, NOT A FILE. `SUBJECTS` carries one row per file that
# owes a symbol to `cfg(debug_assertions)`: the path, the spellings
# whose uses are gated, and the count of uses this tree carries.
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
#   * `crates/mesh/src/trimmed.rs` — the trimmed lane's call of
#     `overused_identified_edge_in`. A SUBJECT IS A FILE, so a symbol
#     whose uses cross files owes a row per file that uses it; a row
#     covering the definition alone pins the attribute on the definition
#     and nothing about the call sites (KNOWN GAP 7 for what that
#     design still cannot see).
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
# twelve of its sites take that shape, in five of the seven `topo` files
# whose CODE names it (an eighth names it in a doc comment only).
#
# KNOWN GAP 7: A SUBJECT IS A FILE, so a symbol whose uses cross files
# owes a row per file that uses it, and a NEW file that calls one is
# caught by nothing here. Nothing else catches it either: such a call
# compiles in every configuration this repo builds — the workspace's
# `[profile.release]` keeps debug assertions on, and the one
# debug-assertions-off job never compiles `mesh` — so the first build
# that would refuse it is a consumer's, after publish. The rows below
# say where the gated symbols are used TODAY; a fifth caller of the
# identified-vertex census is a row this file does not have yet, and
# adding the caller is what has to add it.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

# PATH, the symbol spellings, the PINNED USE COUNT this tree carries,
# and the noun the diagnosis calls the mechanism by. Only the noun may
# carry spaces, and it is last for that reason.
#
# The symbol field is a `|`-SEPARATED LIST of spellings, each matched at
# identifier boundaries, and not a general ERE: the reader splits on
# `|`, so a `|` inside a group would be read as a separator. It reaches
# awk through `-v`, which processes backslash escapes, so `foo\.bar`
# would arrive as `foo.bar` — a spelling needing a literal backslash
# cannot be written here.
#
# NO ROW NAMES A FIXTURE SYMBOL, and it used to name two. A row carried
# a bare SYM and one EXPR beside its spelling list, the self-test
# planted those two fields, and a row's SECOND and later spellings were
# therefore held by nothing: misspell one and the live scan quietly
# stops counting its uses while every fixture stays green. The fixtures
# now DERIVE a symbol and a use from each spelling in turn
# (`spelling_sym`, `spelling_use`), so what the list holds is what the
# list plants — and fixture text is read as text and never compiled, so
# a derived `GATHERS(a)` carries the same shape a hand-written
# `GATHERS.with(get)` did.
#
# THE PIN IS THE OTHER HALF OF THAT, because deriving the fixtures
# proves each spelling is READ and not that it is USED. A row pins the
# number of uses its file carries and `gate` re-derives it every run, so
# a spelling that stops matching — misspelt, renamed under it, or moved
# to a file with no row — reds with the row named rather than lowering
# the total in silence.
SUBJECTS=(
  'crates/topo/src/source.rs bit_identity::|eq_bits 1 the bit channel'
  'crates/editor-core/src/product.rs GATHERS|gathers_on_this_thread 4 the debug-only gather counter'
  'crates/mesh/src/curved.rs identified_ids|overused_identified_edge|overused_identified_edge_in 13 the identified-vertex census the sphere/torus emit pass re-derives'
  'crates/mesh/src/tessellate.rs unpaired_chord_segment 8 the chord-segment pairing census'
  'crates/mesh/src/walk.rs overused_identified_edge_in 1 the shared identified-vertex fan census'
  'crates/mesh/src/trimmed.rs overused_identified_edge_in 1 the trimmed lane call of the identified-vertex fan census'
)
GATE_SCAN_NOUN='debug-only symbol use'

# `--pin-shift N` shifts every row's pinned count by N. It exists for
# the self-test and for nothing else: a pin is a reading of THIS tree,
# so the case that proves the comparison fires perturbs the PIN against
# the real tree rather than perturbing a fixture tree against the pin.
GATE_PIN_SHIFT=0

# A bare symbol, and one use of it, derived from a spelling. A trailing
# `::` is a path prefix rather than a name, so the symbol drops it and
# the use completes it into a call.
spelling_sym() { printf '%s' "${1%::}"; }
spelling_use() {
  case "$1" in
    *::) printf '%scall(a)' "$1" ;;
    *) printf '%s(a)' "$1" ;;
  esac
}

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
  local row path pat count noun
  local report ungated desynced uses pinned total=0 proved= failures=0
  # THE PIN IS CHECKED AGAINST THE TREE IT WAS READ FROM. A row pins how
  # many uses ITS file carries here, so the comparison is meaningful
  # exactly when the gate is reading this repo and not a fixture tree —
  # a fixture carries whatever its planter wrote, and several planters
  # cannot write a row's count at all (a `pin 1` row has no room for a
  # gated use and a leaked one). What proves the comparison fires is
  # `--pin-shift`, which perturbs the pin against the real tree.
  local pins_live=false
  [ "$GATE_ROOT" = "$GATE_REPO_ROOT" ] && pins_live=true
  # EVERY subject is proved present before ANY is scanned, so a subject
  # that moved out from under the gate cannot be masked by a clean scan
  # of the ones that stayed. `gate_require_file` ends the gate at the
  # FIRST missing subject, so two missing subjects name one.
  for row in "${SUBJECTS[@]}"; do
    read -r path pat count noun <<< "$row"
    gate_require_file "$path"
  done
  for row in "${SUBJECTS[@]}"; do
    read -r path pat count noun <<< "$row"
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
    # AFTER the two checks above, so a planted leak reds as a leak: a
    # fixture that moves the count moves it for a reason the enclosure
    # checks have already named.
    if [ "$pins_live" = true ]; then
      pinned=$((count + GATE_PIN_SHIFT))
      if [ "$uses" -ne "$pinned" ]; then
        gate_error "$path carries $uses uses of $noun where its row pins $pinned. A row pins its file's use count so that a spelling which stops matching — misspelt in the list, renamed under it, or moved to a file with no row — reds here instead of lowering the total in silence. A count that FELL means the gate is holding less than the row claims; one that ROSE means the mechanism grew where nobody re-read the enclosure argument. Neither is repaired by editing the number on its own"
        failures=$((failures + 1))
        continue
      fi
    fi
    proved="$proved${proved:+ and of }$noun in $path"
  done
  # Every subject is SCANNED before the gate fails, so one red run names
  # every subject that has an ungated use rather than one per run.
  [ "$failures" -eq 0 ] || exit 1
  GATE_SCAN_FILES=$total
  if [ "$pins_live" = true ]; then
    gate_ok "every use of $proved is inside a cfg(debug_assertions) item or a debug_assert!, and every row carries the use count it pins"
  else
    gate_ok "every use of $proved is inside a cfg(debug_assertions) item or a debug_assert!"
  fi
}

# --- THE FIXTURES ------------------------------------------------------
#
# Every planter takes the subject row it writes — its PATH, a bare SYM
# and one USE of that symbol, both DERIVED from one of the row's
# spellings — and then the fixture root, so each case runs once per
# subject rather than once for the subject the matcher was written
# against. Fixture text is read as text and never compiled: it carries
# the SHAPE the reader decides on, not a type-correct program.
#
# EVERY SPELLING OF A ROW IS PLANTED, and that is the whole reason the
# symbol is derived. The clean tree gives each spelling its own gated
# item, and the basic must-fire case runs once per spelling, so a
# spelling that the reader cannot match fails the self-test in both
# directions: the clean tree loses a use, and the leak planted with that
# spelling is not seen.
plant_source() {
  local path=$1 root=$2
  shift 2
  mkdir -p "$root/$(dirname "$path")"
  printf '%s\n' "$@" > "$root/$path"
}

# The clean tree is every subject, gated, with one item per SPELLING.
gate_plant_clean() {
  local row path pat count noun alt i lines spelling
  for row in "${SUBJECTS[@]}"; do
    read -r path pat count noun <<< "$row"
    lines=()
    i=0
    IFS='|' read -r -a alt <<< "$pat"
    for spelling in "${alt[@]}"; do
      i=$((i + 1))
      lines+=('#[cfg(debug_assertions)]' \
        "pub fn agree_$i(a: f64, b: f64) -> bool { $(spelling_use "$spelling") }")
    done
    plant_source "$path" "$1" "${lines[@]}"
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

# THE PIN, PROVED AGAINST THE TREE IT IS A READING OF. No fixture can
# carry a row's pinned count — a `pin 1` row has no room for a gated use
# and a leaked one in the same file — so the case that proves the
# comparison fires perturbs the PIN instead, over the real subjects, and
# must red naming every row. Run in both directions, because `-ne`
# written as `-lt` would pass a count that ROSE.
gate_selftest_pin() {
  local shift_by=$1 out row path pat count noun
  if out=$("$0" --pin-shift "$shift_by" 2>&1); then
    printf 'SELFTEST FAILED: the gate PASSED with every pin shifted by %s — the pinned use counts are not being compared\n%s\n' \
      "$shift_by" "$out" >&2
    exit 1
  fi
  for row in "${SUBJECTS[@]}"; do
    read -r path pat count noun <<< "$row"
    case "$out" in
      *"$path carries $count uses"*) ;;
      *) printf 'SELFTEST FAILED: a pin shifted by %s did not red for %s, so that row is pinned by nothing:\n%s\n' \
           "$shift_by" "$path" "$out" >&2
         exit 1 ;;
    esac
  done
}

gate_selftest() {
  local want="outside any cfg(debug_assertions) item"
  gate_selftest_clean
  # A `grep` that cannot run is the failure this gate cannot see for
  # itself: it produces no hits, and no hits is what a clean tree
  # produces. Proved here rather than asserted, because before
  # `gate_grep` this exact fixture printed OK and exited 0.
  gate_selftest_without_tool grep "it is grep saying it could not search"
  local row path pat count noun alt spelling sym expr spellings=0
  for row in "${SUBJECTS[@]}"; do
    read -r path pat count noun <<< "$row"
    # THE BASIC MUST-FIRE CASE RUNS ONCE PER SPELLING. A row's second
    # and later spellings are exactly what the old fixtures held
    # nothing against, so each of them plants its own leak and each has
    # to be caught on its own.
    IFS='|' read -r -a alt <<< "$pat"
    for spelling in "${alt[@]}"; do
      spellings=$((spellings + 1))
      gate_selftest_case "$want" plant \
        "$path" "$(spelling_sym "$spelling")" "$(spelling_use "$spelling")"
    done
    # The rest of the shapes are about ENCLOSURE, which is a property of
    # the reader and not of the spelling, so they run once per row on
    # the row's first spelling.
    sym=$(spelling_sym "${alt[0]}"); expr=$(spelling_use "${alt[0]}")
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
  gate_selftest_pin 1
  gate_selftest_pin -1
  printf '%s selftest OK, over %s subjects and the %s spellings they carry, each proved on its own: every spelling plants its own leak, so a spelling the reader cannot match fails here; enclosure by brace depth and by `debug_assert!` statement (rustfmt-wrapped or not); `all(…)` gates in either operand order while `any(…)` and `not(…)` do not; prose, string literals and a longer identifier that merely contains the symbol are not uses; an item ends at a `;` only at bracket depth zero; each row carries the use count it pins, proved against this tree in both directions; and a lost bracket depth, a missing subject and a `grep` that cannot run are each a loud failure\n' \
    "$(gate_name)" "${#SUBJECTS[@]}" "$spellings"
}

# `--pin-shift` is taken out of argv here rather than in `lib.sh`, which
# rejects a flag it does not know: it is this gate's own, and every
# other argument reaches the shared parser unchanged.
GATE_ARGV=()
while [ $# -gt 0 ]; do
  case "$1" in
    --pin-shift) GATE_PIN_SHIFT=$2; shift 2 ;;
    *) GATE_ARGV+=("$1"); shift ;;
  esac
done
gate_parse_args ${GATE_ARGV[@]+"${GATE_ARGV[@]}"}
gate_main
