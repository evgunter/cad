---
id: frame-invariant-vector-bound
kind: ruling
title: Should 'a certified bound on a vector residual is read from each coefficient's norm, never a per-coordinate box' bind future certificates, and in DESIGN.md D4 ¶2 or crates/geom-brep/README.md C2?
status: open
opened: 2026-10-01
needs_ev: true
priority: P1
---


The question rides with `a-rigid-map-re-derives-the-plane-nurbs-edge-certificate-in-a-frame-that-moves-it`.
That row's fix needs no ruling, because both designers agree on it and
no ratified text changes: `SurfaceResidual::sup_bound` reads each vector
Bernstein coefficient's norm. What waits on Ev is only whether the rule
behind the fix binds future certificates, and where it is written.
