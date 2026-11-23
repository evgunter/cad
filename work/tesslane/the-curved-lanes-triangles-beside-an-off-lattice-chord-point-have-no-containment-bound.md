---
id: the-curved-lanes-triangles-beside-an-off-lattice-chord-point-have-no-containment-bound
kind: issue
title: the curved lane's triangles beside a chord point its grid did not place have no containment bound; measured inside one lattice cell, not proved
status: open
opened: 2026-11-23
priority: P2
cost: M
---


Found by `a-filleted-one-segment-hole-rims-band-exceeds-its-mesh-certificate` (branch `tesslane/one-slit-band`). The same missing lemma as `one-containment-lemma-for-an-oblique-boundary`, seen from the curved lane, whose boundary runs lie on iso lines.

## What

The curved lane (`crates/mesh/src/curved.rs`, `tessellate_curved`) sizes at δ with no headroom on its theorem about lattice half cells (`[mesh.sizing]`). Its boundary runs are iso lines, but their chord points need not be lattice nodes: a meridian's points can sit between two rows and a rim's between two columns. Sources measured on main:

- the capped step's ceiling edge on a quarter arc, in the edge's count (3 chords, 2 rows) or in the face's (2 chords, 3 rows) (`a-quarter-arcs-chord-count-sits-on-the-capped-steps-ceiling-edge`);
- a rim carried by several edges, each chorded on its own, so the side holds more chords than the face has columns (a hole of two arcs, filleted: 60 against 59, 104 against 103).

A triangle beside such a point is no half cell, and nothing bounds it in closed form. The certificate holds it.

## What is measured

The curved lane's CDT runs in the lattice metric (`curved`'s module docs, "The lattice metric"). Over 498 one-slit fillet bands on main (`crates/mesh/tests/the_one_slit_band.rs`'s bodies: the revolved section and the fillet door's plate with holes of 2, 3 and 5 arcs; hole radius 0.05, 0.3, 0.9; fillet radius 5e-4 to 5e-2; δ 3e-2 to 1e-4), with 104 bands holding a slit point between rows:

- every band triangle spans at most one lattice column in `u` and at most one and a half row gaps in `v` (the largest readings: `1.0000000000000981` columns, `1.5000000000001397` rows);
- every band triangle lies within 0.89 δ of the torus by sampling;
- none refuses. In raw UV, as the lane ran before, 42 of them refused `CertificateExceeded`, up to 83 times δ.

On PROFILE's branch (`profile/circle-lowers-to-one-segment`, its `edit_bore_two_rims` plate, 161 cells), the worst certificate over triangles with a corner on the slit between rows equals the worst over the lattice's own triangles to within 2 ulp in every cell that has such triangles: the triangles beside the off-row points tie the half cells and never exceed them.

So one and a half rows is reached, not one: the triangles beside an off-row slit point leave their cell in `v`, and the torus's `v` term carries the slack (`cert::cert_torus` reads the triangle's parameter box).

## What closing it takes

The lemma of `one-containment-lemma-for-an-oblique-boundary`, for an iso run whose points are not all lattice nodes: a bound on how far past its cell a constrained Delaunay triangle beside such a point reaches, in the lattice metric, and the curved lane's step solved against it. No headroom factor is to stand in for it.
