# PR 3985 (the arc derives from the pairing): LAST fix pass

The dual review is in, frozen at 7e33abf087. Both reviews are APPROVE-WITH-FIXES with no MAJOR and no wrong body found. The reports are on analysis/reach-dual/3985-r1 and -r2; read both. "A" and "B" below label the two reviews blindly.

This is the LAST fix pass. The orchestrator verifies it and runs the mutants and rows itself; there is no further review round.

## Rulings
- Merge main first (the head conflicts in 7 files). Then re-run your chord differential and the mutants on the merged head, and report the numbers in the PR body.
- Your branch has moved past the frozen head, to 9922b0ac. Say in the PR body what that commit changed, so the verification covers it.
- F1: re-measure the `Undecided` arm on the merged head and fix its premise. If it is now unreachable, say so and pin that.
- F3: fill in the PR body's record of the phase-1 cross-check. Make it consistent with the unit item's counts.
- F4: give the walk-order rungs at `boolean/join.rs:1537-1571` a row of their own, one that a mutant on each rung turns red.
- F2 and F5: sweep the stale prose, in the demos README, the audit doc rows, `snowman.rs` and `lily.rs`.
- Items under "File rather than fix" are filed as work items, not fixed.
- Work under the intent-refactor hold, which is in the d10 item. This is a started unit, so finish it, but don't widen it.
- Run the full battery at all three ε. Don't merge, and post no comments. Update the PR body with a finding → change map.

## 5. Consolidated fix list (deduplicated, by severity)

### Fix in this PR (MINOR)
1. **Merge main and re-run (F6).** Resolve the conflicts in `chord_join.rs`, `boolean/join.rs`, `splitting/join.rs`, `sweep/tests/snowman.rs`, `sweep/tests/tilted_sphere_pair.rs`, `.config/nextest.toml` and `work/join/join-ranks-conic-facing-germs-by-chord.md` (main carries JOIN-3's chord plan). Then re-run the claim-1 differential (B's `probes/chord_dump_*.py`, or A's digest over the sweep probes and editor unions) and the mutant battery on the merged head.
2. **`crates/topo/src/boolean/reduce.rs:3335-3344` (F1).** Delete the `Join(SectionArcWindow{BothContained})` premise. Re-measure whether the collar∖wedge bore (now built by `wedge_through_a_full_turn_collar.rs`, all 48 poses) or any other pose reaches `Placement::Undecided`, and state the measured answer. If `Undecided` is now reachable, pin it with a row that does reach it. If it is not, say so with what was tried.
3. **Cross-check record (F3).**
   - Fill the PR body with the phase-1 cross-check counts. A's re-run of `2f49b372` reports 39,512 agree, 8,575 old-refused and 7 disagree.
   - Name the disagreement classes, including the cylinder-wall ones: `reach_slab_cut_sector_side` [boss,slab,plate] and the `germ_coplanar_conic` tube strut.
   - State that phase 1 predates the shipped pairing (`cae80322`), or re-run the cross-check against the shipped pairing.
   - Make `work/reach/arc-side-rule-has-two-predicates.md:77-79` match what the body actually holds.
4. **Walk-order row (F4).** Add a row of its own for `bool_join_walk_order` / `bool_join_walk_site` (`boolean/join.rs:1537-1571`): four alternating crossings on one section circle (A's slab4 probe: crossings at −40°/60°/120°/220° and −10°/80°/100°/190°, pole y and turned). It must go red, by refusing or by a wrong body, when `walk_passes` is disabled. Add a pose that reaches the in-band "coincides with an end" branch, or record why none can.
5. **Stale prose citing the retired selectors (F2 + F5, one sweep).** Grep the whole tree for `SectionArcSide`, `NoCertifiedRun`, `SectionArcWindow`, `run_is_section_arc`, `bool_between_arc_window`, "run-side arc rule" and "face's window", and fix every hit:
   - `demos/README.md:62` (snowman row: z-moved head now builds; spun and x-moved heads go through the pairing)
   - `docs/predicate-dimension-audit.md:525`, `:536` and F8 at `:789-791`
   - `demos/tour/src/snowman.rs:378`, `demos/tour/src/lily.rs:4127`
   - `crates/sweep/tests/m6_rider.rs:61`, `verbs_sphsph_opening.rs:22`, `crates/geom-core/tests/cert4r?_probes.rs:101`

### Pin or tighten (NOTE and style, in this PR at the implementer's judgement)
6. **Two levers (S-a, S-g).** Make the chord read the sign its pairing already decided, rather than re-deciding it in `chord_arc_leave` (`chord_join.rs:~998-1015`) beside `split_join_conic_heading` (`splitting/join.rs:483`). At minimum, extend `the_chord_arc_rung_is_decided_in_one_place` to count the split's rung too, and split the one-name/two-quantity margin.
7. **Overclaim (S-d).** Narrow the "every carrier and tilt" wording in the `chord_join.rs:76-82` module doc and the PR title to what reaches the rule: cyl/sphere partners; the split refuses spheres.
8. **Rows that cannot go red.**
   - S-i: tighten the `germ_coplanar_conic.rs` `every_op…` row so that it cannot pass on both base and head (assert build or refusal per pose).
   - O-a: check whether `a_pip_with_its_seam…` and `the_die_pips_shape_stops_typed…` can go red under any change in scope; if not, say what they guard.
9. **Collar row gate (N-g).** The collar row is nightly-only (`.config/nextest.toml:20`). Either accept that and say so in the PR body, or put a fast pose in the PR gate.
10. **`leave` convention (S-h).** Assert or document the `choose_roles` own-halves dependency at `boolean/join.rs:631`.

### File, don't fix (pre-existing or structural)
- **Scale-dependent refusal door (N-f).** `bool_join_nearest`'s absolute-metre chord ranking means bar-through-ball pierces refuse `Escalated(bool_join_nearest)` at ×1e-3 and `RingOffCylinderChart` at ×1/×1e3. File this as a new issue unless one already covers `bool_join_nearest`.
- **Window machinery hosted in `chord_join.rs` (S-c).** Move it to its remaining consumers' home (`solid_contain.rs`, ring side), or file that move. This is a reshuffle, so it needs no Ev question.
- **Two walk-order spellings (S-b).** `walk_passes` and `conic_pairs` should share one walk, or be reconciled by a row rather than by prose. File it.
- **`walk_passes` cost (S-j).** Cubic in open germs. File it only if a measured case shows a cost; tour time is unchanged.
- **Anti-re-fork row blind spot (S-e).** File it against the row's design, if item 6 does not close it.
- **Already filed, nothing new.** `point_in_solid` on carved spheres (N-b); the sphere-ring `RingOffCylinderChart` (N-c); the ε 1e-6 and 1e-12 base reds (N-d: `work/cleave/steep-tube-split-refuses-trim-containment-at-eps-1e-6.md`, `work/exch/arc-loft-native-volume-exhausts-the-quadrature-budget-at-eps-1e-12.md`).
- **Brief premise (N-a).** Claim 3b, the `insert.rs` strut-facing swap, belongs with whatever unit carries that change, not with this PR. Correct the brief template if the claim is reused.
