---
id: zero-of-reads-local-where-a-shared-vertex-is-the-corner-of-no-one-clip-pair
kind: issue
title: zero_of reads Local where the cell holds a shared vertex that no zero side holds and that is the corner of no one pair of its clips, so a torus beside a coaxial seam refuses Departure
status: open
opened: 2026-11-21
priority: P0
cost: M
---


## Finding

From EMBED's per-solid census trace (lane `embed/trace-cone-torus`,
2026-11-21), main `c2a0c41f40` with the gate and its trace switches
(`embed/trace-cone-torus-measure`).

`zero_of` (`crates/topo/src/injective.rs`), in the arm with no zero
side and one loose shared vertex, counts the corners of two clips on
different axes holding the vertex and refuses `Local` unless there is
exactly one (`match corners.as_slice()`, `_ => Local`).

One traced row ends there at depth 40,
`germ_circle_torus::a_coaxial_seam_on_the_torus_is_placed_by_its_arc`:
torus `3v1` against torus `4v3` across `11v3`. `4v3`'s cell at
`u ∈ (π/4, π/4 + 1.5e-3)`, `v ∈ (5.9518, 5.9641)`, holding the shared
vertex `8v1`, is `Local` at that arm; `3v1` is presented by a side
(`Zero::Side(YHi)`).

**Unchecked:** whether the count is zero (the shared edge is not a clip
of the cell, so its end is no corner) or two. The diagnostic print sits
on the other arm. A cell that holds a shared vertex the shared edge ends
at, with that edge not a clip, is the shape to test first.

## Count

1 test (torus/torus). A valid body, built at its closed form.

**Reproducer:** `sweep::all germ_circle_torus::a_coaxial_seam_on_the_torus_is_placed_by_its_arc`
(1.3 s).
