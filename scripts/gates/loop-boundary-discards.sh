#!/usr/bin/env bash
# loop-boundary-discards.sh — every place a `LoopBoundary` value is
# thrown away is in the register below, at a pinned count, audited or
# not. ONE home; ci.yml's "LoopBoundary deferral register" step and
# local-scripts/ci-local.sh's discipline row both call this file.
#
# THE INVARIANT. A `continue` carrying a paragraph of justification and
# a `continue` carrying none are the same six characters. The paragraph
# is the bar this repo sets for a deferral — *it names the arm that asks
# the SAME question about the SAME pair* — and prose is not an
# instrument: nothing in CI could tell the two apart, so the same defect
# was found four times by four people reading the code. This gate makes
# the difference a fact CI holds: every discard is a register entry, and
# the entry says whether the arm has been named.
#
# WHY DERIVED AND NOT LISTED, `probe-suite-census.sh`'s shape. The
# population is read out of the tree on every run; the register is only
# the DISPOSITION of what was read. A list would be a snapshot, and a
# snapshot of "places that discard" goes stale in the silent direction —
# by a discard arriving.
#
# WHAT COUNTS AS A DISCARD, and both halves are mechanical rather than a
# judgement embedded in a matcher:
#
#   * THE LET-ELSE FORM. `let LoopBoundary::Cycle { first } = … else {
#     continue };`, and the same wrapped in one layer of pattern (`let
#     Some(LoopBoundary::Cycle { first }) = … else { … }`). The else
#     branch is reached by the OTHER variant, which it never names.
#     Every let-else over this enum is therefore a discard site; there
#     is nothing to decide.
#   * THE MATCH-ARM FORM. `LoopBoundary::Empty { .. } => {}`, `=>
#     continue`, `=> return …`. The tell is a pattern that BINDS
#     NOTHING — `{ .. }`, `{ first: _ }`, `{ vertex: _lone }` (an
#     underscore-prefixed binding is the lint's own spelling for
#     discarded) — so whatever the arm does, it does not look at the
#     value. An arm that binds cannot be a discard: an unused binding is
#     an `unused_variables` warning and this workspace builds with
#     `-D warnings`, so the binding is used or the tree does not
#     compile. Or-patterns of such arms count too.
#
# The branch must CONTINUE THE COMPUTATION — `continue`, `break`,
# `return` — because that is what a deferral does. An arm that aborts
# (`panic!`, `unreachable!`, `todo!`) cannot produce a wrong answer,
# only a loud stop, so it is not in the class and is not counted.
#
# THE REGISTER'S KEY IS `<file>|<fn>|<fragment>`, never a line number:
# line numbers rot under every edit above them. `<fn>` is the nearest
# ENCLOSING `fn` — the innermost one whose brace is still open at the
# site, so a helper `fn` declared and closed earlier in the same body
# cannot claim it. `<fragment>` is a substring of the site's own text
# (the pattern through the first word of the branch, which is all the
# matcher keeps), and is needed only where one `fn` holds discards whose
# dispositions differ; an empty fragment matches every discard in that
# `fn`, and a site matched by several entries goes to the one with the
# longest fragment.
#
# `<count>` IS WHY THIS IS A REGISTER AND NOT AN ALLOWLIST. Without it
# an entry absorbs every later discard in its `fn`: a third one arrives,
# matches an existing key, and passes — silently taking that entry's
# disposition, which for an `audited` entry means a brand-new deferral
# reported as audited. So each entry pins how many sites it stands for
# and the gate reds when the number differs IN EITHER DIRECTION. A
# fragment that matches nothing is caught by the same check, which is
# what validates a fragment at all.
#
# THE TWO DISPOSITIONS. `audited: <arm>` means someone has named the arm
# that asks the same question about the same pair. `unaudited` means the
# discard is recorded and the question is open; it is a debt, not a
# pass. THE GATE READS NEITHER — it verifies that the entry exists and
# that its count is right, not that the sentence is true; a reviewer
# does that. What the counts buy is that a NEW discard reds until it is
# registered, which is the property a bare tally of unaudited sites
# could not give.
#
# THE REDS: an entry that is not well formed; a live discard no entry
# matches; an entry whose matched-site count is not the one it pins
# (zero included — an entry whose site is gone claims a disposition for
# code that does not exist).
#
# WHAT THE MATCHER CANNOT SEE, measured rather than asserted, and with
# the direction of each error:
#
#   * A DISCARD SPLIT OVER MORE CODE LINES THAN THE WINDOW — an
#     UNDER-count. The reader is fed `--window $WINDOW`. Measured on
#     this tree, the same matcher returns 60 sites at window 4, 78 at 6,
#     79 at 8 and 80 from window 10 out to 32. The flat tail is the
#     evidence that 16 clears the tree with margin; re-running the sweep
#     is how a taker checks it rather than trusting this paragraph.
#   * `if let …` AND `while let …` — an UNDER-count, and the largest
#     one: `if let LoopBoundary::Cycle { first } = … { … }` skips the
#     `Empty` case with no else branch to read at all. Three live sites
#     (`review_m0_pr7.rs`, `iso.rs`, `review_m1_pr4.rs`), all test
#     helpers today. Matching them needs the arm the source does not
#     write, which is a different instrument.
#   * A NO-BINDING ARM THAT PRODUCES A VALUE rather than transferring
#     control — `=> None`, `=> Err(…)`, `=> true`. Six live no-binding
#     arms are neither empty nor a control transfer: five produce a
#     value, one panics (and a panic is out of scope above). An
#     UNDER-count, and deliberate: the class is a deferral, and an arm
#     whose value flows on is a decision its caller consumes.
#   * A `;` INSIDE THE SCRUTINEE (`let … = [x; 3] else`), a TUPLE
#     let-else, and a match arm that does not start its own source line
#     — all UNDER-counts; none occurs.
#   * A DISCARD INSIDE A CLOSURE is keyed on the enclosing `fn`, because
#     a closure is not an item and the reader has no name for it. That
#     is what `<fragment>` is for.
#   * `macro_rules!` BODIES AND `include!`d TEXT, which `lib.sh`'s
#     reader does not expand. `#[cfg(test)]` items are skipped, so a
#     discard reachable only from a test module is not counted.
#   * `Self::(Cycle|Empty) { … }` IS MATCHED, for an `impl LoopBoundary`
#     that writes its own variants that way. The cost is the one
#     OVER-count here: another enum with a struct variant named `Cycle`
#     or `Empty`, matched through `Self::` and discarded, would be
#     counted. None exists; it would arrive as an unregistered site, not
#     as a silent pass.
#
# OUT OF SCOPE BY DEFINITION, so absent rather than missed: a `continue`
# taken for a different reason (an arena miss), the same class over any
# other enum, and the aborting branches named above.
#
# `--register FILE` replaces the register below with one read from a
# file. It exists so the self-test can plant a malformed entry — a red
# no fixture could otherwise reach, because the register it guards is
# baked into this file.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

# Code lines joined into one record. See the measurement in the header.
WINDOW=16

# THE REGISTER — `<file>|<fn>|<fragment>|<count>|<disposition>`.
REGISTER=(
  "crates/editor-core/src/names/emit.rs|face_half_edges||1|unaudited"
  "crates/mesh/src/trimmed.rs|trim_polygon||1|unaudited"
  "crates/mesh/src/walk.rs|loop_edges||1|unaudited"
  "crates/step-export/src/volume.rs|shell_signed_volume||1|unaudited"
  "crates/step-export/src/writer.rs|face_bound||1|unaudited"
  "crates/step-import/src/adopt.rs|rotate_loop_firsts||1|unaudited"
  "crates/sweep/src/blend/battery.rs|consumption_sweep||1|unaudited"
  "crates/sweep/src/blend/build.rs|face_cycle||1|unaudited"
  "crates/sweep/src/blend/surgery.rs|loop_walk||1|unaudited"
  "crates/sweep/src/swept.rs|describe_face_rim_at_rest||1|unaudited"
  "crates/topo/src/boolean/contain.rs|iso_bounded_wall||1|unaudited"
  "crates/topo/src/boolean/contain.rs|loop_cycle_points||1|unaudited"
  "crates/topo/src/boolean/finish.rs|classify_shell||1|unaudited"
  "crates/topo/src/boolean/join.rs|face_vertex_points||1|unaudited"
  "crates/topo/src/boolean/join.rs|resolve_roles_geometric||2|unaudited"
  "crates/topo/src/boolean/ops.rs|classify_shells||1|unaudited"
  "crates/topo/src/boolean/ops.rs|describe_minted_edges||1|unaudited"
  "crates/topo/src/boolean/ops.rs|sphere_extent_scan||1|unaudited"
  "crates/topo/src/boolean/rest.rs|bfs_order||1|unaudited"
  "crates/topo/src/boolean/rest.rs|cycle_starts||1|unaudited"
  "crates/topo/src/boolean/rest.rs|halves_at||1|unaudited"
  "crates/topo/src/boolean/rest.rs|patch_faces||1|unaudited"
  "crates/topo/src/boolean/rest.rs|shared_run||1|unaudited"
  "crates/topo/src/boolean/rest.rs|slit_zip||1|unaudited"
  "crates/topo/src/boolean/rest.rs|zip_folded||1|unaudited"
  "crates/topo/src/boolean/rim_wedge.rs|face_boundary_circles||1|unaudited"
  "crates/topo/src/boolean/solid_contain.rs|cone_slant_window||1|unaudited"
  "crates/topo/src/boolean/solid_contain.rs|cylinder_chart_trim||1|unaudited"
  "crates/topo/src/boolean/solid_contain.rs|sphere_chart_trim||1|unaudited"
  "crates/topo/src/boolean/solid_contain.rs|torus_chart_windows||1|unaudited"
  "crates/topo/src/boolean/surface_group.rs|surface_group||1|unaudited"
  "crates/topo/src/boolean/vtxfac.rs|classify_vertex_on_face||1|unaudited"
  "crates/topo/src/boolean/zip.rs|zip_seam||1|unaudited"
  "crates/topo/src/census.rs|snapshot||1|unaudited"
  # The hull closure decides nothing about emptiness: its two callers
  # want opposite things from it and each answers at its own call site.
  "crates/topo/src/census.rs|sweep_cross_solid_backstop|else { continue|1|audited: the arm above face_points — a loop this cannot walk contributes nothing, and both callers answer emptiness themselves"
  # The planar x planar skip's premise: only a face whose whole boundary
  # is admitted is in front of the exact sweeps.
  "crates/topo/src/census.rs|sweep_cross_solid_backstop|else { return|1|audited: the arm above line_bounded — anything unresolvable is not a line, so the face stays with the containment arm"
  "crates/topo/src/chart_region.rs|face_boundary_points||1|unaudited"
  "crates/topo/src/chart_region.rs|loop_uv_polygon||1|unaudited"
  "crates/topo/src/chord_join.rs|face_azimuth_window||1|unaudited"
  "crates/topo/src/coherence.rs|traversals||1|unaudited"
  "crates/topo/src/euler.rs|find_half_edge||1|unaudited"
  "crates/topo/src/euler.rs|mef_chord||1|unaudited"
  "crates/topo/src/euler.rs|mef_lone||1|unaudited"
  "crates/topo/src/euler.rs|mev_line||1|unaudited"
  "crates/topo/src/euler.rs|mev_lone_plan||1|unaudited"
  "crates/topo/src/euler_kill.rs|kvfs||1|unaudited"
  "crates/topo/src/euler_ring.rs|mekr_both_empty||2|unaudited"
  "crates/topo/src/euler_ring.rs|mekr_empty_ring||1|unaudited"
  "crates/topo/src/euler_ring.rs|mekr_empty_target||1|unaudited"
  "crates/topo/src/merge_faces.rs|loop_winding||1|unaudited"
  "crates/topo/src/movefac.rs|movefac||1|unaudited"
  "crates/topo/src/offset_axial.rs|nappe_signed||1|unaudited"
  "crates/topo/src/pcurves.rs|clear_face_caches||1|unaudited"
  "crates/topo/src/pcurves.rs|validate_pcurves||2|unaudited"
  "crates/topo/src/pcurves.rs|walk_loop||1|unaudited"
  "crates/topo/src/props.rs|loop_edges||1|unaudited"
  "crates/topo/src/replace_face.rs|boundary_edges_into||1|unaudited"
  "crates/topo/src/review_m1_pr4.rs|some_single_op_reaches||1|unaudited"
  "crates/topo/src/seqgen.rs|first_empty_ring_site||2|unaudited"
  "crates/topo/src/seqgen.rs|mef_chords_candidates||1|unaudited"
  "crates/topo/src/shell.rs|duplicate_in_loop||1|unaudited"
  "crates/topo/src/shell.rs|face_boundary_points||1|unaudited"
  "crates/topo/src/shell.rs|face_neighbours||1|unaudited"
  "crates/topo/src/shell.rs|loop_points||1|unaudited"
  "crates/topo/src/shell.rs|rename_loop_surface||1|unaudited"
  "crates/topo/src/shell.rs|ring_rows||1|unaudited"
  "crates/topo/src/shell.rs|split_cycle||1|unaudited"
  "crates/topo/src/splitting/containment.rs|loop_points||1|unaudited"
  "crates/topo/src/splitting/finish.rs|classify_shell||1|unaudited"
  "crates/topo/src/splitting/finish.rs|describe_section_boundary||1|unaudited"
  "crates/topo/src/splitting/join.rs|certify_section_area||1|unaudited"
  "crates/topo/src/splitting/join.rs|loop_starts||1|unaudited"
  "crates/topo/src/validate.rs|loop_cycle_of||1|unaudited"
  "crates/topo/src/validate.rs|tier1||1|unaudited"
  "crates/topo/src/validate.rs|tier3_local_checks_marked||2|unaudited"
)

# The matchers, in one place. Anchored at the start of a record, because
# a discard is a construct that BEGINS at its site: a pattern free to
# match mid-record would read a `LoopBoundary::Cycle {` several lines
# into a window as a site of its own. `ANCHOR_RE` is that same head
# matched over the LINE view, where it marks the lines a site can start
# at and the enclosing `fn` open at each of them.
#
# NO BACKSLASH APPEARS IN THEM, and that is not a style choice. A
# metacharacter is written as a one-member bracket expression (`[{]`,
# `[(]`, `[|]`, `[.]`) because a backslashed one has to survive TWO
# escape passes to reach the matcher intact — bash's, and awk's own
# processing of a `-v` assignment — and it did not: `-v RE='…\(…'`
# handed awk `(` and the ERE came out unbalanced. mawk shrugged and
# matched; the hosted runner's awk warned on every `\(` and then died
# on the unmatched one, so the gate's own clean fixture failed there
# and passed here. `bit-identity-debug-only.sh`'s header names the same
# hazard. The regexes reach awk through ENVIRON below, which does no
# escape processing at all, and a bracket expression is then correct
# under either route and under every awk.
PATH_PREFIX='([A-Za-z_][A-Za-z0-9_]*::)*'
ENUM="(${PATH_PREFIX}LoopBoundary|Self)::(Cycle|Empty)"
NO_BINDING='[{] *([.][.]|[A-Za-z_][A-Za-z0-9_]* *: *_[A-Za-z0-9_]*) *[}]'
DEFER='(continue|break|return)([^A-Za-z0-9_]|$)'
LET_RE="^let (${ENUM} [{][^;{}]*[}]|[A-Za-z_][A-Za-z0-9_:]*[(]${ENUM} [{][^;{}]*[}][)]) = [^;]*else *[{] *${DEFER}"
ARM_RE="^${ENUM} ${NO_BINDING}( *[|] *${ENUM} ${NO_BINDING})*( if [^;{}]*)? => ([{] *[}]|[{] *${DEFER}|${DEFER})"
ANCHOR_RE="^(let )?([A-Za-z_][A-Za-z0-9_:]*[(])?${ENUM} [{]"

# Set by --register; empty means the array above.
GATE_REGISTER_FILE=

gate() {
  gate_require_crate_sources
  local lineview winview report entries
  if [ -n "$GATE_REGISTER_FILE" ]; then
    if ! entries=$(cat "$GATE_REGISTER_FILE" 2>/dev/null); then
      gate_error "$(gate_name): cannot read the register at $GATE_REGISTER_FILE, so there is nothing to check the tree against"
      exit 1
    fi
  else
    entries=$(printf '%s\n' "${REGISTER[@]}")
  fi
  # THE READER'S STATUS IS CHECKED, not inherited. `lib.sh` says a
  # matcher that did not run is not a clean scan; this gate reads
  # through awk rather than grep, so `gate_grep`'s marker cannot speak
  # for it and the check is written here instead.
  if ! lineview=$(gate_rust_code --skip-cfg-test "${GATE_SOURCE_FILES[@]}"); then
    gate_error "$(gate_name): the shared Rust reader could not build the code-only line view, so what it did not match is unknown — that is not a pass"
    exit 1
  fi
  if ! winview=$(gate_rust_code --skip-cfg-test --window "$WINDOW" "${GATE_SOURCE_FILES[@]}"); then
    gate_error "$(gate_name): the shared Rust reader could not build the code-only window view, so what it did not match is unknown — that is not a pass"
    exit 1
  fi
  if [ -z "$lineview" ] || [ -z "$winview" ]; then
    gate_error "$(gate_name): the shared Rust reader returned NOTHING over $GATE_SCAN_FILES source file(s) — the scan decided nothing, which is not a pass"
    exit 1
  fi
  export LBD_ANCHOR="$ANCHOR_RE" LBD_LET="$LET_RE" LBD_ARM="$ARM_RE"
  if ! report=$(printf '%s\n' "$entries" "===" "$lineview" "===" "$winview" \
    | awk '
      BEGIN {
        ANCHOR = ENVIRON["LBD_ANCHOR"]
        LETRE = ENVIRON["LBD_LET"]
        ARMRE = ENVIRON["LBD_ARM"]
      }
      /^===$/ { phase++; next }
      phase == 0 {
        ne++
        n = split($0, f, "\\|")
        if (n != 5 || f[1] == "" || f[2] == "" || f[4] !~ /^[1-9][0-9]*$/ ||
            (f[5] != "unaudited" && f[5] !~ /^audited: ./)) {
          print "MALFORMED|" $0
        }
        ef[ne] = f[1]; ei[ne] = f[2]; eg[ne] = f[3]; ec[ne] = f[4] + 0; ed[ne] = f[5]
        next
      }
      {
        i = index($0, ":")
        file = substr($0, 1, i - 1); rest = substr($0, i + 1)
        j = index(rest, ":")
        line = substr(rest, 1, j - 1) + 0
        txt = substr(rest, j + 1)
      }
      # THE ENCLOSING fn, not the nearest preceding one. A `fn` declared
      # and closed inside another body — a local helper, a method in a
      # local `impl` — is not the enclosing item of anything after its
      # closing brace, and keying a discard on it names the wrong owner.
      # So the braces are counted and the innermost `fn` still open at
      # the site is the answer.
      phase == 1 {
        if (file != curfile) { curfile = file; depth = 0; pending = ""; nfn = 0 }
        bare = txt; sub(/^[ \t]+/, "", bare)
        if (bare ~ ANCHOR) at[file ":" line] = (nfn > 0) ? fname[nfn] : "(no enclosing fn)"
        if (match(txt, /(^|[^A-Za-z0-9_])fn [A-Za-z_][A-Za-z0-9_]*/)) {
          s = substr(txt, RSTART, RLENGTH); sub(/^[^f]*fn /, "", s); pending = s
        }
        n = length(txt)
        for (k = 1; k <= n; k++) {
          c = substr(txt, k, 1)
          if (c == "{") {
            depth++
            if (pending != "") { nfn++; fname[nfn] = pending; fdepth[nfn] = depth; pending = "" }
          } else if (c == "}") {
            if (nfn > 0 && fdepth[nfn] == depth) nfn--
            depth--
          } else if (c == ";") { pending = "" }
        }
        next
      }
      {
        sub(/^ /, "", txt)
        if (!(txt ~ LETRE || txt ~ ARMRE)) next
        # The enclosing `fn`, read at the line the window starts on. A
        # window starts on a CODE line and its first component is the
        # text of that line, so a record matching above has a site
        # there and `at[]` names its item; an empty one matches no
        # entry and is reported, never dropped. (No apostrophe here:
        # a single quote cannot appear anywhere in this program, which
        # is itself single-quoted.)
        item = at[file ":" line]
        head = txt
        if (txt ~ LETRE) {
          if (match(txt, /else *[{] *[A-Za-z_]*/)) head = substr(txt, 1, RSTART + RLENGTH - 1)
        } else {
          if (match(txt, /=> *[{]? *[A-Za-z_]*/)) head = substr(txt, 1, RSTART + RLENGTH - 1)
        }
        total++
        hit = 0; best = -1
        for (k = 1; k <= ne; k++) {
          if (ef[k] != file || ei[k] != item) continue
          if (eg[k] != "" && index(head, eg[k]) == 0) continue
          if (length(eg[k]) > best) { best = length(eg[k]); hit = k }
        }
        if (hit == 0) print "UNREG|" file "|" line "|" item "|" head
        else {
          seen[hit]++
          if (ed[hit] ~ /^audited/) aud++; else un++
        }
      }
      END {
        for (k = 1; k <= ne; k++) {
          have = (k in seen) ? seen[k] : 0
          if (have != ec[k]) print "MISCOUNT|" ef[k] "|" ei[k] "|" eg[k] "|pinned " ec[k] "|matched " have
          if (ed[k] ~ /^audited/) eaud++; else eun++
        }
        print "COUNT|" total + 0 "|" ne + 0 "|" eaud + 0 "|" eun + 0
      }
    '); then
    gate_error "$(gate_name): the register comparison could not run, so nothing about the tree was decided — that is not a pass"
    exit 1
  fi
  local bad
  bad=$(printf '%s\n' "$report" | sed -n '/^COUNT|/!p')
  if [ -n "$bad" ]; then
    printf '%s\n' "$bad"
    gate_error "$(gate_name): the deferral register does not match the tree — a MALFORMED line is an entry that is not <file>|<fn>|<fragment>|<count>|(unaudited|audited: …) with a positive count; an UNREG line is a LoopBoundary discard no entry names, so add it to REGISTER as \`unaudited\` (or as \`audited: <the arm that asks the same question about the same pair>\` if you are auditing it now); a MISCOUNT line is an entry standing for a different number of discards than it pins, and both directions matter — 'matched 0' is an entry whose site is gone, and a rise is a NEW discard that would otherwise have inherited an existing entry's disposition"
    exit 1
  fi
  local counts total nent aud un
  counts=$(printf '%s\n' "$report" | sed -n 's/^COUNT|//p')
  total=${counts%%|*}; counts=${counts#*|}
  nent=${counts%%|*}; counts=${counts#*|}
  aud=${counts%%|*}; un=${counts#*|}
  gate_ok "$total LoopBoundary discard(s) the matcher sees, each registered at its pinned count by one of $nent entries ($aud marked audited, $un unaudited)"
}

# --- THE FIXTURE ------------------------------------------------------
#
# THE CLEAN TREE IS WRITTEN FROM THE REGISTER ITSELF, which is the only
# way a gate with these reds can have a passing fixture at all: every
# entry must find exactly the number of discards it pins, so the fixture
# plants that many. It is also a check on the register's own shape — an
# entry whose `<fn>` or `<fragment>` the reader cannot reproduce fails
# here rather than on a real tree.
#
# THE PLANTED SPELLING CYCLES through every form the two matchers
# accept, so the clean fixture is where each one is shown to be READ,
# and the firing planters below show them FIRE. Every planted site sits
# under a COMMENT-ONLY LINE, and that is this fixture's second job: a
# line carrying no code is not a record, and a reader that emitted one
# would start a window at the comment and count every site here twice —
# every entry MISCOUNTing at pinned+1. The pinned counts are what make
# that a check rather than a hope.
gate_plant_site() {
  local file=$1 item=$2 form=$3
  {
    printf 'fn %s() {\n' "$item"
    printf '    for lk in loops {\n'
    printf '        // the comment-only line whose record repeats the window\n'
    case "$form" in
      wrapped-continue)
        printf '        let LoopBoundary::Cycle { first } = body\n'
        printf '            .get_loop(lk)\n'
        printf '            .ok_or_else(corrupt)?\n'
        printf '            .boundary\n'
        printf '        else {\n'
        printf '            continue;\n'
        printf '        };\n'
        printf '        use_it(first);\n' ;;
      wrapped-return)
        printf '        let LoopBoundary::Cycle { first } = body\n'
        printf '            .get_loop(lk)\n'
        printf '            .ok_or_else(corrupt)?\n'
        printf '            .boundary\n'
        printf '        else {\n'
        printf '            return false;\n'
        printf '        };\n'
        printf '        use_it(first);\n' ;;
      oneline)
        printf '        let LoopBoundary::Cycle { first } = lp.boundary else { continue; };\n'
        printf '        use_it(first);\n' ;;
      nested)
        printf '        let Some(LoopBoundary::Cycle { first }) = body.get_loop(lk).map(|l| l.boundary) else { return Vec::new(); };\n'
        printf '        use_it(first);\n' ;;
      no-space)
        printf '        let crate::entity::LoopBoundary::Empty { vertex } = lp.boundary else{break;};\n'
        printf '        use_it(vertex);\n' ;;
      arm-dotdot)
        printf '        match lp.boundary {\n'
        printf '        LoopBoundary::Empty { .. } => continue,\n'
        printf '        LoopBoundary::Cycle { first } => use_it(first),\n'
        printf '        }\n' ;;
      arm-underscore)
        printf '        match lp.boundary {\n'
        printf '        LoopBoundary::Cycle { first: _ } => return None,\n'
        printf '        LoopBoundary::Empty { vertex } => use_it(vertex),\n'
        printf '        }\n' ;;
      arm-named-underscore)
        printf '        match lp.boundary {\n'
        printf '        topo::LoopBoundary::Empty { vertex: _lone } => {}\n'
        printf '        topo::LoopBoundary::Cycle { first } => use_it(first),\n'
        printf '        }\n' ;;
      arm-or-pattern)
        printf '        match lp.boundary {\n'
        printf '        LoopBoundary::Empty { .. } | LoopBoundary::Cycle { .. } => continue,\n'
        printf '        }\n' ;;
      arm-self)
        printf '        match self {\n'
        printf '        Self::Empty { .. } => break,\n'
        printf '        Self::Cycle { first } => use_it(first),\n'
        printf '        }\n' ;;
    esac
    printf '    }\n'
    printf '}\n'
  } >> "$file"
}

# The forms the cycle draws from. `wrapped-continue` and
# `wrapped-return` are also what a fragment naming `continue` or
# `return` selects, so an entry whose fragment names neither cannot be
# planted and fails the self-test — which is the point: a fragment the
# fixture cannot reproduce is one the tree may not reproduce either.
GATE_PLANT_FORMS=(wrapped-continue oneline nested no-space arm-dotdot
  arm-underscore arm-named-underscore arm-or-pattern arm-self wrapped-return)

gate_plant_clean() {
  local t=$1 e file item frag count disp i form n=0
  for e in "${REGISTER[@]}"; do
    IFS='|' read -r file item frag count disp <<<"$e"
    mkdir -p "$t/$(dirname "$file")"
    for ((i = 0; i < count; i++)); do
      case "$frag" in
        "") form=${GATE_PLANT_FORMS[$((n % ${#GATE_PLANT_FORMS[@]}))]}; n=$((n + 1)) ;;
        *" continue") form=wrapped-continue ;;
        *" return") form=wrapped-return ;;
        *) form=unplantable ;;
      esac
      gate_plant_site "$t/$file" "$item" "$form"
    done
  done
}

plant_unregistered_let() {
  mkdir -p "$1/crates/planted/src"
  gate_plant_site "$1/crates/planted/src/lib.rs" arrived_unregistered oneline
}

plant_unregistered_arm() {
  mkdir -p "$1/crates/planted/src"
  gate_plant_site "$1/crates/planted/src/lib.rs" arrived_unregistered_arm arm-named-underscore
}

# A helper `fn` declared and closed inside the body that holds the
# discard: the nearest PRECEDING `fn` is the helper, the enclosing one
# is the outer body, and the diagnosis has to name the outer body.
plant_nested_helper_fn() {
  mkdir -p "$1/crates/planted/src"
  {
    printf 'fn outer_holder() {\n'
    printf '    fn inner_helper() -> bool {\n'
    printf '        true\n'
    printf '    }\n'
    printf '    for lk in loops {\n'
    printf '        let LoopBoundary::Cycle { first } = lp.boundary else { continue; };\n'
    printf '        use_it(first);\n'
    printf '    }\n'
    printf '}\n'
  } > "$1/crates/planted/src/lib.rs"
}

# One registered file rewritten with its `fn`s and none of their
# discards: the entries survive, the sites do not, and every entry for
# that file matches 0 where it pins at least 1.
plant_registered_site_gone() {
  local t=$1 e file item target
  target=$(printf '%s' "${REGISTER[0]}" | cut -d'|' -f1)
  : > "$t/$target"
  for e in "${REGISTER[@]}"; do
    IFS='|' read -r file item _ _ _ <<<"$e"
    [ "$file" = "$target" ] || continue
    printf 'fn %s() {}\n' "$item" >> "$t/$target"
  done
}

# One more discard under a `fn` an entry already stands for. Without the
# pinned count this is the absorbed site: it matches an existing key and
# passes, taking that entry's disposition with it.
plant_extra_site() {
  local file=$1 item=$2 form=$3 t=$4
  gate_plant_site "$t/$file" "$item" "$form"
}

plant_other_enum_discard() {
  mkdir -p "$1/crates/planted/src"
  {
    printf 'fn other_enum() {\n'
    printf '    for lk in loops {\n'
    printf '        let CurveGeom::Certified { first } = c.geom else { continue; };\n'
    printf '        use_it(first);\n'
    printf '    }\n'
    printf '    match lp.kind {\n'
    printf '        EdgeKind::Empty { .. } => continue,\n'
    printf '        EdgeKind::Cycle { first } => use_it(first),\n'
    printf '    }\n'
    printf '}\n'
  } > "$1/crates/planted/src/lib.rs"
}

plant_bound_and_used() {
  mkdir -p "$1/crates/planted/src"
  {
    printf 'fn bound_and_used() {\n'
    printf '    match loop_data.boundary {\n'
    printf '        LoopBoundary::Cycle { first } => first,\n'
    printf '        LoopBoundary::Empty { vertex: lone } if loop_key == outer => {\n'
    printf '            return Err(corrupt(lone));\n'
    printf '        }\n'
    printf '        LoopBoundary::Empty { vertex: lone } => {\n'
    printf '            extent = extent.max(dist(lone));\n'
    printf '            continue;\n'
    printf '        }\n'
    printf '    }\n'
    printf '}\n'
  } > "$1/crates/planted/src/lib.rs"
}

plant_prose_and_predicate() {
  mkdir -p "$1/crates/planted/src"
  {
    printf '//! Never write `let LoopBoundary::Cycle { first } = b else { continue };`\n'
    printf '/* nor LoopBoundary::Empty { .. } => continue, in a block comment */\n'
    printf 'const WHY: &str = "LoopBoundary::Empty { .. } => {}";\n'
    printf 'fn predicate() -> bool {\n'
    printf '    matches!(loop_data.boundary, LoopBoundary::Empty { .. })\n'
    printf '} // nor LoopBoundary::Cycle { .. } => return, in a trailing one\n'
  } > "$1/crates/planted/src/lib.rs"
}

# The MALFORMED red, which no fixture tree can reach: the register it
# guards is baked into this file, so the case hands the gate a written
# one instead.
gate_selftest_malformed_register() {
  local tmp out reg
  tmp=$(mktemp -d)
  gate_plant_clean "$tmp"
  reg=$tmp/planted-register.txt
  printf '%s\n' "${REGISTER[@]}" | sed '1s/|1|unaudited$/|1|maybe-audited/' > "$reg"
  if out=$("$0" --root "$tmp" --register "$reg" 2>&1); then
    rm -rf "$tmp"
    printf 'SELFTEST FAILED: the gate PASSED on a register entry whose disposition is neither unaudited nor an audit\n%s\n' "$out" >&2
    exit 1
  fi
  rm -rf "$tmp"
  gate_selftest_assert_diagnosed "a malformed register entry" "$out"
  case "$out" in
    *MALFORMED*) ;;
    *) printf 'SELFTEST FAILED (a malformed register entry): the gate fired for some OTHER reason:\n%s\n' "$out" >&2
       exit 1 ;;
  esac
}

gate_selftest() {
  local want="the deferral register does not match the tree"
  gate_selftest_clean
  # The reader is awk, so an awk that cannot run is this gate's silent
  # failure: it produces no records, and no records is what a tree with
  # no discards produces.
  gate_selftest_without_tool awk "could not build the code-only"
  gate_selftest_case "$want" plant_unregistered_let
  gate_selftest_case "$want" plant_unregistered_arm
  gate_selftest_case "|outer_holder|let LoopBoundary::Cycle" plant_nested_helper_fn
  gate_selftest_case "$want" plant_registered_site_gone
  # A second discard under a one-site entry, and a second under an entry
  # a FRAGMENT narrows — the two shapes a key without a count absorbs.
  gate_selftest_case "matched 2" plant_extra_site \
    crates/editor-core/src/names/emit.rs face_half_edges oneline
  gate_selftest_case "matched 2" plant_extra_site \
    crates/topo/src/census.rs sweep_cross_solid_backstop wrapped-continue
  gate_selftest_malformed_register
  gate_selftest_passes "a let-else and a match arm discarding some OTHER enum" plant_other_enum_discard
  gate_selftest_passes "a LoopBoundary value BOUND and used, not discarded" plant_bound_and_used
  gate_selftest_passes "the spelling in prose, in a doc comment, in a string literal and inside matches!" plant_prose_and_predicate
  printf '%s selftest OK: passes a tree whose every discard is registered at its pinned count, across all ten spellings the matchers accept and with every site under a comment-only line, which the shared reader must not make a record of; fires on an unregistered let-else, an unregistered match arm, a register entry whose site is gone, a malformed entry, and a second discard absorbed by a plain key or by a fragment; names the ENCLOSING fn rather than a closed helper; stays quiet on another enum, on a bound-and-used value and on prose; and stays RED, with a diagnosis, when the reader itself cannot run\n' "$(gate_name)"
}

# `--register` is this gate's own flag, so it is taken out of argv
# before lib.sh's parser — which rejects what it does not know — sees
# the rest.
GATE_ARGV=()
while [ $# -gt 0 ]; do
  case "$1" in
    --register) GATE_REGISTER_FILE=${2:-}; shift 2 ;;
    *) GATE_ARGV+=("$1"); shift ;;
  esac
done
gate_parse_args ${GATE_ARGV[@]+"${GATE_ARGV[@]}"}
gate_main
