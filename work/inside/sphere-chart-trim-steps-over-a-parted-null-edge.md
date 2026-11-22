---
id: sphere-chart-trim-steps-over-a-parted-null-edge
kind: issue
title: sphere_chart_trim steps over every null edge, a parted one included, where the region gather skips only a placeholder
status: open
opened: 2026-11-20
priority: P3
cost: E
refs: [sphere-region-refuses-null-scaffolding-the-torus-and-ruled-doors-step-over]
---


## What

`sphere_chart_trim` (`crates/topo/src/solid_contain.rs`) walks a sphere
face's outer loop and, at an edge with no certified curve, `continue`s
("Null scaffolding: no carrier, so no class claim"). It does not ask
whether the edge is a placeholder (`topo::null::is_placeholder`: a null
edge whose two ends are one point). A null edge whose ends have parted
is a gap in the boundary, not scaffolding, and the trim steps over it
too: the face is read into the chart-rectangle class with the gap's
course unread, both ends' latitudes folded in as vertices.

Every other boundary reader of a face draws that line at the one
predicate: `region::boundary` (`crates/topo/src/region.rs`) skips a
placeholder under `Scaffolding::Skipped` and answers `None` for any other
uncertified edge, and so does the loop lift past placeholders
(`pcurves/lift.rs`, `Lift::read_with`).

The trim's readers are `cover.rs`'s sphere-face reach bound
(`sphere_window`) and the
chord-join island rows.

## Why it matters

A latent soundness gap (P3): a face whose boundary has a gap gets a
latitude/azimuth window it may not lie in. Parted null edges are
transient (tier 2 refuses null scaffolding at rest), so this is reachable
only on an operator's working copy.

## Owed

Step over a placeholder only: at an uncertified edge,
`if !crate::null::is_placeholder(body, he_data.edge) { return Ok(None) }`
before the `continue`, with a row: the trim of a face with a null edge
fanned in (`region::with_null_edge`) reads the plain face's window, and
the same edge with its far end parted answers `None` (the pattern of
`torus_region.rs`'s `a_walls_null_edge_bounds_nothing`).

Found by the sweep of `charttail/sphere-scaffolding`.
