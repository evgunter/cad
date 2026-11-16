---
id: props-and-tier-3-each-derive-a-loops-chart-position-again
kind: issue
title: props' rotational lane and tier 3's side reading each derive a loop's chart position again, beside the lift
status: open
opened: 2026-11-12
---


Found by `onedoor/three-walks` (2026-11-12), sweeping for readers that
re-derive a loop's position in its face's chart after `topo::pcurves`'
`Lift` became the one reading of it (DESIGN `[one-way.places]`, "A
loop's position in its face's chart").

Two readings in `geom-brep` walk a face's loop edges on their carriers
and place each on a chart branch by their own rule, with no `Body` and
so without the stored joint elements:

- `geom_brep::props::quad::rotational` (`loop_windings`, `loop_lifts`,
  `edge_branch`): "the chain starts on the first edge's own lift; past
  each edge it gains the edge's branch turns, and past each junction
  the whole turns the closure took off its level gap".
- `geom_brep::chart_singularity::walk_loop` (read by `loop_holds_on_left`
  and `nappe`, tier 3's and STEP import's side reading): the loop's
  winding about the surface axis and the signed area of its chart
  image, from samples of each carrier.

Neither was moved by the unit: both sit below `topo`, take `LoopEdge`s
and not a body, and the second runs at import, before any row is
minted. The question for this slate is whether the rotational lane's
`topo` caller (`props::quad_lane`, which already reads
`pcurves::lifted_images`) can hand the lift's decks down in place of
`loop_lifts`' own count, so the quadrature and the stored lift cannot
place one loop on two branches.
