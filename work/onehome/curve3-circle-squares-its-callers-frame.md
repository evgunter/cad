---
id: curve3-circle-squares-its-callers-frame
kind: issue
title: Curve3::circle squares a frame from raw directions for its callers, a Frame::new behind a second name
status: open
opened: 2026-11-23
priority: P2
cost: M
refs: [carrier-frames-the-production-mints]
---

## What

`geom::Curve3::circle(center, axis, radius, u_ref)`
(`crates/geom/src/curves/conic.rs`, `Curve3::circle`, about line 1138)
is `Circle::new(Frame::new(center, axis, u_ref), radius)`: the
undecided squaring of `Frame::new`, reached under another name. A sweep
by `Frame::new(` cannot see a caller of it, so narrowing or renaming
`Frame::new` leaves these mints in place, each still squaring arithmetic
that can degenerate (a trim circle's seam read off a point on its
axis) into poison rather than refusing typed. The ellipse held on its
arc (`Ellipse` anchor constructor, `conic.rs` about line 746,
`at: Frame::new(anchor, axis, toward)`) is the same shape.

Found by the blind-spot pass of `carrier-frames-the-production-mints`'s
G1 (sweep) sweep (branch `onehome/mints-sweep`, 2026-11-23).

## Sites in sweep (production, as of that branch)

- `blend/arms.rs` `plane_sphere_blend`, the two trim circles (seam
  the torus's decided `u_ref`, so the squaring there re-squares a
  frame's axes: class A);
- `blend/arms.rs` `Meridian::blend`'s trim closure (seam `±radial` by
  `ring_radius`'s undecided `unit_sign`) and `chamfer_frustum`'s trim
  closure (seam the sheet's radial): both re-place the meridian's
  decided frame, so class A once the sign is a `select_le_zero`;
- `blend/surgery.rs` `attach_contact`'s arc arm
  (`Curve3::circle(center, n, radius, u)` with `u = flat(p0)`, a
  direction read back off a stored vertex, the class
  `band/a-blend-arcs-seam-reference-is-read-off-its-vertex-less-its-centre`
  already names): class B.

Tree-wide, `grep -rn 'Curve3::circle(' crates --include=*.rs` outside
`tests/` and `examples/` returned 434 hits on 2026-11-23, many of them in in-file test
modules; the production share per crate is unmeasured.

## Shape

The narrowing follow-up that makes the decided mint the path of least
resistance for `Frame::new` takes `Curve3::circle` with it: a circle
built from a frame its construction minted (`Circle::new(frame, r)`),
and `Curve3::circle` narrowed the way `Frame::new` is.
