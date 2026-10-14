---
id: blend-reach-may-refuse-a-valid-thin-cylinder-as-degenerate
kind: issue
title: blend: the band reach may refuse a valid cylinder of tiny radius as Degenerate, with no recourse
status: closed
opened: 2026-10-14
priority: P3
cost: M
closed: 2026-10-14
---

## Finding

Raised by the review of PR 5116 and not chased there. `reach.rs`
`band_reach` maps every `FaceBoxes::of` failure to `Breach::Degenerate`,
which renders "There is no way through". `face_box_rule`
(`topo/src/boolean/boxes.rs`) builds a cylinder's slab axis with
`UnitVec3::levered(axis, BOX_CYLINDER_AXIS, band, radius)`, levered by the
radius. That constructor's own doc says an arm "little longer than the band
refuses even a unit vector", and it refuses an arm the band does not decide
positive (`LeveredUnitError::Arm`).

So a cylinder whose radius is definitely positive but within a few band
widths may fail the box while tier 3 admits it. In that case a valid
at-rest body would reach `Degenerate`, a sentence that calls the body
broken, with no recourse.

## What the taker owes

Find whether tier 3's representability margins admit a cylinder whose
radius the levered axis decision refuses, at `Tol::witness()` and at the
1e-6 and 1e-12 rows. If they do, pin a row that reaches it through
`fillet_edges`. Then either have the box admit it (the axis is unit by
convention, and its lever could be the face's extent rather than its
radius) or give the refusal a cause and recourse that fits a thin cylinder.
If they do not, close this with the margin that excludes it.

## Closed

The gap was real at the margin and no door reached it. Tier 3 check 1
admits a cylinder whose radius is exactly positive
(`analytic_datum_verdicts` reads `radius.lo > 0`), and the box's
levered axis refused one the band reads as zero or in band
(`UnitVec3::levered`'s arm gate), so tier 3's margins do not exclude
it. Every door swept refuses that radius by a band decision of its
own, at witness eps, f64: a block with a circular through-hole of
radius r, filleted and chamfered (0.1) on an outer edge, builds
through the profile and extrude down to r = 1e-8 and passes both
verbs; at r ≤ 1e-8 the profile refuses (`segment_straightness`,
`vertex_separation`, `DegenerateSegment`). Through STEP (the same
block exported at r = 0.25, the radius rewritten) it imports and
passes down to r = 1e-8, and refuses below it at the edge
certificate (the dihedral arm, `min(curvature arms, extent)` on the
same band, and the interval span). A fillet's own cylinder refuses r
≤ 1.2e-8 at its headroom and curvature checks. The certificate's arm
gate nearly closes the chain, but not for every face: a cylinder face
whose neighbours are all on its own or a coincident surface is not
levered by it.

So the box no longer needs the lever to box a cylinder face.
`face_box_rule` takes the boundary hull where the levered axis does
not decide but the axis is a direction (tier 3's own
`validate::is_direction`): a cylinder is ruled, so every point of a
cylinder face lies on a ruling segment between two boundary points,
and the face lies in its boundary's convex hull, as a plane face does.
An at-rest cylinder therefore always boxes, and `band_reach`'s
`Degenerate` is left for an axis that is no direction, which tier 3
refuses. The row is
`boxes::tests::a_cylinder_whose_radius_the_band_does_not_decide_boxes_by_its_boundary`,
both lanes, under a band reading the radius in band and one reading it
zero.
