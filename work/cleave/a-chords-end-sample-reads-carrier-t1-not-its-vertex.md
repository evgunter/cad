---
id: a-chords-end-sample-reads-carrier-t1-not-its-vertex
kind: issue
title: Certification reads an edge's end sample at carrier(t1), one rounding off its vertex, so at the band's edge its verdict depends on the edge's orientation
status: dispatched
opened: 2026-10-11
priority: P2
cost: M
branch: cleave/chord-end-sample
---


## What

Found by the review of PR 4625 (`cleave/folded-graze-join-desync`).
`geom-brep/src/certify.rs` `certify`'s check 4 (the per-sample
`Intersection` residuals, `let p = spec.carrier.eval(t)` near line
2647) reads every sample, the two ends included, through the carrier.
For a `line_between` chord (`EdgeCurveSpec::line_between`, ~1035) the
start sample is `origin + 0·dir`, the start vertex bit for bit, and the
end sample is `origin + len·(p1 − p0)/len`, which lands up to one
rounding off the end vertex.

Witness: `sweep` `near_tangent_pierce_poses`' `EDGE_ON_FACES`, e.g.
`Ltop nt e2 a13 d-1e-9` at ε = 1e-9. The far vertex of the prism's
vertical edge lies 9.999999994736444e-10 m off the cube's near face
(exact over the stored `f64`s: 5e-19 inside the zero band's edge). The
section chord pierce → vertex evaluates the vertex as
(1, 0.9999999999999999, 0), whose residual is 1.0000000518091955e-9:
just in band, so the run refuses `Escalated { EdgeOnFaces }`. The
chord vertex → pierce, in the other three runs of the pose, reads the
vertex as its origin, in the zero band, and builds. One input, two
verdicts, by the edge's orientation.

## Owed

(Q1) Re-pose the end samples at the edge's vertices, the points the
endpoint checks (`EndpointStart`, `EndpointEnd`) already pin the
carrier to, so check 4 reads the same point at either orientation; or
argue that the carrier, not the vertex, is what check 4 certifies and
that the orientation dependence at the band's edge is admissible.
Either way the `EDGE_ON_FACES` pin's run list moves with it.
