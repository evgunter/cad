---
id: clearance-recomputes-a-cells-net-and-jet-per-pair
kind: issue
title: The clearance walk recomputes a cell's restricted net, its jets and its centre normal for every pair the cell is in; nothing is cached per cell
status: open
opened: 2026-11-01
priority: P3
cost: M
---

Found by DIRSUPPORT's review (2026-11-01), the review's Q6.

`crates/topo/src/clearance.rs` `support_separation` builds each cell's
`Hull` (`NurbsSurface::rect_net`, or a plane's jet) and evaluates
each cell's centre normal (`Surface::normal`, one jet) afresh for
every cell pair it is asked about. A cell sits in many pairs on a
walk's frontier, so the net and the jet are recomputed once per pair,
not once per cell. `Cell` is `Copy` and holds only its box (`enc`),
so there is nowhere to keep them.

Evidence: after DIRSUPPORT,
`editor-core` `m10_6_r1_probes_interval::a_planted_violated_reads_violated_over_a_certified_leaf`
ran at 1.0 s on the PR gate, the fast-set bar, and moved to the slow
set. Its share of the support's per-pair cost is not measured.

A per-cell cache (the restricted points and the centre normal, taken
when the cell is made, as `enc` is) would make the support's cost per
cell, not per pair.
