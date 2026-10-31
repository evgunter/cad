---
id: a-mesh-diagonal-flips-on-last-bit-uv-noise
kind: issue
title: A curved patch's quad diagonal flips on last-bit UV noise, so mesh goldens move with positions identical
status: open
opened: 2026-10-24
---


Found by the review of PR 5659 (`work/chart/chart-inversion-is-spelled-three-times.md`), which moved eight `crates/mesh/tests/d9_mesh_goldens.rs` digests: ball, sphere_wedge, cone and donut, each at both δ.

## What

PR 5659 changed how a curved face's chart coordinates are read: geom's one reading, in C4's windows, with `Real`'s trig instead of `std`'s. The UV values the walk builds moved in their last bits. The review measured the moved meshes against the old ones:

- the vertex positions are identical to ≤ 3e-16;
- the quality metrics are identical;
- what moved is a **quad-diagonal tie-break**: which diagonal a grid quad is split along, decided on UV values that differ only in their last bits.

So the triangulation is a function of last-bit UV noise rather than of the geometry. Any change that moves those bits (a trig implementation, a window, an association order) moves the goldens while the mesh is unchanged in every sense a consumer can see. The δ = 0.05 cone's mesh holds a 0.0° triangle; the review found it in the meshes before and after the change alike, so it is not new with PR 5659.

## Owed

Decide each quad's diagonal on a margin that is not last-bit noise: an explicit tie-break when the two diagonals are within the band of each other, read in metres. Look at the δ = 0.05 cone's 0.0° triangle while you are there. Re-baseline what moves, and say what moved.

## Moved to TESSWALK (what the walk and the lanes decide: structural premises, and readings taken off undecided numbers) (2026-10-28)

Cut out of TESS by its semantic-seam split (`work/README.md`, Track size) when TESS measured 146 budget points against 30. Moved by `git mv`; the id and body are unchanged.
