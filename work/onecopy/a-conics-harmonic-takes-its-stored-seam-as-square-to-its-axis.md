---
id: a-conics-harmonic-takes-its-stored-seam-as-square-to-its-axis
kind: issue
title: Conic::along reads the part of a direction across the plane and takes the stored seam as square to the stored axis: a seam 2.3e-14 off square puts a stored point 7.5e-8 m off the extent at a semi-axis of 2.6e6 m
status: open
opened: 2026-11-23
priority: P3
cost: M
---


Filed by ONEROUND's `oneround/conic-readers` (2026-11-23), from a
reviewer's note on PR 7510; here because
`crates/geom/src/curves/conic.rs` is this program's ground.

## What was seen

`geom::Ellipse::anchored(anchor, axis, toward, a, b)` with `toward`
near `axis` stores a seam with `u_ref · axis = −2.3e-14` (the frame's
squaring, `Frame::new`, at an ill-conditioned `toward`). `Conic::along`
(`crates/geom/src/curves/conic.rs`) reads the conic's harmonic along a
direction off the part of that direction ACROSS the plane,
`(â × n) × â`, which is the direction itself as the frame's axes see it
only where they are square. So a stored point stands
`a · (u_ref · axis) · (axis · d)` off the extent `Conic::extent_along`
returns along `d`: 7.5e-8 m at `a = 2.6e6` (the reviewer's probe,
`cover::tests::held_arc_rows`, which nets the skew out of its bar as
`Case::skew`).

Both `Frame::new` and `along` are the merge base's; PR 7510 changed
neither. The miss is larger than the ones that PR removed (2.5e-10 m at
2.2e7), on a fixture no mint produces.

## Which mints produce such a seam

- `plane_cone_section` does not. 6,200 of its tilted ellipses
  (half-angles 0.05, 0.4, 1.2 and 1.55; planes from 0.5 to 1e-7 of the
  way short of the parabola; a temporary probe beside
  `carrier_resolution::tests::a_far_centred_section_stands_at_the_scale_its_form_is_stored_at`,
  not kept): the worst `|u_ref · axis|` is 1.7e-16, and the worst
  `a · |u_ref · axis|` at a semi-axis of at most 1e6 m is 4.5e-11 m.
- The cylinder's section and the re-seams do not either (PR 7510's
  reviewers; not re-measured here).
- **STEP import's placement does, past ε at an ordinary size.**
  `RigidMotion::from_axis_and_reference` (and `Frame::new`, one
  Gram–Schmidt pass) admits a reference within about 1e-6 rad of its
  axis and mints a frame out of square by 4.4e-10: a stored point 1e-9 m
  off its extent at 2.3 m (a reviewer, measured at the mint). That mint
  is `work/importgate/a-step-placement-whose-reference-lies-nearly-along-its-axis-mints-a-frame-out-of-square.md`.

So "under ε at a buildable size" holds of the kernel's own section mints
and is false of an imported placement.

## What is owed

Either the stored seam is square to the axis to a rounding at every
mint (then `along` is right and the squaring door is what to hold), or
`along` reads the harmonic of the stored directions as they are. Which
is `work/onehome/a-stored-frame-reads-as-two-curves-a-rounding-of-the-size-apart.md`'s
question as much as this one's.
