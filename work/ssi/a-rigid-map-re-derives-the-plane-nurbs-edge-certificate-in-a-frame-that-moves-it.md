---
id: a-rigid-map-re-derives-the-plane-nurbs-edge-certificate-in-a-frame-that-moves-it
kind: issue
title: transform_rigid_via re-derives the plane x NURBS edge certificate in the new frame, and its componentwise box norms move under rotation, so an edge certified near its band can refuse after a rigid map
status: open
opened: 2026-09-28
priority: P3
cost: M
---

## Found (ENCL rigid-map headroom lane, branch `encl/rigid-map-approx-headroom`, 2026-09-28)

The sweep for siblings of
`work/encl/a-rotation-can-refuse-an-approx-face-that-certifies-near-eps.md`
— a rigid map that re-derives a hull-assembled bound in the new frame
and refuses on it — met this one. **Inferred from the construction,
not reproduced.**

`topo::transform_rigid_via` re-certifies every mapped edge carrier
through `EdgeCurve::certify_via` (`crates/topo/src/transform.rs`,
`transform_rigid_via`). An `Intersection` edge between a plane and a
described NURBS wall certifies through the injected lane,
`geom_brep::plane_nurbs_limbs` (`crates/geom-brep/src/edge_nurbs.rs`),
which delegates to the rung-3 SSI certificate. Parts of that
certificate read a vector's size off its componentwise enclosure box —
`EncloseBox::speed_sup` (`crates/geom-brep/src/ssi/enclose.rs`), the
chart-floor rate, the tube pad, the transverse stretch — and a box norm
of the same vector set can differ by up to √3 between two frames. The
edge's `hull_sup` limb is a Bernstein composite over `S(P(t)) − C(t)`,
which is at least as frame-sensitive in its enclosure width (the
`Approx` sibling's trace shows the width, not the norm, moving).

The transform module's own docs claim the opposite: "a rigid map
preserves every distance-valued residual up to rounding, so
re-certification of a valid body succeeds". That holds for sampled
residuals and implicit-form scalars, not for a bound assembled from
ambient-frame hulls.

## What is open

1. Reproduce: a plane × NURBS edge certified within a few percent of
   its band, moved by an oblique rotation through
   `transform_rigid_via` with the lane injected. The `Approx` row's
   scale-to-land-near-ε construction
   (`crates/topo/tests/rigid_map_near_eps_approx.rs`) is the template.
2. If it reproduces, the remedy is this lane's to choose. The `Approx`
   face could be re-fitted because its fit is derived from a
   description; an edge's declared carrier is not re-derivable the same
   way, so re-minting is not obviously available here.
