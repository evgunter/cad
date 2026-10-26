---
id: a-boolean-cuts-a-flush-face-kept-once-along-a-slits-line
kind: issue
title: A Boolean whose other solid lies flush with a slit's face beside the slit's line, where the op keeps one of the two flush faces, refuses SlitLineCrossed Flush: the fate cut has no corner to end in
status: review
opened: 2026-10-20
priority: P1
cost: M
branch: cleave/fate-cut-ends-at-a-slit-tip
---


Found by the review of PR 5479 (`cleave/recl-places-germs-along-a-slit-line`).

## What

An operand carries a slit or ring in face F (PR 5380's slit form). The
other solid has an edge along the slit's line, with one of its faces
flush with F beside the line (coplanar, its flank read `Along` F's) and
its other face in the far slit wedge. recl reads two germs on the line
(`crates/topo/src/boolean/recl.rs`, `recl_edges`). Where the op keeps
one copy of the two flush faces (`tables::kept_copy`), placing them
leaves the fate stage a cut along the line that it cannot end:
`ClassificationInvariant` "a fate cut's end lies in no corner of the
dropped face" (`boolean::fate`). So recl refuses first, typed
`SlitLineCrossed { gap: SlitLineGap::Flush }`, no lever, wherever such a
flank reads `Along` on a slit's line holding two germs
(`FlankRead::along`).

## Witnesses

The plate less the void lying under its top
(`below_the_top(lying(0.0))`), against the tetrahedron at `MEET` over
`[bearing(60, 0.6, 0), bearing(150, 0.6, 0), bearing(20, 0.6, −0.7)]`
and its mirror over the ray, `[bearing(60, 0.6, 0), bearing(−30, 0.6, 0),
bearing(100, 0.6, −0.7)]`. At rest and turned, at ε default, 1e-6 and
1e-12: `x ∩ y` and `y ∪ x` refuse `Flush`, 8 cells at each ε. `x − y`,
`y − x`, `y ∩ x` and `x ∪ y` build (one germ, or no kept copy), tier 3
and 3′, the line's edges the material's creases. Pinned by
`a_partner_flush_with_the_rings_face_refuses_typed_where_one_copy_is_kept`
in `crates/topo/tests/a_ring_crossed_along_its_line_places_its_germs_apart.rs`.
Main refused all six `SlitLineCrossed`.

## Fix

Give the fate stage a corner where the germs' cut along the slit's line
ends in the dropped flush face, or place the germs so the cut runs
along an edge of it. Then drop the `Flush` arm and re-pin the cells as
building.
