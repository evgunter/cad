---
id: spline-trims-chain-spells-one-rows-direction-twice
kind: issue
title: spline_trim's chain reads one row's direction twice: LiftedRow::plus for its ends and walks_forward for its pieces
status: open
opened: 2026-11-23
priority: P3
cost: E
---


Found by the review of BOXFINISH's iso-image unit (PR 7393, 2026-11-23).

`crates/topo/src/spline_trim.rs`, the chain builder (about line 885):
an iso row's two ends are put in its edge's forward order by
`LiftedRow::plus` (`if row.plus { [entry, exit] } else { [exit, entry] }`),
its pieces are built forward by `image_pieces`, and the run is then
reversed where `!walks_forward(body, row.half_edge)`
(`he_plus == he`). `plus` is `pcurves::is_plus(body, half_edge)`. Both
ask whether the chain walks the edge forward, through two functions.

Not checked: that the two agree on every half-edge (a closed edge whose
two half-edges are one; a seam). If they are one fact, the chain should
read it once, from the row; if they can differ, the site owes the case
where they do.
