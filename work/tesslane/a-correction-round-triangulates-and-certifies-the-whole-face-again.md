---
id: a-correction-round-triangulates-and-certifies-the-whole-face-again
kind: unit
title: A correction round triangulates and certifies the whole face again; inserting into the standing triangulation and re-certifying only the changed triangles would make a round cheap
status: open
opened: 2026-11-23
priority: P3
cost: M
---


Filed by `a-refused-certificate-adds-a-point-and-no-face-sizes-below-delta` (stage 1, 2026-11-23), on the orchestrator's ruling.

## What it is

A face whose triangle reads over δ adds a point and is triangulated again (`crates/mesh/README.md` `[mesh.sizing]`; `trimmed::TrimmedFace::attempt`, `curved::tessellate_curved`). Every round rebuilds the constrained triangulation from all the face's points and re-reads every triangle's certificate. On the 80 of 3,238 chart-lane analytic faces in `mesh` and `sweep`'s meshing modules (ε `10⁻⁹`) that take a round, 55 cost more time than the face did at the old δ/2 sizing, and 53 of those end coarser than it: median 1.19 times, worst 2.6 times (a wide tilted cylinder, 3 rounds, 0.77 times the triangles). Summed over the 80: 459 ms to 523 ms (release, one thread, nothing else running).

## What a round costs, measured

One pass, split (dev profile, the `mesh` tests that build chart-lane faces):

| chart | triangulation and constraints | emit and certificates |
|---|---|---|
| sphere | 57% | 43% |
| torus | 56% | 44% |
| cylinder | 48% | 52% |
| cone | 24% | 76% |
| spline | 62% | 38% |

## What would make it cheap

- Insert the round's points into the standing triangulation instead of rebuilding it: about half a round, by the split above.
- Re-certify only the triangles the insertion changed: the rest.

Both keep the mesh a function of (body, δ): which triangles are over δ is the same, and the points are the same; only the work to reach them changes. The determinism the lane relies on (the patch's interior numbering by coordinates, `TrimmedFace::emit`) has to be re-derived for an incremental insertion, whose triangulation need not equal a fresh one's on cocircular points.

## Done when

A round costs the work of its points, measured on the same faces, and no face's bytes move except where cocircular ties move and the PR shows the tie.
