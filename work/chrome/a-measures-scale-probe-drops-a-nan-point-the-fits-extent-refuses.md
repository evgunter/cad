---
id: a-measures-scale-probe-drops-a-nan-point-the-fits-extent-refuses
kind: issue
title: measured_at_scale reads a body's size off the scale probe through NaN-dropping min/max, while the δ fit's extent refuses the same mesh
status: open
opened: 2026-11-25
priority: P3
cost: E
---



Found by the review of `vgeom/substitution-sweep` (PR 7764). That PR
made `scene.rs`'s `extent` answer `None` when the scale probe's
points span no finite box. This row is the sibling it left, on CHROME
ground (`combine.rs` is not VGEOM's by paths).

**What.** Two doors measure a body's size off its mesh at
`scene::SCALE_PROBE_DELTA`, as a diagonal, and they disagree about a
poisoned point:

- `scene.rs` `extent` (the δ fit's ladder start) goes through
  `bvh::Aabb::from_points`, whose `pmin`/`pmax` propagate NaN. A
  non-finite diagonal is `None`, and the ladder starts where the
  probe's count was read.
- `combine.rs` `measured_at_scale` (the step that clears a body,
  `step_clear_of`, and an inserted part's frame, `pickindex.rs`
  `clear_of_drawn`) goes through `width_along`, whose
  `fold(… lo.min(r), hi.max(r))` uses `f64::min`/`max`. Those answer
  with the other operand against a NaN, so a NaN position is silently
  absent from the width and the measuring chord is sized off the rest.

An overflowing width goes the other way and probably
refuses downstream (an `inf` chord is not a tessellation tolerance),
but that was not checked.

**Fix shape.** One door for "a mesh's size at the scale probe",
answering `None` for a box that is not finite, read by both. Or align
`width_along` on `bvh::Aabb`'s NaN-propagating fold and say at the
site what the clearance step does with `None` (it already answers
`Ok(None)` for no extent). Mesh positions carry no finiteness
guarantee in `mesh::types`, so whether a NaN position is reachable at
all decides the priority; the sweep did not establish it.
