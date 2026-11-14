---
id: spiric-extent-overshoots-the-sheet-slack-at-interval
kind: issue
title: At Interval a spiric wall row widens a torus face's extent past the sheet slack, so a cell ending on a parallel that lies on the window's bisection grid is never read: the grid-aligned torus cut off its axis exhausts the depth budget
status: open
opened: 2026-10-31
priority: P2
cost: M
---



## Finding

Found by SPIRIC (`injectivity-reads-no-spiric-edge-on-a-torus`) while
building the spec's directly built spiric section.

**Reproducer.** The ring torus `(2, 0.5)` about `y`, its profile's two
vertices at `(2, ±0.5)` (so its parallels lie at `v = ±π/2`), less
`slab(0.3)` turned a quarter about `y`: the fixture
`torus_cut_off_its_axis` in
`crates/sweep/tests/self_intersection_proves_injectivity.rs` with those
two vertices.
- `Body<f64>`: `Holds` (20 candidates, 219 certified, `cornered` 28).
- `Body<Interval>`: `Refused(Budget(Depth { max_cell_depth: 40 }))`,
  40 pairs refused, all non-touching pairs of two torus quadrants
  (`3v1`, `5v1`, or `3v1`, `4v1`) beside the vertex where the meridian
  `u = 0` meets the parallel `v = 3π/2`.

The row as committed puts the vertices `0.3` rad round the tube instead,
off the window's bisections, and holds at both scalars.

**Cause.**
- At `Interval` the spiric edge's parameter span is about 25 ulps wide
  (`t1 ∈ [6.283185307179575, 6.283185307179598]`).
- A wall row enters `Boundary::extent` by its enclosure over that span
  (`clearance::row_of`), so the face's extent overshoots its parallel
  by about `2e-14`.
- `injective::sheets` grants `few_ulps(τ) ≈ 1.1e-14` of slack. A cell
  of the next quadrant that ends exactly on the parallel (the torus
  window's bisections hit `3π/2`) therefore meets the extent's copy.
- `present_chart` then reads the parallel as outside (the touch) and
  the meridian as inside, and returns `Local`. The touching pair splits
  into non-touching children that `cell_region` cannot decide either,
  because a cell ending on the boundary is never decided outside. Those
  children split to the depth floor.

The answer is a typed refusal, never a wrong one. Before SPIRIC such a
face had no extent and refused `Departure` at the root.

## The design problem

The extent is honest: the edge's span really is that wide at
`Interval`. Three ways out, none argued yet:
- The sheet slack scales with the extent's own width rather than with
  `τ`'s rounding. That is a wider ε concession, which needs a verdict.
- `present_chart` reads a cell that lies past one line on its non-face
  side and on the face side of the other line of a convex corner as
  vacuous. That is a new corner argument.
- A wall row's `v` ends are read off its neighbours' ends (its `v` is
  monotone). This is tolerance-dependent, which is also an ε.

**The line-corner sliver absorbs the same overshoot.** On the klein
elbow's cavity at `Interval`, the same wide extent clips a line-corner
cell beside a torus half's parallel to a zero-width rectangle.
`present_line_corner`'s sliver (SPIRIC, #6159) reads that rectangle
vacuous; without it the cavity refuses `Departure` at the parallel. A fix
here changes what reaches that sliver, so re-check that site and its row
`a_line_corner_cell_reaching_the_line_from_outside_reads_vacuous` when
this row is fixed.
