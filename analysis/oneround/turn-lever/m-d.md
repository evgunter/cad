# M-d: the bound from below with each boundary edge's own reach added

## What was computed

`below_d` is the door's bound from below (the faces' vertices and the edge's
ends from the span's middle) maxed with:
- every boundary edge's `cover::edge_reach_bounds(…).below()` from the same
  point. It is exact on a line and a circular arc, and sampled at 17 points
  otherwise; each sample is a point of the edge.
- the door's own edge, through `geom_brep::edge_reach_bounds`.

It was read at every door call, timed with a wall clock per call, and every
composite was run a third time at it (M-b's harness). Coverage is as in M-b.

## The ratio of the bound from above to each bound from below

At 1e-9, f64, 19.56 M door reads:

| above / below | vertices only | with the edges' reach |
|---|---|---|
| < 1 | 0 | 29,468 |
| = 1 | 14,469,073 | 16,676,782 |
| (1, 1.01] | 297,173 | 222,699 |
| (1.01, 1.1] | 713,719 | 346,658 |
| (1.1, 2] | 2,183,612 | 783,516 |
| (2, 10] | 457,602 | 129,248 |
| (10, 100] | 74,361 | 14,334 |
| (100, 1e3] | 19,136 | 14,954 |
| > 1e3 | 11,075 | 8,092 |
| poison above | 1,334,422 | 1,334,422 |

1e-12 (7.03 M reads) and 1e-6 (11.1 M reads) have the same shape.

The "< 1" row means an edge's sampled reach (a point of the edge) stands
farther from the pivot than the bound from above, which holds the faces'
covers, the edge's own cover and the points. So either a face's cover misses a
point of its own boundary edge, or the edge is not on the face it bounds. That
is M-c's question, read with real points; M-c's rerun attributes it by door.

## Straddles cured

**None, because none existed.** M-b found no finite straddle at any ε. With
the edges added, the pairs at `below_d` against the bound from above differ
only where the bound from above is poison: 36,826 at 1e-9, as at the vertices.

**Newly served definite verdicts:** none. `below_d` agrees with the bound from
above wherever the vertices did.

**On M-e's truth subpopulation** (plane-plane line edges, straight-edged
faces), `below_d` equals the vertices' bound, since each edge's reach from a
point is its farther end. So there is nothing on that subpopulation to check
against the truth.

## What it costs

These are the edge reads added per door call, wall clock on a loaded 4-core
box, summed over every row. The door's own cost without them was not timed.

| Lane | calls | added per call |
|---|---|---|
| f64 | 37.7 M | 45 µs |
| Dual | 303 k | 89 µs |
| Interval | 605 k | 0.70 ms |
| Sym⟨Interval⟩ | 138 k | 1.90 ms |
| Sym⟨f64⟩ | 68.9 k | 4.08 ms |

The symbolic lanes pay 40 to 90 times what f64 pays: each sampled carrier
point is a trigonometric form. That is the cost the doors' docs give for
reading vertices only. At 1e-9 the Sym lanes' total is about 544 s, spread
over the run.
