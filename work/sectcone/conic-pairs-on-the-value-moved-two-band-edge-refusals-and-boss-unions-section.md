---
id: conic-pairs-on-the-value-moved-two-band-edge-refusals-and-boss-unions-section
kind: issue
title: PR 7354 (conic pairs on the value) moved two sweep band-edge refusals and boss_union's section, unjudged; the sweep rows are held red and the editor-core pins hold the move
status: open
opened: 2026-11-23
priority: P1
cost: M
refs: [main-red-eleven-editor-core-and-sweep-rows-moved-on-main-after-c390e1f9, 7354]
---


PR 7354 (`sectcone/conic-pairs-on-the-value`, merge `4a94f7109b`)
moved five rows that its own body does not name. They were found and
bisected by BOXFINISH in
`work/green/main-red-eleven-editor-core-and-sweep-rows-moved-on-main-after-c390e1f9`.
None of the five has been judged. GREEN U18 closed that row on
2026-11-23 and left the judgement here. Three of the rows are held
pins and two are pinned red, so main is green on all five. Each pin
cites this row, and each goes when this row is judged.

## The two `sweep` rows: refusals that changed kind at the band edge

Bisected on hosted CI. Both rows pass on `4a94f7109b^1` (`5ea1ac0a70`,
run 45312853278) and fail on `4a94f7109b` (run 45312883535). On main
`a433ff83d6` (run 45307943329) they fail with the same text, at the
default ε.

- `sweep::all bisector_nodes::the_near_node_poses_at_the_witness_band`
  (default ε only, because the row stands down at any other).
  At δ = 1e-8, the cell `A's cap, direct (B, A), ∪` used to build, or
  refuse with its rim's placement escalated (`Containment`). It now
  refuses as
  `Err(Euler(RebasedCarrier { edge: EdgeKey(15v3), error: Escalated { check: TransversalityArm, sample: 1, … margin 9.999999911469715e-9 …, predicate: Some("dihedral_arm_wedge") } }))`.
  Two questions are open. Is a dihedral arm read at a margin of δ the
  right refusal at the escalation edge? And should it reach the
  Boolean as an untyped Euler error, rather than as a
  `BooleanError::Escalated` decision?
- `sweep::all section_end_poses::a_section_end_at_the_band_edge_refuses_as_the_inputs_at_the_boolean_and_its_lever_builds`
  (default ε and 1e-6; it skips at 1e-12). The row's Boolean poses
  used to refuse at the join as
  `JoinError::SectionEndEscalated { reading: SquareToAxis }`, with its
  question and lever. In every op and order they now stop earlier, as
  `Escalated { decision: Coincidence(Section, Moot) }`. The predicate
  is `curve_carrier_gap` (margin ≈ t) on the square box and on one
  order of each 1e-6 wall tilt, and `pc_axis_plane_parallel`
  (margin ≈ 2t) on the rest. The pin's `MOOT_AT_THE_SECTION` table
  holds every line. The refusal now carries no section-end reading and
  no lever, so the row's lever half (the face square, or clearly off
  square) is not reached at those ε.

**Held:** each row runs under `test_utils::pinned_red::row` at the ε
where it is red. The pin names main's stop, with the margins. At the
other ε each row runs as written.

## The three `editor-core` pins: `boss_union`'s section, re-seated

Read from 7354's diff, not shown: `boss_union`'s section circle is
seated again per carried component, the same re-seating 7354 states
for `mesh`'s `route_rows`. CIW's PR 7501 pins it twice:

- `seat7_sweep_lowering::the_sweep_documents_evaluate_to_their_committed_digests`'s
  `boss_union` digest, `0x8d66_5160_109c_0ae9`;
- `m10_p_fence::the_interval_corpus_holds_sectcones_move_unjudged`, the
  id-masked `Interval` corpus. That pin also carries PR 7299's narrowing
  of `cut_cylinder`, which GERM has judged.

The third row, `revalue_corpus::revalue_boss_union_apart_to_its_parting`,
read `(2390, 3856)` after 7354. Later merges have re-cut its pin twice,
to `(2399, 3856)` (`aca51efe7e`) and then to `(2372, 3856)` (PR 7396's
least-turn re-pin), so 7354's part of the move is in that pin
unjudged too. The row is red on main again for another reason, filed in
`work/green/main-red-eleven-editor-core-and-sweep-rows-moved-on-main-after-c390e1f9`'s
closing section.

**Owed:** judge each move. Where the new answer is right, re-pin with
the reason, and drop `pinned_red` where it is used. Where it is wrong,
fix the reader. Reproduce with
`cargo nextest run -p sweep --ignore-default-filter -E 'test(=bisector_nodes::the_near_node_poses_at_the_witness_band) | test(=section_end_poses::a_section_end_at_the_band_edge_refuses_as_the_inputs_at_the_boolean_and_its_lever_builds)'`.
Each pin then fails with "and it passed" once its row is fixed.
