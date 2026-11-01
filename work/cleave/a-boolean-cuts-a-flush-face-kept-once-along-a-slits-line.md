---
id: a-boolean-cuts-a-flush-face-kept-once-along-a-slits-line
kind: issue
title: A Boolean that keeps one of two flush faces where one carries a slit cuts the dropped one through the slit's tip, which the fate cut reads as a cusp
status: closed
opened: 2026-10-20
priority: P1
cost: M
branch: cleave/fate-cut-ends-at-a-slit-tip
closed: 2026-10-26
---


Found by the review of PR 5479 (`cleave/recl-places-germs-along-a-slit-line`).

## What

An operand carries a slit or ring in face F (PR 5380's slit form), and
the other solid has a face flush with F. Where the op keeps one copy of
the two flush faces, the coincident fate cut (`crates/topo/src/boolean/fate.rs`,
`cut`) divides the dropped copy where its holder changes. On main
(`42af7c0b48`) that refuses three ways:

- recl refuses first, `SlitLineCrossed { gap: Flush }`, where the
  other solid's edge runs along the slit's line with a flank flush with
  F (`recl.rs` `recl_edges`);
- the join refuses first, `SlitFaceGlued`, where it cuts the flush face
  along the slit's two edges and the op drops the sliver between them
  (`join.rs` `resolve_roles_geometric`, PR 5538's guard);
- the fate cut refuses `ClassificationInvariant`: "a fate cut's end lies
  in no corner of the dropped face" or "a fate cut leaves a slit in the
  dropped face" (FLUSH's row
  `work/flush/a-fate-cut-refuses-a-slit-floor-flush-with-an-inner-slab.md`).

## Witnesses

- The plate less the void lying under its top, against the tetrahedra
  at `MEET` flush with the top beside the ring
  (`a_ring_crossed_along_its_line_places_its_germs_apart.rs`,
  `a_partner_flush_with_the_rings_face_builds_where_one_copy_is_kept`).
- The keel's slit forms against flush slabs and boxes, and the plate's
  ring against flush boxes
  (`a_keel_on_the_floor_subtracts_as_a_slit.rs`: `a_box_flush_under_a_ringed_top_builds_in_every_op`,
  `a_slab_flush_with_the_slit_floor_intersects_it_keeping_the_keel`,
  `a_flush_partner_crossing_the_slit_intersects_it_in_either_order`,
  `a_partner_flush_beside_the_keel_unions_with_the_slit_at_tier_three`,
  and the restored row of `a_face_glued_to_the_slit_floor_merges_its_two_edges`).

## Found (branch `cleave/fate-cut-ends-at-a-slit-tip`)

Three readings of the fate cut, one mechanism under all three refusals:

- **A slit's tip read as a cusp.** `places` reads each corner at a cut's
  end by `corner_holds`. At a slit's or zero-area ring's tip the
  corner's two bounds are distinct straight edges leaving along one ray;
  its zero-turn arm held a direction only where both bounds were one
  dangling edge, so the cut's end, or a corner's dropped sector, lay in
  no corner. `fate::tip_wraps` now reads such a corner from its loop's
  turns: on a plane face a loop's corners turn a whole turn
  (counterclockwise for the outline, clockwise for a ring), and a
  corner whose bounds leave along one ray turns a half-turn, clockwise
  round a tip the face wraps, counterclockwise round a spur. A
  zero-area ring turns at two tips, a sliver's outline at two spurs, an
  outline with a slit at its tip. A curved face, a curved edge on the
  loop, or half-turns both ways refuse `SlitFaceGlued`, typed; two
  curved edges leaving tangent are still a cusp at this order.
- **A cut along a slit where the holder does not change.** Two fate
  edges of one holder overlapping along a length
  (`census::collinear_overlaps`) over one dropped face, with one kept
  face on both sides after the join, are pieces of a slit's two edges:
  the kept face holds the dropped one across the line, so `cut` mints no
  chord along them ("leaves a slit"). Where the join has divided the
  kept face along the line, the holder changes there and the cut stays,
  as on main. `border_fates` lists the candidate pairs; `cut` compares
  the kept faces after the join.
- **Two touches on one point.** A slit's open end holds one vertex per
  wedge on one point. Each touched the dropped face at its own lone
  vertex, so the two cuts through them did not meet. `cut` now mints one
  lone vertex per touched point key.

recl's `SlitLineGap::Flush` arm and PR 5538's guard then fire on no
scene that reaches them, and are removed with the `Flush` variant and
`FlankRead::along`. `y ∪ x` with the ring's flush partner keeps only
the top on both sides of the line and refuses `Closes`, which is
`a-boolean-closes-a-slits-face-across-its-line`'s gap.

**Outcome diff.** The review's probe (2043 cells per ε: the keel's slit
forms against 16 flush boxes, the ring against flush partner
tetrahedra, boxes under the ringed top, two lumps touching along an
edge; six ops, three poses), release builds of main `42af7c0b48` and
the branch, at ε default, 1e-6 and 1e-12: every cell main builds is
identical on the branch, counts, volume, tiers, records and
`naming.substituted` row for row. 234 cells per ε move from a refusal to
a build, tiers 1, 2 and 3; 231 of them pass 3′, and the three that do
not are the slit-first union of
`a-union-beside-a-slit-misses-an-edge-overlap-record` (P2). 54 per ε
move between typed refusals: `Flush` to `Closes` (48) and to
`CrossedTwice` (6). Still refused on the branch: `PinchRaysInAFace` for
the two lumps' partner ∩ lumps, as on main.

The probe's other kernel-defect refusals are the same on main and the
branch, and are filed: `a-box-corner-at-a-rings-tip-reads-two-crossing-germs-along-one-direction`,
`a-fate-cut-end-between-two-touching-lumps-lies-in-no-piece`,
`a-ring-partner-inside-the-ring-unions-result-invalid` (P1). Also filed
from this row: `a-union-beside-a-slit-misses-an-edge-overlap-record`
(P2), `a-sliver-of-more-than-two-edges-is-not-read-as-one` (P3),
`slit-face-glued-has-no-witness` (P3).

**With PR 5544 on main.** `Closes` is gone with recl's `kept_slit_face`
reading, and the ring's flush partner `y ∪ x` reaches the output
stage. There it refused `Merge InvalidInput` "declared surface key does
not resolve": `finish_output` (`boolean/ops.rs`) read the declared
surface pairs before 5544's `slit::close_struts`, whose `kev` runs
outside a surgery scope and sweeps the arena, so a declared surface no
face held (the dropped flush copy's) was gone when the merge read it.
`finish_output` now drops a pair whose side the closing consumed, the
rule `declared_surface_pairs` already applies to a consumed side. All
six ops build with the flush partners, left and right, at rest and
turned, at the three ε, the two orders of each op alike.
`SlitFaceGlued` carries `SlitFaceGap` (`RingRun`, `ArcsSeparate`,
`TipUnread`).
