---
id: blend-reach-may-refuse-a-valid-thin-cylinder-as-degenerate
kind: issue
title: blend: the band reach may refuse a valid cylinder of tiny radius as Degenerate, with no recourse
status: open
opened: 2026-10-14
priority: P3
cost: M
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
