---
id: five-more-injectivity-margins-are-levered-at-a-cell-of-the-sweep
kind: issue
title: Five more margins of the injectivity check multiply an angle or a sign by a cell's own extent, as the departure did: the column, the corner's turn, the graph's advance, the cusp and the monotone arm
status: open
opened: 2026-11-22
priority: P1
cost: H
---



## Finding

From EMBED's lane `embed/remeter` (2026-11-22). Ev's ruling on PR 7095
re-metered the departure reading: an angle is metered at the reach of
the faces it governs, never at a cell of the question's own subdivision
(`docs/DESIGN.md` `[topology.tiers]`). The lane read every funnel
decision in `crates/topo/src/injective.rs` (`decide(`,
`interval_sign_as(`, `is_zero(`; 26 sites) for the same shape and
re-metered none but the departure. Five sites have it:

| site | function | what it decides | what it multiplies by |
|---|---|---|---|
| `self_intersection_column` | `monotone_run` | a curved edge's chart curve advances one way along a chart axis over each piece | the piece's parameter width and the cell's chart speed |
| `self_intersection_corner_departs` | `leaves_graph` | a corner's other edge leaves the curve read through the vertex toward its face side | the columns' parameter span and the rectangle's chart speed |
| `self_intersection_corner_departs` | `leaves_line` | the other edge leaves the line it meets at the vertex | the piece's width and the chart speed |
| `self_intersection_graph` | `graph_of` | the shared edge advances along the projection's first axis | the piece's parameter span |
| `self_intersection_cusp` | `cusp` | the two faces' outward normals are opposed (wedge 0 or 2π), decided `Zero` | the cell ball's reach |
| `self_intersection_monotone` | `monotone` | the least eigenvalue of `sym(L·J)` is positive | the cell's spans, as `J`'s columns |

A cell thinner than the band cannot pass any of the first four or the
last, whatever the geometry does, and a smaller cell makes it worse.
The cusp is the reverse: once the cell's ball is small enough, any two
normals read `Zero`.

**Whether a margin is owed at all**, by Ev's test on PR 7095 ("the
margin is needed for things that must tolerate the geometry being ε
off"):
- `_column` and `_graph` want a sign: that a curve is a graph over an
  axis, or an edge over a direction. `monotone_run`'s own doc says a
  sign of the advance is a sign of it times any positive lever. They
  may owe no margin, only a sign that is not rounding's.
- `_corner_departs` is a corner's turn and `_monotone` a face's own
  turning: angles, which owe a margin at the reach of what they govern
  (the two edges at the corner; the face).
- `_cusp` is tier 3's fact, `WedgeVerdict::End`, which the program page
  already has the cusp site reading off tier 3's verdict.

## Measured

Counters on `embed/remeter-measure` at `a246952d3` (the gate harness
with `embed/remeter` merged in), read with `SD_LOG`: per site, every
read, the reads that did not decide, and of those the reads whose rate
before the cell's factor is strictly signed. "Signed" is an enclosure
that excludes zero, one signed only at rounding size included. For the
monotone arm the rate is the same eigenvalue with each column a unit at
the cell's centre. For the cusp the second column is `Zero` reads and
the third those whose normals' sum is longer than 1e-3. Default ε.

**The tour walk** (85 stops, one process):

| site | reads | undecided | undecided on a signed rate |
|---|---:|---:|---:|
| column | 151,386 | 7,516 | 2,854 |
| corner departs, `leaves_graph` | 158 | 35 | 0 |
| corner departs, `leaves_line` | 10,819 | 4,400 | 925 |
| graph | 79,053 | 8,279 | 2,269 |
| cusp (`Zero`; off-opposed) | 67,045 | 14 | 0 |
| monotone | 94,845 | 70,153 | 18,394 |

**The suites** (`self_intersection_proves_injectivity`,
`shell_cavity_embedded`, `a_seamless_pipe_cut…`, `sf2b_interval_probe`,
`torax_interval`, `clearance::wiring_rows`; 79 tests, 68 processes that
ran a sweep):

| site | reads | undecided | undecided on a signed rate |
|---|---:|---:|---:|
| column | 291,923 | 13,084 | 6,358 |
| corner departs, `leaves_graph` | 286 | 0 | 0 |
| corner departs, `leaves_line` | 49,293 | 5,400 | 3,784 |
| graph | 21,276 | 1,804 | 95 |
| cusp (`Zero`; off-opposed) | 18,597 | 362 | 0 |
| monotone | 5,845 | 3,841 | 2,500 |

**275 census tests through the at-rest gate** (every test of
`measure/gate/census-c2a0c41f40.json` that refused
`Uncertified/Departure/Edge` between plane and plane or plane and
cylinder, in topo, sweep and editor-core, that exists today):

| site | reads | undecided | undecided on a signed rate |
|---|---:|---:|---:|
| column | 12,797,970 | 238,435 | 63,848 |
| corner departs, `leaves_graph` | 66,042 | 25,277 | 528 |
| corner departs, `leaves_line` | 1,451,688 | 745,450 | 275,526 |
| graph | 778,404 | 33,466 | 4,817 |
| cusp (`Zero`; off-opposed) | 821,969 | 29 | 0 |
| monotone | 6,712 | 4,700 | 2,720 |

What the counts do not say: whether a pair refused for one of these
reads. An undecided read splits the cell, and most pairs certify at a
smaller one. The monotone arm's are the largest share on the tour
(18,394 of its 94,845 reads). `leaves_line` is the largest among the
census tests, 275,526 reads on a signed rate, in bodies that refuse at
a shared edge today.

**The cusp read no pair of normals that were not opposed** in any of
the three populations: its `Zero` reads all had a sum shorter than
1e-3. The defect is in its form and unmet here.

## What is needed

Each site read against Ev's test, with the designers where the answer
changes what a margin requires or at what scale:
- the two signs (`_column`, `_graph`): whether a banded sign at a lever
  that is not the cell, or no band;
- the two angles (`_corner_departs`, `_monotone`): the lever each is
  metered at;
- the cusp: read off tier 3's verdict, as the program page has it.

`embed/tangent-corner` met `_corner_departs` from the other side, a
tangent joint's rate straddling zero.
