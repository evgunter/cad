#!/usr/bin/env bash
# loop-boundary-discards.sh — every place a `LoopBoundary` value is
# thrown away is in the register below, audited or not. ONE home;
# ci.yml's "LoopBoundary deferral register" step and
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
#     continue };` — the else branch is reached by the OTHER variant,
#     which it never names. Every let-else over this enum is therefore a
#     discard site; there is nothing to decide.
#   * THE MATCH-ARM FORM. `LoopBoundary::Empty { .. } => {}`, `=>
#     continue`, `=> return …`. The tell is `{ .. }` (or `{ _ }`): the
#     pattern binds nothing, so whatever the arm does, it does not look
#     at the value. An arm that binds cannot be a discard — an unused
#     binding is an `unused_variables` warning and this workspace builds
#     with `-D warnings`, so the binding is used or the tree does not
#     compile.
#
# The branch must CONTINUE THE COMPUTATION — `continue`, `break`,
# `return` — because that is what a deferral does. An arm that aborts
# (`panic!`, `unreachable!`, `todo!`) cannot produce a wrong answer,
# only a loud stop, so it is not in the class and is not counted.
#
# THE REGISTER'S KEY IS `<file>|<fn>|<fragment>`, never a line number:
# line numbers rot under every edit above them. `<fn>` is the nearest
# enclosing `fn` name, read out of the same code-only view; `<fragment>`
# is a substring of the site's own text, and is needed only where one
# `fn` holds discards whose dispositions differ. An empty fragment
# matches every discard in that `fn`.
#
# THE TWO DISPOSITIONS. `audited: <arm>` means someone has named the arm
# that asks the same question about the same pair — the bar #620, #637
# and #737 set. `unaudited` means the discard is recorded and the
# question is open; it is a debt, not a pass. The OK line prints both
# counts so the unaudited number can be worked down and a rise in it is
# visible. Auditing a site is its owner's work, not this gate's.
#
# THE TWO REDS, and the second is why this is a register rather than an
# allowlist: (a) a live discard no entry matches — a discard arrived
# unregistered; (b) an entry no live discard matches — the register is
# claiming a disposition for code that is gone, and a stale register is
# as dishonest as a missing one.
#
# WHAT THE MATCHER CANNOT SEE, measured rather than asserted:
#
#   * A DISCARD SPLIT OVER MORE CODE LINES THAN THE WINDOW. The reader
#     is fed `--window $WINDOW`, so a `let` whose `else` is further than
#     that many code lines away is invisible. Measured on this tree: the
#     count is 60 at window 4, 78 at 6, 79 at 8 and 80 from window 10
#     all the way out to 24. The flat tail is the evidence that 16
#     clears the tree with margin, and re-running that sweep is how a
#     taker checks it rather than trusting this paragraph.
#   * A MATCH ARM THAT DOES NOT START ITS OWN SOURCE LINE, and a pattern
#     written without the spaces `rustfmt` puts in (`LoopBoundary::Empty{..}`).
#     Both matchers anchor at the start of a record.
#   * A DISCARD INSIDE A CLOSURE is keyed on the `fn` that holds the
#     closure, because a closure is not an item and the reader has no
#     name for it. That is what `<fragment>` is for.
#   * `macro_rules!` BODIES AND `include!`d TEXT, which `lib.sh`'s reader
#     does not expand.
#
# OUT OF SCOPE BY DEFINITION, so absent rather than missed: a `continue`
# taken for a different reason (an arena miss), the same class over any
# other enum, and the aborting branches named above.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

# Code lines joined into one record. See the measurement in the header.
WINDOW=16

# THE REGISTER — `<file>|<fn>|<fragment>|<disposition>`.
REGISTER=(
  "crates/editor-core/src/names/emit.rs|face_half_edges||unaudited"
  "crates/mesh/src/trimmed.rs|trim_polygon||unaudited"
  "crates/mesh/src/walk.rs|loop_edges||unaudited"
  "crates/step-export/src/volume.rs|shell_signed_volume||unaudited"
  "crates/step-export/src/writer.rs|face_bound||unaudited"
  "crates/step-import/src/adopt.rs|rotate_loop_firsts||unaudited"
  "crates/sweep/src/blend/build.rs|face_cycle||unaudited"
  "crates/sweep/src/blend/surgery.rs|loop_walk||unaudited"
  "crates/sweep/src/swept.rs|describe_face_rim_at_rest||unaudited"
  "crates/topo/src/boolean/contain.rs|a_ring_vertex_is_never_shadowed_by_an_outer_edge||unaudited"
  "crates/topo/src/boolean/contain.rs|iso_bounded_wall||unaudited"
  "crates/topo/src/boolean/contain.rs|loop_cycle_points||unaudited"
  "crates/topo/src/boolean/finish.rs|classify_shell||unaudited"
  "crates/topo/src/boolean/join.rs|face_vertex_points||unaudited"
  "crates/topo/src/boolean/join.rs|needs_interior_certificate||unaudited"
  "crates/topo/src/boolean/ops.rs|classify_shells||unaudited"
  "crates/topo/src/boolean/ops.rs|describe_minted_edges||unaudited"
  "crates/topo/src/boolean/ops.rs|sphere_extent_scan||unaudited"
  "crates/topo/src/boolean/rest.rs|bfs_order||unaudited"
  "crates/topo/src/boolean/rest.rs|cycle_starts||unaudited"
  "crates/topo/src/boolean/rest.rs|halves_at||unaudited"
  "crates/topo/src/boolean/rest.rs|patch_faces||unaudited"
  "crates/topo/src/boolean/rest.rs|shared_run||unaudited"
  "crates/topo/src/boolean/rest.rs|slit_zip||unaudited"
  "crates/topo/src/boolean/rest.rs|zip_folded||unaudited"
  "crates/topo/src/boolean/rim_wedge.rs|face_boundary_circles||unaudited"
  "crates/topo/src/boolean/solid_contain.rs|cone_slant_window||unaudited"
  "crates/topo/src/boolean/solid_contain.rs|cylinder_chart_trim||unaudited"
  "crates/topo/src/boolean/solid_contain.rs|sphere_chart_trim||unaudited"
  "crates/topo/src/boolean/solid_contain.rs|torus_chart_windows||unaudited"
  "crates/topo/src/boolean/vtxfac.rs|classify_vertex_on_face||unaudited"
  "crates/topo/src/boolean/zip.rs|zip_seam||unaudited"
  # The hull closure decides nothing about emptiness: its two callers
  # want opposite things from it and each answers at its own call site.
  "crates/topo/src/census.rs|sweep_cross_solid_backstop|else { continue|audited: the arm above face_points — a loop this cannot walk contributes nothing, and both callers answer emptiness themselves"
  # The planar x planar skip's premise: only a face whose whole boundary
  # is admitted is in front of the exact sweeps.
  "crates/topo/src/census.rs|sweep_cross_solid_backstop|else { return|audited: the arm above line_bounded — anything unresolvable is not a line, so the face stays with the containment arm"
  "crates/topo/src/census.rs|snapshot||unaudited"
  "crates/topo/src/chart_region.rs|face_boundary_points||unaudited"
  "crates/topo/src/chart_region.rs|loop_uv_polygon||unaudited"
  "crates/topo/src/chord_join.rs|face_azimuth_window||unaudited"
  "crates/topo/src/coherence.rs|traversals||unaudited"
  "crates/topo/src/euler.rs|find_half_edge||unaudited"
  "crates/topo/src/euler.rs|mef_chord||unaudited"
  "crates/topo/src/euler.rs|mef_lone||unaudited"
  "crates/topo/src/euler.rs|mev_line||unaudited"
  "crates/topo/src/euler.rs|mev_lone_plan||unaudited"
  "crates/topo/src/euler_kill.rs|kvfs||unaudited"
  "crates/topo/src/euler_ring.rs|mekr_both_empty||unaudited"
  "crates/topo/src/euler_ring.rs|mekr_empty_ring||unaudited"
  "crates/topo/src/euler_ring.rs|mekr_empty_target||unaudited"
  "crates/topo/src/iso.rs|form_is_invariant_under_cycle_first_rotation||unaudited"
  "crates/topo/src/merge_faces.rs|loop_winding||unaudited"
  "crates/topo/src/movefac.rs|movefac||unaudited"
  "crates/topo/src/offset_axial.rs|nappe_signed||unaudited"
  "crates/topo/src/pcurves.rs|clear_face_caches||unaudited"
  "crates/topo/src/pcurves.rs|validate_pcurves||unaudited"
  "crates/topo/src/pcurves.rs|walk_loop||unaudited"
  "crates/topo/src/props.rs|loop_edges||unaudited"
  "crates/topo/src/replace_face.rs|boundary_edges_into||unaudited"
  "crates/topo/src/review_m1_pr4.rs|some_single_op_reaches||unaudited"
  "crates/topo/src/seqgen.rs|first_empty_ring_site||unaudited"
  "crates/topo/src/seqgen.rs|mef_chords_candidates||unaudited"
  "crates/topo/src/shell.rs|duplicate_in_loop||unaudited"
  "crates/topo/src/shell.rs|face_boundary_points||unaudited"
  "crates/topo/src/shell.rs|face_neighbours||unaudited"
  "crates/topo/src/shell.rs|rename_loop_surface||unaudited"
  "crates/topo/src/shell.rs|ring_rows||unaudited"
  "crates/topo/src/shell.rs|split_cycle||unaudited"
  "crates/topo/src/splitting/containment.rs|loop_points||unaudited"
  "crates/topo/src/splitting/finish.rs|classify_shell||unaudited"
  "crates/topo/src/splitting/finish.rs|describe_section_boundary||unaudited"
  "crates/topo/src/splitting/join.rs|certify_section_area||unaudited"
  "crates/topo/src/splitting/join.rs|loop_starts||unaudited"
  "crates/topo/src/validate.rs|loop_cycle_of||unaudited"
  "crates/topo/src/validate.rs|plane_every_face||unaudited"
  "crates/topo/src/validate.rs|tier1||unaudited"
  "crates/topo/src/validate.rs|tier3_local_checks_marked||unaudited"
)

# The matchers, in one place. Anchored at the start of a record, which
# is also what makes each site count ONCE: a window whose first record
# is blank (a comment-only line strips to whitespace) repeats the same
# join one line early, so a hit is kept only where the LINE view shows
# the pattern really starting.
PATH_PREFIX='([A-Za-z_][A-Za-z0-9_]*::)*'
DEFER='(continue|break|return)([^A-Za-z0-9_]|$)'
LET_RE="^let ${PATH_PREFIX}LoopBoundary::(Cycle|Empty) \\{[^;{}]*\\} = [^;]*else \\{ *${DEFER}"
ARM_RE="^${PATH_PREFIX}LoopBoundary::(Cycle|Empty) \\{ *(\\.\\.|_) *\\}( if [^;{}]*)? => (\\{ *\\}|\\{ *${DEFER}|${DEFER})"
ANCHOR_RE="^(let )?${PATH_PREFIX}LoopBoundary::(Cycle|Empty) \\{"

gate() {
  gate_require_crate_sources
  local lineview winview report
  # THE READER'S STATUS IS CHECKED, not inherited. `lib.sh` says a
  # matcher that did not run is not a clean scan; this gate reads
  # through awk rather than grep, so `gate_grep`'s marker cannot speak
  # for it and the check is written here instead.
  if ! lineview=$(gate_rust_code "${GATE_SOURCE_FILES[@]}"); then
    gate_error "$(gate_name): the shared Rust reader could not build the code-only line view, so what it did not match is unknown — that is not a pass"
    exit 1
  fi
  if ! winview=$(gate_rust_code --window "$WINDOW" "${GATE_SOURCE_FILES[@]}"); then
    gate_error "$(gate_name): the shared Rust reader could not build the code-only window view, so what it did not match is unknown — that is not a pass"
    exit 1
  fi
  if [ -z "$lineview" ] || [ -z "$winview" ]; then
    gate_error "$(gate_name): the shared Rust reader returned NOTHING over $GATE_SCAN_FILES source file(s) — the scan decided nothing, which is not a pass"
    exit 1
  fi
  if ! report=$(printf '%s\n' "${REGISTER[@]}" "===" "$lineview" "===" "$winview" \
    | awk -v ANCHOR="$ANCHOR_RE" -v LETRE="$LET_RE" -v ARMRE="$ARM_RE" '
      /^===$/ { phase++; next }
      {
        i = index($0, ":")
        file = substr($0, 1, i - 1); rest = substr($0, i + 1)
      }
      phase == 0 {
        ne++
        n = split($0, f, "|")
        if (n != 4 || f[1] == "" || f[2] == "" ||
            (f[4] != "unaudited" && f[4] !~ /^audited: ./)) {
          print "MALFORMED|" $0
        }
        ef[ne] = f[1]; ei[ne] = f[2]; eg[ne] = f[3]; ed[ne] = f[4]
        next
      }
      {
        j = index(rest, ":")
        line = substr(rest, 1, j - 1) + 0
        txt = substr(rest, j + 1)
        sub(/^[ \t]+/, "", txt)
      }
      phase == 1 {
        if (txt ~ ANCHOR) anch[file ":" line] = 1
        if (match(txt, /(^|[^A-Za-z0-9_])fn [A-Za-z_][A-Za-z0-9_]*/)) {
          s = substr(txt, RSTART, RLENGTH); sub(/^[^f]*fn /, "", s)
          ni[file]++; il[file, ni[file]] = line; inm[file, ni[file]] = s
        }
        next
      }
      {
        if (!(txt ~ LETRE || txt ~ ARMRE)) next
        if (!((file ":" line) in anch)) next
        item = "(no enclosing fn)"
        for (k = 1; k <= ni[file]; k++) if (il[file, k] <= line) item = inm[file, k]
        head = txt
        if (txt ~ LETRE) {
          if (match(txt, /else \{ *[A-Za-z_]*/)) head = substr(txt, 1, RSTART + RLENGTH - 1)
        } else {
          if (match(txt, /=> *\{? *[A-Za-z_]*/)) head = substr(txt, 1, RSTART + RLENGTH - 1)
        }
        total++
        hit = 0
        for (k = 1; k <= ne; k++) {
          if (ef[k] != file || ei[k] != item) continue
          if (eg[k] != "" && index(head, eg[k]) == 0) continue
          used[k] = 1; hit = k; break
        }
        if (hit == 0) print "UNREG|" file "|" line "|" item "|" head
        else if (ed[hit] ~ /^audited/) aud++
        else un++
      }
      END {
        for (k = 1; k <= ne; k++) if (!(k in used)) print "STALE|" ef[k] "|" ei[k] "|" eg[k]
        print "COUNT|" total + 0 "|" aud + 0 "|" un + 0
      }
    '); then
    gate_error "$(gate_name): the register comparison could not run, so nothing about the tree was decided — that is not a pass"
    exit 1
  fi
  local bad
  bad=$(printf '%s\n' "$report" | sed -n '/^COUNT|/!p')
  if [ -n "$bad" ]; then
    printf '%s\n' "$bad"
    gate_error "$(gate_name): the deferral register does not match the tree — a MALFORMED line is a register entry that is not <file>|<fn>|<fragment>|(unaudited|audited: …); an UNREG line is a LoopBoundary discard no entry names, so add it to REGISTER as \`unaudited\` (or as \`audited: <the arm that asks the same question about the same pair>\` if you are auditing it now); a STALE line is an entry no discard matches any more, so delete it — a register claiming a disposition for code that is gone says nothing true"
    exit 1
  fi
  local counts total aud un
  counts=$(printf '%s\n' "$report" | sed -n 's/^COUNT|//p')
  total=${counts%%|*}; counts=${counts#*|}
  aud=${counts%%|*}; un=${counts#*|}
  gate_ok "$total LoopBoundary discard(s), every one in the register ($aud audited, $un unaudited)"
}

# --- THE FIXTURE ------------------------------------------------------
#
# THE CLEAN TREE IS WRITTEN FROM THE REGISTER ITSELF, which is the only
# way a gate with both reds can have a passing fixture at all: every
# entry must find a live discard, so the fixture plants one per entry.
# It is also a check on the register's own shape — an entry whose `<fn>`
# or `<fragment>` cannot be reproduced by the reader fails here.
#
# The registered discards are planted in the WRAPPED form `rustfmt`
# produces, so the clean fixture exercises the window join; the firing
# planters below use the one-line form, so both reach the matcher.
gate_plant_clean() {
  local t=$1 e file item frag body
  for e in "${REGISTER[@]}"; do
    file=$(printf '%s' "$e" | cut -d'|' -f1)
    item=$(printf '%s' "$e" | cut -d'|' -f2)
    frag=$(printf '%s' "$e" | cut -d'|' -f3)
    case "$frag" in *return*) body='return false;' ;; *) body='continue;' ;; esac
    mkdir -p "$t/$(dirname "$file")"
    {
      printf 'fn %s() {\n' "$item"
      printf '    for lk in loops {\n'
      printf '        let LoopBoundary::Cycle { first } = body\n'
      printf '            .get_loop(lk)\n'
      printf '            .ok_or_else(corrupt)?\n'
      printf '            .boundary\n'
      printf '        else {\n'
      printf '            %s\n' "$body"
      printf '        };\n'
      printf '        use_it(first);\n'
      printf '    }\n'
      # An arm-form discard under the same `fn`, so the clean fixture
      # carries both shapes. Only where the entry has no fragment: a
      # fragment naming the let-else text would not match this one.
      if [ -z "$frag" ]; then
        printf '    match lp.boundary {\n'
        printf '        LoopBoundary::Empty { .. } => continue,\n'
        printf '        LoopBoundary::Cycle { first } => use_it(first),\n'
        printf '    }\n'
      fi
      printf '}\n'
    } >> "$t/$file"
  done
}

plant_unregistered_let() {
  mkdir -p "$1/crates/planted/src"
  {
    printf 'fn arrived_unregistered() {\n'
    printf '    for lk in loops {\n'
    printf '        let LoopBoundary::Cycle { first } = lp.boundary else { continue; };\n'
    printf '        use_it(first);\n'
    printf '    }\n'
    printf '}\n'
  } > "$1/crates/planted/src/lib.rs"
}

plant_unregistered_arm() {
  mkdir -p "$1/crates/planted/src"
  {
    printf 'fn arrived_unregistered_arm() {\n'
    printf '    match lp.boundary {\n'
    printf '        topo::LoopBoundary::Empty { .. } => {}\n'
    printf '        topo::LoopBoundary::Cycle { first } => use_it(first),\n'
    printf '    }\n'
    printf '}\n'
  } > "$1/crates/planted/src/lib.rs"
}

# One registered file rewritten with its `fn`s and none of their
# discards: the entries survive, the sites do not.
plant_registered_site_gone() {
  local t=$1 e file target
  target=$(printf '%s' "${REGISTER[0]}" | cut -d'|' -f1)
  : > "$t/$target"
  for e in "${REGISTER[@]}"; do
    file=$(printf '%s' "$e" | cut -d'|' -f1)
    [ "$file" = "$target" ] || continue
    printf 'fn %s() {}\n' "$(printf '%s' "$e" | cut -d'|' -f2)" >> "$t/$target"
  done
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

gate_selftest() {
  local want="the deferral register does not match the tree"
  gate_selftest_clean
  # The reader is awk, so an awk that cannot run is this gate's silent
  # failure: it produces no records, and no records is what a tree with
  # no discards produces.
  gate_selftest_without_tool awk "could not build the code-only"
  gate_selftest_case "$want" plant_unregistered_let
  gate_selftest_case "$want" plant_unregistered_arm
  gate_selftest_case "$want" plant_registered_site_gone
  gate_selftest_passes "a let-else and a match arm discarding some OTHER enum" plant_other_enum_discard
  gate_selftest_passes "a LoopBoundary value BOUND and used, not discarded" plant_bound_and_used
  gate_selftest_passes "the spelling in prose, in a doc comment, in a string literal and inside matches!" plant_prose_and_predicate
  printf '%s selftest OK: passes a tree whose every discard is registered, including the rustfmt-wrapped join; fires on an unregistered let-else, on an unregistered match arm and on a register entry whose site is gone; stays quiet on another enum, on a bound-and-used value and on prose; and stays RED, with a diagnosis, when the reader itself cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
