---
id: boolean-doors-build-overlapping-coincident-edges-from-inputs-that-hold-none
kind: issue
title: Boolean doors build coincident edges with overlapping material wedges from inputs that hold none, and every tier passes them
status: closed
opened: 2026-10-14
priority: P1
cost: H
refs: [reverting-a-slit-body-overlaps-its-coincident-edges-wedges-and-passes-every-tier, census-passes-a-face-inside-an-edges-material-wedge]
branch: cleave/gate-refuses-overlapping-wedges
closed: 2026-10-19
---

Found by the fix pass on PR 5106 (`cleave/revert-repairs-coincident-edges`),
measured on `origin/main` with ∖'s new check bypassed. A silent wrong
answer: the volume is right and tiers 3 and 3′ pass.

## What

Where a Boolean result has two or more edges on one line, each edge
classifies its own material wedge (D1's coincident-distinct-edges
class), so the wedges must be disjoint. These results carry wedges
that overlap, built by the join and the finish, not by `revert`. Either
input holds no such line, or the line the result carries is not the
input's. Each witness is one edge pair on one line: the two wedges and
their overlap.

| Witness (test, op) | Wedges | Overlap |
|---|---|---|
| `review_4740_r1_probes::a_subtraction_leaving_two_quarters_records_its_touch`, ∖ | 270°, 270° | 180° |
| `review_4740_r1_probes::an_intersection_of_two_reflex_blocks_records_its_touch`, ∩ | 270°, 270° | 180° |
| `boolean_pinch_copies::pinch_copies_do_not_reach_rest`, ∖ (two lines) | 270°, 270°; 90°, 313.6° | 180°; 43.6° |
| `review_4740_r1_probes::a_kiss_the_op_drops_records_no_edge_edge_row`, ∖ | 90°, 310° | 40° |
| `pierce_strut_at_a_pinch::the_staircase_and_the_plate_build_in_every_op`, ∪ and ∩ (two lines each) | 270°, 270° | 180° |
| `union_flush_onto_edge_contact::a_pinch_line_flat_in_the_shared_corners_face_builds_in_every_op`, ∪ | 260°, 190° | 180° |
| `union_flush_onto_edge_contact::a_dangling_null_edge_inside_another_along_one_end_builds_in_every_op`, ∪ | 330.8°, 195.6° | 180° |

A union of two solids touching along a line should leave two 90°
creases, or one where a face continues; the 180° overlaps are each
solid's two faces paired with the other's.

## The check

`census::line_wedges` (PR 5106) reads the material wedges of a body's
edges on one line and says whether two overlap. ∖ runs it on the lines
of its operand it reverts. Run at the gate over every line where
coincident edges lie, it is tier 3′'s side check
(`census-passes-a-face-inside-an-edges-material-wedge`, restread), and
it refuses this whole class.

## Owed

The pairing in the join and finish that these doors take at a line
where four or more sheets meet: the material decides the edges (the
`[ev]` PR 4795 principle). Until then, the side check refuses the class
typed rather than passing it.

## The gate refuses the class (branch `cleave/gate-refuses-overlapping-wedges`)

The Boolean's result gate (`boolean/ops.rs` `gate`, its
`coincident_wedges_disjoint`) reads every line where two or more of the
result's edges lie (`census::collinear_overlaps`) and each line's wedges
(`census::line_wedges`), for every op and at every scalar. An overlap is
`BooleanError::LineWedges { fault: Overlap }`, a kernel limitation
naming the missing re-pairing with no lever ("There is no way through
yet"); an in-band reading is `Escalated { LineWedges }`. ∖'s
reverted-line check (PR 5106) is folded in: every overlap it found the
gate finds, so it and `RevertError::WedgesOverlap`/`NoSheet` are gone.
A wedge end on the line (a cusp or slit edge, which the reader has no
interior direction for) refuses typed (`LineWedgeFault::Unread`, `Degenerate`) rather than as an
in-band escalation.

Census (topo, sweep, editor-core, slow set included, ε default, 1e-6,
1e-12): every newly refusing op reads overlapping wedges on main. The
set is the same at all three ε but for one probe matrix, whose count of
refused cells moves with ε because which of its cells reach the gate
does (146, 125, 141; `a_vertex_read_again_classes_every_edge`'s
`PARTNERS_OVERLAPPING`). The row's witnesses are pinned at their
refusals beside their fixtures. What stays owed is this row's re-pairing
in the join and the finish (the material decides the edges, ratified on
`[ev]` PR 4795); the pins that return with it are listed on
`work/topo/revert-re-pairs-the-sheets-on-a-line-where-coincident-edges-meet.md`.
