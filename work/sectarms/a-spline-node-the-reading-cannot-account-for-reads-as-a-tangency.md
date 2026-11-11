---
id: a-spline-node-the-reading-cannot-account-for-reads-as-a-tangency
kind: issue
title: a spline × implicit node whose lines are near each other is not listed: the accounting runs out of cells, or the sweep declines
status: open
priority: P2
cost: M
opened: 2026-11-11
---


Found in the NURBS × implicit stage 3b review fix pass (PR 6811).

## What happens

A node is listed with its lines, side and four arcs when its lines are
far enough apart in the wall's chart. Each arc's spans are graded toward
the node (`Ends::graded`, `crates/geom-brep/src/ssi/ends.rs`) so limb 3's
tube encloses the transverse gradient without straddling. The node rows
(`crates/geom-brep/src/pair_section/spline_implicit/node_rows.rs`) list
nodes with lines at ±1.5 slope and at 1.5 and −0.4, and the cross
saddle over a sheared chart, against a plane and a sphere of radius 100,
at ε 1e-9, 1e-6 and 1e-12.

As the lines close up, two other limits bind first:

- **Lines 45° apart** (`4Y(Y − X)` against `z = 0`): the sweep decides
  the node and the arcs certify, but the chart accounting
  (`exhaust::account_chart_partner`) runs out of its 200000 cells. That
  ground is the narrow sectors beside the node, where `φ` is small over
  cells that are neither in a tube nor in the node's box. The reading
  refuses `Pinched { predicate: "spline_critical_unread" }`
  (`read_crossing`), which topo reads as a tangency, as main read the
  node's pinch. The row
  `a_node_the_reading_cannot_account_for_reads_as_a_tangency` pins it.
- **Lines 40° apart or nearer:** the regularity sweep declines `Budget`
  (the survivor cap,
  `work/sectarms/the-sweeps-survivor-cap-declines-a-unit-scale-saddles-node.md`).
  No node is handed in, so the march crosses the node in one step, and
  limb 3's tube straddles. The pair refuses `Unaccounted`, which topo
  reads as unclassified. Main does the same, so this is not a
  regression.

## What would close it

The accounting could read a node's sectors from what the node frame
proves: the sign of `φ` in each sector off the lines, out to the box the
Hessian bounds. The survivor cap's own issue covers the second limit.
