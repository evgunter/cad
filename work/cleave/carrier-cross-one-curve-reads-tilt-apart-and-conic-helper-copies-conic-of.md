---
id: carrier-cross-one-curve-reads-tilt-apart-and-conic-helper-copies-conic-of
kind: issue
title: carrier_cross's one-curve margin leans on the tilt row's separate Zero, and its conic helper re-spells Conic::of
status: closed
opened: 2026-10-12
priority: P3
cost: E
branch: cleave/carrier-cross-one-curve
closed: 2026-10-14
---

Found by the narrow re-review of PR 4782 (head d88f865dd1, verdict
MERGE); filed rather than held for, as style findings and notes.

## What

1. **Composition (likely).** `carrier_cross.rs` `one_curve`'s margin
   says it bounds the curves' distance "to first order", but it
   under-reads an out-of-plane tilt by up to √2 (ratio 1.41 for
   near-circles, 1.94 at a/b = 2). Its Zero therefore relies on the
   tilt row's own separate Zero: one decision inferred from two others
   (DESIGN.md Q1). Never measured past 1.5z in 1500 fuzz pairs per ε.
2. **A fresh copy (sure).** The new `conic(center, axis, major, minor,
   u_ref)` helper in `carrier_cross.rs` re-spells
   `geom_brep::Conic::of`'s Ellipse arm as a struct literal.
3. **Inside `carrier_speed.rs` (sure).** `speed_at` and `speed_hi`
   repeat the same Line and Circle arms, and the Circle arm duplicates
   `Conic::of`; `carrier_touch::speed_bound` now matches the carrier
   twice.
4. **Asymmetry (sure, harmless).** The turn term divides by e's
   (a + b) alone; swapping e and f moves the margin by up to 1.3e-2
   relative.
5. **Pins (sure).** The transverse ellipse × ellipse arm and the
   one-curve Positive branch have unit pins only.

## Fix

Fold the tilt into the one-curve margin so its Zero is one decision;
build the conic through `Conic::of`; one per-kind arm in
`carrier_speed.rs`; make the margin symmetric; add a body pin for the
transverse arm if a door reaches it.

## Built (branch cleave/carrier-cross-one-curve)

1. `one_curve`'s margin is now `|c_e − c_f| + ‖L_e − L_f·Q‖` (operator
   norm), `L = [a·û, b·v̂]`, `Q` turning `f`'s frame the way up `e`'s is
   and then by the best-matching (Procrustes) rotation. Pairing
   `c_e + L_e·x` with `c_f + L_f·Q·x` bounds the curves' Hausdorff
   distance outright, tilt out of the plane included, so its Zero is
   one decision. Order- and sign-free in the stored semi-axes (an
   orthogonal factor of `L`, absorbed by `Q`).
   Measured against a dense-sample Hausdorff distance (2048-point coarse
   search plus Newton per point, 512 samples plus golden refinement for
   the sup), a/b ∈ {1, 1.001, 1.1, 2, 10, 100}, b ∈ {0.01, 1, 10},
   perturbations of 0.2–5·ε in centre, semi-axes, in-plane turn, tilt
   and all at once, `f` stored in either order and either way up:
   margin / Hausdorff on main 0.24–0.94 for a tilt (the separate tilt
   row covered it), 1.2–4.0 for the semi-axes; now 1.00 for centre,
   semi-axes and tilt, 1.00–1.20 for a turn, 1.00–1.54 all at once, at
   ε = 1e-9 and 1e-6. At 1e-12 the sampler's own rounding on 1000 m
   ellipses reads isolated rows down to 0.72; the bound is an upper
   bound by construction, so no pose reads Zero past the zero threshold.
2. `meetings` builds its conics with `Conic::of`; the `conic` helper is
   gone.
3. `carrier_speed.rs` reads the carrier once (`Kind::of`, whose conic
   arm is `Conic::of`), and every bound is a `Kind` method;
   `carrier_touch::speed_bound` is `carrier_speed::speed_and_curvature_hi`.
4. Symmetric: the larger of the two orders is read.
5. Body pins (sweep `a_ruling_lying_on_a_wall`): the transverse
   ellipse × ellipse arm is reached by two oblique bottoms crossing on
   one wall, which build every op at their closed form; the one-curve
   Positive branch is reached by two rims read coplanar but k·ε apart
   at k = 30, 80, which refuse `CurvedPierceUnsupported` (filed:
   `two-rims-read-coplanar-refuse-unsupported-or-untyped`).

Fix pass (review of PR 5058): `parallel_circles`, the circle sibling,
now decides "one circle" with the same margin (`one_curve_gap`, row
`bool_carrier_cross_one_curve`) instead of three Zeros. The witness —
two of a sphere's circles turned 0.9·z about `x = 0.9` — escalates at
every ε (unit pin). No body door reaches it: the in-span row (one
ball) or the shared-rim rows (`rim_circle_axis_parallel`, a spun ball)
fire first. The gap is pinned bitwise symmetric, the k < 1 body rows
pin `FallbackExtentUnsupported`, and the frame is
`Conic::semi_axis_vectors`.

## Closed — PR 5058 merged; its margin moved into geom-brep (fuse/curve-pair-reader-c)

PR 5058 merged on 2026-10-14. Its one-curve margin, `one_curve_gap` with the Procrustes `frame_gap`, is now `geom_brep::curve_pair::pose`'s one-curve reading (`one_curve`, row `curve_pair_pose_one_curve`) for every conic pair in one plane. `carrier_cross::meetings` and its `one_curve` are deleted (stage (c) of `the-curve-pair-reader` (closed, PRs 5154, 5162 and 5227)). The unit pin `two_ellipses_are_one_curve_only_within_the_band_of_each_other` moved to `geom_brep::curve_pair::one_curve_rows` unchanged, bitwise symmetry included. The body pin `two_rims_the_band_reads_coplanar_are_one_curve_only_within_it` is re-baselined on that PR.
