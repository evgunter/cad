---
id: coplanar-ellipses-one-curve-bound-compares-frames-not-loci
kind: issue
title: two coplanar ellipses are read one curve by a frame comparison, not a locus bound
status: open
opened: 2026-10-14
priority: P2
cost: M
---


`geom_brep::curve_pair::pose`'s two-ellipse arm (`conic_conic`,
`Coplanar::Ellipses`, in `crates/geom-brep/src/curve_pair/pose.rs`)
decides one curve through `frame_gap`, which compares the two frames
point-by-parameter over only the eight signed permutations `Q` of
`[a·û, b·v̂]`. A frame rotation `δφ` costs about `a·δφ` there, while the
two loci differ only by about `(a − b)·δφ`. Two derivations of one
tilted section, whose major directions differ by rounding (about
`u/sin θ` at tilt `θ`), refuse `MeetFault::Unsupported("two ellipses in
one plane that are not one curve")` or escalate. Measured on main (two
designers, 2026-10-14): an ordinary ellipse re-derived from an axis one
ulp different refuses at r = 100, tilt 1e-4, ε = 1e-12, and escalates at
r = 1, tilt 1e-4 and at r = 100, tilt 1e-2.

The circle × ellipse arm (`Coplanar::EllipseCircle`) is sound: its
bound is a locus bound. `topo::boolean::carrier_cross::one_curve_gap`
already reads two ellipses by the least `‖L_e − L_f·Q‖` over the whole
orthogonal group (the Procrustes rotation), a locus bound.

Fix: read the ellipse pair's one-curve decision on a locus bound.
