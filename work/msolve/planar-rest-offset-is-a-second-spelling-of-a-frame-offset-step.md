---
id: planar-rest-offset-is-a-second-spelling-of-a-frame-offset-step
kind: issue
title: PlanarRest's offset is a second spelling of a normal step on the a side's frame offset
status: open
opened: 2026-10-03
priority: P3
cost: M
---


Filed by PLACE's mate-frame-offset unit (branch
`place/mate-frame-offset`), which made a mate frame a base composed
with an offset (`[ev]` #3920) and left this fold out on purpose.

## What

`MatePrimitive::PlanarRest { offset }` (`crates/editor-core/src/mate.rs`)
translates `a`'s frame by `offset` along its own axis before the coset
is read (`crates/editor-core/src/mate/solve.rs`, `mate_coset`'s
`PlanarRest` arm: `fa * translation(z * offset)`, subgroup
`Planar { normal: axis }`). A frame offset whose last step is the same
normal translation (`MateFrame::on_face(Frame::translation([0, 0, d]))`
on side `a`) composes to the same target and the same normal, so the two
spellings pin one coset. Two spellings of one datum, which the ruling's
general form makes redundant.

## Why it was not folded in the same unit

- The primitive's offset is one of the three readers
  `MatePrimitive::authored_lengths` exists for (the lever, the edit
  door's finiteness, the content key). Folding it retires that
  mechanism (no primitive would author a length), which is its own
  review.
- It is a wire break of its own (`PlanarRest` becomes a unit variant)
  and touches every author of a planar rest: the Python
  `MatePrimitive.planar_rest(offset)` constructor and its census rows,
  the viewer's mate tool choice, the tour's refusals stop, and the
  msolve suites that author standoffs.
- The fold changes which side carries the standoff (always `a`'s
  frame today; either side's offset after), so the lever's terms move
  from `Σ|authored lengths|` into `‖origin‖`; the bits of every levered
  clash on a standoff move and need re-baselining with a reason.

## What it wants

Retire `PlanarRest`'s offset: the variant becomes a unit, a standoff is
written on the frame, `authored_lengths` goes, and the levered-clash
rows re-baseline. Old files refuse `Unreadable` with the regenerate
recourse, as the frame change did.
