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

## A pose where it decides (2026-11-23, review of PR 7543)

The review of `oneround/conic-pair-readers` (`analysis/oneround/conic-pair-readers/review-report.md` on `oneround/conic-pair-readers-review`, M2; probe `review/probe_far.rs`) found it decide a wrong answer through `curve_pair::meet`. On the arc-held ladder's fat section at `a` = 1e9 (`plane_cone_section`'s numbers, anchor near (1.3, −0.7, 0.4)), the stored seam stands 6.8e-17 off square to the stored axis, so the curve the stored numbers state leaves the plane through its anchor normal to its axis by `a·|u_ref·axis|·(1 − cos θ)`: 8.3e-9 m at θ = 0.5, 1.35e-7 m at the antipode. A line square to the arc at θ = 0.5, lifted 1e-8 m off that plane, stands 7.89e-9 m over it at 60 digits while the stated curve stands 8.27e-9 m over it: the true gap is 3.8e-10 m. At ε 1e-9 `meet` answers `Ok`, no meeting; `Conic::height_of` of the line's `f64` point reads 1.49e-8, and the log ends on `curve_pair_pose_level_arc` and `curve_carrier_piece_clear_by`. The pose stands 1.2e5 m from the stored point, past the session box (1e3 m). PR 7543's doors (`Conic::height_of`, `Conic::level_at`, `Height`) take the frame as square and say so in their docs.
