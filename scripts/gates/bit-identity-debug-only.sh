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
#     guarantee. The row pins the channel call (`bit_identity::`,
#     `eq_bits`) AND the three debug-only witnesses built on it —
#     `plane_bits_witness`, `vec3_bits_witness`, `bits_witness` — whose
#     own gated `fn` heads name the channel nowhere. A ROW HOLDS THE
#     ATTRIBUTES ON THE STATEMENTS THAT NAME ITS SPELLINGS AND NO
#     OTHERS, so a mechanism's every spelling belongs on its row.
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
#   * `crates/topo/src/{euler,euler_ring,euler_kill,null,split,movefac}.rs`
#     and `crates/topo/src/boolean/voids.rs` — the arena-delta
#     mechanism, in EVERY spelling its statements name: `ArenaDelta`,
#     the arena shift each euler operator declares;
#     `assert_euler_postcondition`, which checks it against the counts
#     taken before the mutation; `arena_counts`, which takes those
#     counts; and `ArenaCounts`, their type, on the one row whose file
#     names it. One row per `topo` file whose CODE names any of them,
#     per the rule above; a `topo` file that names one in a doc comment
#     only has no uses to pin and so has no row. Most of the sites put
#     the attribute in STATEMENT position over the multi-line call that
#     reads the delta, which is what the third enclosure clause below
#     places.
#
#     WHY EVERY SPELLING AND NOT THE HEADLINE ONE. A row holds the
#     attributes on the statements that NAME its spellings, so a
#     statement of the same mechanism naming none of them is invisible
#     here however debug-only it is: `let before = self.arena_counts();`
#     names no delta and no assert, and `voids.rs`'s postcondition call
#     passes a binding and so names no struct literal. All four
#     spellings are debug-only in their own right — `arena_counts` and
#     `ArenaCounts` live in `test_support_impl`, mounted under
#     `#[cfg(any(debug_assertions, test, feature = "test-support"))]`,
#     so an ungated user of either stops a consumer's release build
#     compiling.
#
# `#[cfg(test)]` CODE IS NOT SCANNED (`gate_rust_code --skip-cfg-test`),
# and that is the same question this gate asks everywhere else: what a
# CONSUMER'S RELEASE BUILD compiles. A test module is compiled by
# `cargo test` and by nothing a consumer runs, so a debug-only symbol
# used inside one can never break their build, and reading it can only
# report a violation that is not one — which is the direction that
# pushes a gate toward an allowlist. It also decides which spellings a
# row can carry: a type named freely in a crate's own test modules, as
# `ArenaCounts` is, is a spelling this reader could not serve while it
# read them.
#
# SO THE PINS COUNT PRODUCTION USES, and a row whose file carries the
# mechanism in its tests too pins fewer than the file names — the mesh
# rows most of all, where the census helpers are read by test rows
# beside their live callers. A pin is a reading, not a target: when the
# reading gets righter the pin is re-taken.
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
#   * AN ITEM UNDER A STATEMENT-POSITION ATTRIBUTE HAS NO BODY BRACE,
#     and that is the same depth test read at the `{` instead of at the
#     `;`. Which delimiter arrives at bracket depth ZERO first says what
#     the item is: a `{` there is the item entering, and the item then
#     runs to the `}` that balances it; a `;` there ends an item that
#     was never entered. A `{` met at bracket depth ABOVE zero is
#     neither — it is argument text, a struct literal or a block
#     expression inside a call's still-open `(` — so it moves brace
#     depth and nothing else. `topo`'s
#     `self.assert_euler_postcondition(before, ArenaDelta { … }, "op");`
#     sites are the live population: the item is the CALL, and it ends
#     at the call's `;`. How many there are is what the rows' pins say,
#     and re-derive on every run.
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
# would is one this reader cannot serve. The match is also
# CASE-SENSITIVE, so `arena_counts` and `ArenaCounts` are two spellings
# and not one; `euler.rs`'s row pins both, and its pin is what says the
# reader keeps them apart.
#
# KNOWN GAP 4: a rustfmt-wrapped attribute — `#[cfg(` and
# `debug_assertions` and `)]` on three lines — is not read as a gate,
# because the attribute matcher reads one line, so a use inside that
# item fires. Cry-wolf, and no such site exists here.
#
# KNOWN GAP 5: a `{ … }` const-generic default in a gated signature
# marks the item entered at that brace, so the item reads as closed at
# the matching `}`. Both what follows that `}` inside the SIGNATURE — a
# parameter typed `[ArenaDelta; N]`, say — and the real body then fire.
# Angle brackets are not counted, so the default's brace sits at bracket
# depth zero and the statement-position rule does not reach it.
# Cry-wolf again, in the same direction.
#
# THE GAP NUMBERS ARE STABLE IDS, not a sequence, so a gap that is
# closed leaves a hole rather than renumbering its neighbours. There is
# no KNOWN GAP 6: an attribute in STATEMENT position over a multi-line
# call whose arguments carry a brace is a shape the reader places, and
# the rule it places it by is the third enclosure clause above rather
# than an entry here. Nothing is missing from the register.
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
  'crates/topo/src/source.rs bit_identity::|eq_bits|plane_bits_witness|vec3_bits_witness|bits_witness 6 the bit channel'
  'crates/editor-core/src/product.rs GATHERS|gathers_on_this_thread 4 the debug-only gather counter'
  'crates/mesh/src/curved.rs identified_ids|overused_identified_edge|overused_identified_edge_in 5 the identified-vertex census the sphere/torus emit pass re-derives'
  'crates/mesh/src/tessellate.rs unpaired_chord_segment 2 the chord-segment pairing census'
  'crates/mesh/src/walk.rs overused_identified_edge_in 1 the shared identified-vertex fan census'
  'crates/mesh/src/trimmed.rs overused_identified_edge_in 1 the trimmed lane call of the identified-vertex fan census'
  'crates/topo/src/euler.rs ArenaDelta|assert_euler_postcondition|arena_counts|ArenaCounts 21 the arena delta the euler operators declare'
  'crates/topo/src/euler_ring.rs ArenaDelta|assert_euler_postcondition|arena_counts 17 the arena delta the ring operators declare'
  'crates/topo/src/euler_kill.rs ArenaDelta|assert_euler_postcondition|arena_counts 17 the arena delta the kill-direction operators declare'
  'crates/topo/src/null.rs ArenaDelta|assert_euler_postcondition|arena_counts 5 the arena delta the null-entity operators declare'
  'crates/topo/src/split.rs ArenaDelta|assert_euler_postcondition|arena_counts 5 the arena delta the split operators declare'
  'crates/topo/src/boolean/voids.rs ArenaDelta|assert_euler_postcondition|arena_counts 4 the arena delta the void transplant declares'
  'crates/topo/src/movefac.rs ArenaDelta|assert_euler_postcondition|arena_counts 6 the arena delta the face-move operator declares'
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
# THE SAME NAME IN ANOTHER CASE, in the two shapes Rust spells one in,
# derived from a symbol so the near-miss fixture can plant them. Both
# have the same home for the same reason the two above do: what a
# fixture plants is derived from the row, never written beside it.
spelling_camel() {
  printf '%s' "$1" | awk '{
    n = split($0, p, "_"); s = ""
    for (i = 1; i <= n; i++) s = s toupper(substr(p[i], 1, 1)) substr(p[i], 2)
    printf "%s", s
  }'
}
spelling_upper() { printf '%s' "$1" | tr 'a-z' 'A-Z'; }

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
      # The per-file reset. It carries no end-of-file check of its own:
      # `gate` invokes this reader once per subject, over the records of
      # that one subject, so the only file boundary a run ever meets is
      # at the first record, where no previous file is open to close.
      if (f != FNAME) {
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
        # tolerated: an attribute whose brackets are STILL OPEN when the
        # file ends is reported as a reader desync and reds the gate.
        #
        # THE END OF THE FILE IS WHERE THAT IS ASKED, because a `{`
        # inside an open bracket is ordinary argument text (the arm
        # below) rather than evidence of anything. Depth that never
        # returns to zero is what is left.
        #
        # WHERE ONE COMES FROM, now that the shared lexer nests block
        # comments: not from a nested `/* /* */ */`, which it reads
        # whole. Two sources are left.
        #
        #   * Source that is genuinely unbalanced, which does not
        #     compile and so should not reach a gate. The fixtures plant
        #     this shape, because it is the one that needs no gap.
        #   * A lexer blind spot yet to be found.
        #
        # The guard is defensive across both: it does not diagnose the
        # cause, only that this gate cannot place a use.
        t = piece; bdepth += gsub(/[[(]/, "", t)
        t = piece; bdepth -= gsub(/[])]/, "", t)
        if (cut == 0) break
        d = substr(code, cut, 1)
        # The three arms below execute the THIRD ENCLOSURE CLAUSE in the
        # header: only a `{` at bracket depth zero enters an item, and
        # only a `;` at bracket depth zero ends one that was never
        # entered.
        if (d == "{") {
          depth++
          if (gated == 1 && seen == 0 && bdepth <= 0) seen = 1
        }
        else if (d == "}") {
          depth--
          if (gated == 1 && seen == 1 && depth <= gdepth) gated = 0
        } else if (gated == 1 && seen == 0 && bdepth <= 0) gated = 0
        stmt = ""
        code = substr(code, cut + 1)
      }
    }
    # THE ONE PLACE A LOST BRACKET DEPTH IS ASKED ABOUT, and it is one
    # place because the reader is invoked once per subject: the end of
    # the input IS the end of the file, so no earlier boundary can carry
    # this check and no later one exists. An attribute whose brackets
    # never closed leaves `gated` set with `seen` clear and depth above
    # zero, and that is a reader that cannot say where its item ended.
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
    report=$(gate_rust_code --skip-cfg-test "$path" | debug_only_report "$pat")
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

# `topo`'s LIVE SHAPE: a statement-position attribute over one call,
# wrapped over several lines, with a struct literal in an argument. Both
# uses are inside the gated call, so this must PASS.
#
# THE SECOND USE SITS AFTER THE ARGUMENT BRACE CLOSES, and that is what
# this case holds: a reader that took that brace for the item's body
# would end the item at the matching `}` — still inside the call — and
# report the second use as a production leak.
plant_statement_attribute_over_a_braced_call() {
  local path=$1 expr=$3 root=$4
  plant_source "$path" "$root" \
    'pub fn op(a: f64, b: f64) -> Result<(), ()> {' \
    '    #[cfg(debug_assertions)]' \
    '    self.assert_shift(' \
    '        before,' \
    "        Shift { first: $expr, ..Shift::ZERO }," \
    "        $expr," \
    '    );' \
    '    Ok(())' \
    '}'
}

# AND THE SAME SHAPE WITH A USE BELOW IT, so that placing the statement
# stays a PLACEMENT and not a licence. This case holds the `;`: a reader
# that never ended the item there would carry the gate on down the
# function and read this leak as gated.
plant_leak_after_a_statement_attribute() {
  local path=$1 expr=$3 root=$4
  plant_source "$path" "$root" \
    'pub fn op(a: f64, b: f64) -> Result<(), ()> {' \
    '    #[cfg(debug_assertions)]' \
    '    self.assert_shift(' \
    '        before,' \
    "        Shift { first: $expr, ..Shift::ZERO }," \
    '        "op",' \
    '    );' \
    "    let _ = $expr;" \
    '    Ok(())' \
    '}'
}

# BRACES NESTED AND IN MORE THAN ONE ARGUMENT. This case holds that the
# depth is a COUNT: a reader that merely skipped the first brace it met
# inside the call would serve the shape above and enter the item at the
# second, ending the gate before the trailing use.
plant_statement_attribute_with_nested_braces() {
  local path=$1 expr=$3 root=$4
  plant_source "$path" "$root" \
    'pub fn op(a: f64, b: f64) -> Result<(), ()> {' \
    '    #[cfg(debug_assertions)]' \
    "    self.assert_shift(Shift { first: $expr }, Shift { second: $expr }," \
    "        Outer { inner: Shift { deep: $expr } }," \
    "        $expr);" \
    '    Ok(())' \
    '}'
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

# A LOST BRACKET DEPTH IS REPORTED, NOT PASSED. A bracket left open in
# the code view — for any reason — means depth never returns to zero,
# the gated item never reads as entered, and every use to the end of
# the file would read as gated. That is the silent direction, so it is
# reported instead.
#
# THE STRAY BRACKET IS PLANTED AS CODE, not smuggled through a gap in
# the reader. It used to arrive as `/* outer /* inner */ ( */`, which
# worked only while `lib.sh`'s lexer closed a block comment at the
# FIRST `*/` and read the tail as code; the lexer nests now, so that
# line is a comment and reaches nothing. Planting the bracket directly
# keeps the whole path proved — this arm, the marker it writes, and
# `gate_ok`'s refusal to print over it — which a fixture that fed the
# awk a synthetic view could not.
#
# THE BRACES BELOW THE STRAY BRACKET DO NOT REPAIR IT, which is what
# this case holds that its end-of-file twin does not: two whole braced
# items follow, each opening and closing at brace depth, and the
# BRACKET depth they sit inside is still one when the file ends.
plant_desync_open_bracket() {
  local path=$1 expr=$3 root=$4
  plant_source "$path" "$root" \
    '#[cfg(debug_assertions)]' \
    '(' \
    "pub fn agree(a: f64, b: f64) -> bool { $expr }" \
    "pub fn leak(a: f64, b: f64) -> bool { $expr }"
}

# THE SAME LOSS WITH NOTHING AFTER IT. The attribute is the last thing
# in the file, so no later delimiter could end its item even if the
# depth were sound. Every use here is gated, so only the desync can red
# this fixture.
plant_desync_open_bracket_at_end_of_file() {
  local path=$1 expr=$3 root=$4
  plant_source "$path" "$root" \
    '#[cfg(debug_assertions)]' \
    "pub fn agree(a: f64, b: f64) -> bool { $expr }" \
    '#[cfg(debug_assertions)]' \
    '('
}

# AND THE SHAPE THAT USED TO DESYNC, KEPT AS THE CONTROL THAT SAYS IT NO
# LONGER DOES. The reader nests block comments, so the whole line is a
# comment: the attribute gates `agree`, and `leak` below it is an
# ORDINARY ungated use. This case fires on the leak, not on a desync —
# which is the difference between a reader that lost its place and one
# that did not.
plant_nested_block_comment_then_a_leak() {
  local path=$1 expr=$3 root=$4
  plant_source "$path" "$root" \
    '#[cfg(debug_assertions)]' \
    '/* outer /* inner */ ( */' \
    "pub fn agree(a: f64, b: f64) -> bool { $expr }" \
    "pub fn leak(a: f64, b: f64) -> bool { $expr }"
}

# The same comment closing the file, where every use IS gated: nothing
# is left over, so this must PASS. Its twin above fires; between them
# the nested comment is pinned in both directions.
plant_nested_block_comment_closing_the_file() {
  local path=$1 expr=$3 root=$4
  plant_source "$path" "$root" \
    '#[cfg(debug_assertions)]' \
    "pub fn agree(a: f64, b: f64) -> bool { $expr }" \
    '#[cfg(debug_assertions)]' \
    '/* outer /* inner */ ( */'
}

# THE NEAR MISSES AT THE IDENTIFIER BOUNDARY: a longer name that merely
# CONTAINS the symbol is not a use of it, and neither is the same name
# in another case. Four shapes, because they are four claims.
#
#   * `pre<sym>_post` needs BOTH anchors gone to match, so on its own it
#     says nothing about either one.
#   * `<sym>_before` — the shape a field takes — needs the TAIL anchor.
#   * `saved_<sym>` needs the LEAD anchor.
#   * `<SYM>` in SCREAMING_SNAKE needs the match to fold case; the
#     CamelCase form does not, because a fold does not remove the
#     underscore that keeps `ArenaCounts` from containing `arena_counts`
#     — for a single-word lowercase spelling it would, and this line is
#     load-bearing for one. No spelling here is that shape today.
#
# PER SPELLING, NOT PER ROW, because an anchor is a property of the
# SPELLING: the reader derives each end's anchor from whether that end
# of the spelling is an identifier character, so a row's second and
# later spellings are held by a case run on its first only where their
# ends happen to agree. `bit_identity::` ends in a `:` and carries no
# tail anchor at all, and the `<sym>_before` line is that spelling's
# CONTROL rather than its test — which is the reading the per-row form
# gave every spelling silently.
#
# A DERIVED FORM THAT IS ITSELF A SPELLING OF THE ROW IS DROPPED, and
# so is one that equals the symbol (`GATHERS` is its own upper form):
# either would plant a real, ungated use and fire a case whose whole
# claim is that it does not. The row's spelling list is the third
# argument for that reason, where the other planters take a use.
plant_near_miss_identifier() {
  local path=$1 sym=$2 pat=$3 root=$4 form lines
  lines=(
    "pub fn near(a: f64, b: f64) -> bool { pre${sym}_post(a, b) }"
    "pub struct Near {"
    "    pub ${sym}_before: usize,"
    "    pub saved_${sym}: usize,"
    '}'
  )
  form=$(spelling_camel "$sym")
  near_miss_is_new "$form" "$sym" "$pat" &&
    lines+=("pub struct $form { pub n: usize }")
  form=$(spelling_upper "$sym")
  near_miss_is_new "$form" "$sym" "$pat" &&
    lines+=("pub const $form: usize = 0;")
  plant_source "$path" "$root" "${lines[@]}"
}

# A derived form is plantable when it is neither the symbol it came from
# nor any other spelling on the row.
near_miss_is_new() {
  [ "$1" != "$2" ] || return 1
  case "|$3|" in *"|$1|"*) return 1 ;; esac
  case "|$3|" in *"|$1::|"*) return 1 ;; esac
  return 0
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
    # TWO WAYS TO MISS, AND THEY WANT DIFFERENT WORDS. A row that did
    # not red at all is pinned by nothing. A row that red with a count
    # OTHER than its own is pinned, and what is stale is the number in
    # the list — the reading the tree carries has moved. Saying the
    # first of the second sends a reader looking for a matcher bug in a
    # gate that is working.
    case "$out" in
      *"$path carries $count uses"*) ;;
      *"$path carries "*)
        printf 'SELFTEST FAILED: a pin shifted by %s red for %s, but with a use count that is not the %s its row pins — the row is compared and the NUMBER is stale, so re-take the pin against this tree:\n%s\n' \
          "$shift_by" "$path" "$count" "$out" >&2
        exit 1 ;;
      *) printf 'SELFTEST FAILED: a pin shifted by %s did not red for %s at all, so that row is pinned by nothing:\n%s\n' \
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
    # THE TWO CASES THAT ARE ABOUT THE SPELLING RUN ONCE PER SPELLING:
    # the basic must-fire leak, because a row's second and later
    # spellings are exactly what the old fixtures held nothing against,
    # and the near misses, because the reader derives each end's anchor
    # from that end of the SPELLING.
    IFS='|' read -r -a alt <<< "$pat"
    for spelling in "${alt[@]}"; do
      spellings=$((spellings + 1))
      gate_selftest_case "$want" plant \
        "$path" "$(spelling_sym "$spelling")" "$(spelling_use "$spelling")"
      gate_selftest_passes "longer identifiers that merely contain $spelling, at each end alone, and the same name in another case" \
        plant_near_miss_identifier "$path" "$(spelling_sym "$spelling")" "$pat"
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
    gate_selftest_case "$want" \
      plant_leak_after_a_statement_attribute "$path" "$sym" "$expr"
    gate_selftest_case "the reader lost bracket depth" \
      plant_desync_open_bracket "$path" "$sym" "$expr"
    gate_selftest_case "the reader lost bracket depth" \
      plant_desync_open_bracket_at_end_of_file "$path" "$sym" "$expr"
    gate_selftest_case "$want" \
      plant_nested_block_comment_then_a_leak "$path" "$sym" "$expr"
    gate_selftest_case "the gate's subject is gone" plant_subject_gone "$path"
    gate_selftest_passes "a debug_assert!, prose, a string literal and a gated inner module" \
      plant_permitted_shapes "$path" "$sym" "$expr"
    gate_selftest_passes "a rustfmt-wrapped debug_assert! around the use" \
      plant_wrapped_debug_assert "$path" "$sym" "$expr"
    gate_selftest_passes "cfg(all(…)) with debug_assertions as the SECOND operand" \
      plant_all_cfg_swapped "$path" "$sym" "$expr"
    gate_selftest_passes "a nested block comment closing the file, whose stray bracket is comment and not code" \
      plant_nested_block_comment_closing_the_file "$path" "$sym" "$expr"
    gate_selftest_passes 'a `;` inside a gated signature' \
      plant_semicolon_in_signature "$path" "$sym" "$expr"
    gate_selftest_passes 'a statement-position attribute over a multi-line call whose argument carries a brace' \
      plant_statement_attribute_over_a_braced_call "$path" "$sym" "$expr"
    gate_selftest_passes 'the same with nested braces in two of the call arguments' \
      plant_statement_attribute_with_nested_braces "$path" "$sym" "$expr"
  done
  gate_selftest_pin 1
  gate_selftest_pin -1
  printf '%s selftest OK, over %s subjects and the %s spellings they carry, each proved on its own: every spelling plants its own leak, so a spelling the reader cannot match fails here; enclosure by brace depth and by `debug_assert!` statement (rustfmt-wrapped or not); `all(…)` gates in either operand order while `any(…)` and `not(…)` do not; prose, string literals, `cfg(test)` code, a longer identifier that merely contains a spelling (prefixed, suffixed, or both at once) and the same name in another case are not uses — the near misses run per SPELLING, because the reader takes the anchor at each end from that end of the spelling; an item ends at a `;` only at bracket depth zero, and only a `{` at bracket depth zero is the item ENTERING — so a statement-position attribute over a multi-line call whose arguments carry braces (nested, in more than one argument) covers that call and stops at its `;`, with a use below it firing as the ungated use it is; a NESTED block comment is comment to its balancing `*/`, so the stray bracket in one is not code — pinned both ways, the leak after one firing as the ordinary leak it is and the same comment closing a file passing; each row carries the use count it pins, proved against this tree in both directions; and a lost bracket depth (planted as code, both with braced items below the stray bracket and with nothing below it), a missing subject and a `grep` that cannot run are each a loud failure\n' \
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
