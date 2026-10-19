---
id: boolean-doors-build-overlapping-coincident-edges-from-inputs-that-hold-none
kind: issue
title: Boolean doors build coincident edges with overlapping material wedges from inputs that hold none, and every tier passes them
status: review
opened: 2026-10-14
priority: P1
cost: H
refs: [reverting-a-slit-body-overlaps-its-coincident-edges-wedges-and-passes-every-tier, census-passes-a-face-inside-an-edges-material-wedge]
branch: cleave/gate-refuses-overlapping-wedges
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
