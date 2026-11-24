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

The curved lane's CDT runs in its lattice's metric (`curved`'s module docs, "The lattice metric"). Two corpora, each named with its figures:

- **This unit's sweep** (498 bands on main: the revolved section and the fillet door's plate with holes of 2, 3 and 5 arcs; hole radius 0.05, 0.3, 0.9; fillet radius 5e-4 to 5e-2; δ 3e-2 to 1e-4; 104 bands with a meridian point between rows): none refuses, where raw UV refused 42, up to 83 δ. Every band triangle samples within 0.89 δ of the torus.
- **The review's sweep** (720 bands): raw UV refuses 32, up to 281 δ; none refuses in the metric. The worst certificate is 0.99 δ and the worst sampled deviation 0.927 δ.

Across both, a triangle spans at most one lattice column, and at most `max(1, nv/m)` row gaps, `nv` the band's rows and `m` its meridian's chords. The review checked that rule on a synthetic CDT over m from 1 to 10, nv from 1 to 8 and residual aspect in [1/√2, √2]. The 1.5 rows of this unit's sweep is that bound for a slit of 2 chords on 3 rows: a meridian chord itself spans `nv/m` gaps, so it is a side of the boundary, not a reach past a cell. `crates/mesh/tests/the_one_slit_band.rs` pins the rule, read from each band's own counts.

On PROFILE's branch (`profile/circle-lowers-to-one-segment`, its `edit_bore_two_rims` plate, 161 cells), the worst certificate over triangles beside the off-row points ties the worst lattice half cell to within 2 ulp in every cell that has such triangles.

## What closing it takes

The lemma of `one-containment-lemma-for-an-oblique-boundary`, for an iso run whose points are not all lattice nodes: a bound on how far a constrained Delaunay triangle beside such a point reaches, in the lattice metric (the rule above, proved), and the curved lane's step solved against it. Where `m < nv`, a triangle on a meridian chord spans more than one row, which the half-cell theorem does not cover: the bound has to price that. No headroom factor is to stand in for it.
