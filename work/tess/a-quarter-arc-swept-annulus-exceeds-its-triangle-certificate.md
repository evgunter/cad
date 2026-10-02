---
id: a-quarter-arc-swept-annulus-exceeds-its-triangle-certificate
kind: issue
title: a sweep_body loop of circle_split(4) sections passes tier 3 and refuses tessellation CertificateExceeded at delta 1e-2
status: open
opened: 2026-10-02
---

Found by SHOW's `klein-scene-should-adopt-the-one-body-loop-sweep` unit
(2026-10-02). Pinned live as the Klein bottle's wall 8
(`demos/tour/src/klein.rs`, `wall_probes`, built by `one_body_loop`
with `annulus(true, ..)`).

## What

The Klein bottle's top loop as ONE body: an annulus whose two walls
are `circle_split(centre, r, 4, 0.0)` (r = 0.275 and 0.225 m — four
quarter arcs each, the door
`lofted-circle-sections-are-unmeshable-and-say-so-three-steps-late`
names for a lofted round section) swept by `sweep::sweep_body`
(33 stations, v-degree 3) along a degree-3 interpolant through 49
exact points of two tangent arcs of radius 1.2 m (270° then 90°),
starting at (0, 0, 3) from `path_start_frame`. It passes tier 3
(`validate_geometric`; the volume's sign decides, the reporting
number is a bracket [0.416, 0.769] m³ around the Pappus 0.592 m³).
`mesh::tessellate(&body, 1e-2, Tol::witness())` refuses:

```
CertificateExceeded { face: FaceKey(7v1), bound: 0.012427265077752946, requested: 0.01 }
```

With eight arcs per wall (`circle_split(.., 8, ..)`) it refuses too,
on another face and further over: `FaceKey(3v3)`, bound 0.0302.

`TessellateError::CertificateExceeded`'s doc reads it as a kernel-side
defect or degenerate geometry. The loft walls here are neither thin
nor degenerate: 0.05 m section wall, spine radius 1.2 m, curvature
sign flipping once at the arcs' tangent joint (C1, not C2, in the
sampled points; the interpolant smooths it). Untraced: whether this is
`rim-chords-exceed-snapped-column-count`'s configuration (a rim whose
curve bound exceeds the snapped column count beside a malign band),
which that row recorded as reachable but never observed.

## Done when

The bottle's wall 8 stops refusing: the quartered one-body loop meshes
at δ = 1e-2.
