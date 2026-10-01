---
id: frame-invariant-vector-bound
kind: ruling
title: Should 'a certified bound on a vector residual is read from each coefficient's norm, never a per-coordinate box' bind future certificates, and in DESIGN.md D4 ¶2 or crates/geom-brep/README.md C2?
status: closed
opened: 2026-10-01
priority: P1
closed: 2026-10-01
---


The question rides with `a-rigid-map-re-derives-the-plane-nurbs-edge-certificate-in-a-frame-that-moves-it`.
That row's fix needs no ruling, because both designers agree on it and
no ratified text changes: `SurfaceResidual::sup_bound` reads each vector
Bernstein coefficient's norm. What waits on Ev is only whether the rule
behind the fix binds future certificates, and where it is written.

## Closed (2026-10-01)

Ev: "lgtm!" on `[ev]` PR 3677. The sentence lands in `docs/DESIGN.md` D4 ¶2 as written: a certified upper bound on a vector-valued residual is read from the Euclidean norm of each coefficient, never from a per-coordinate box folded into a norm.
