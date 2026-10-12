---
id: a-touch-along-a-glued-tangency-cites-no-tangency-decision
kind: issue
title: An edge-edge row along a glued tangency cites the two end fusions only: the glue's tangency decision has no citation path into a boolean's records
status: open
opened: 2026-10-11
priority: P2
cost: M
design: true
---


Filed by the `ee` writer of
`the-boolean-records-no-line-touch-and-the-census-does-not-demand-one`.

## What

C3's shortest-chain rule, as that row reads it: a touch along a line
cites its two end fusions, and where a flank pair is a glued tangency,
also the glue's tangency decision. The writer cites the end fusions only.
`boolean::touch_rows` (`crates/topo/src/boolean/mod.rs`) builds the row's
citations from `contacts.vv`. A record's `Backing::Decided` indexes the
reduction's pending rows, which `ops::cite_rows` renumbers. The glue's
decisions reach the result by another route: `glue::touched` emits them
as the coincidences recorded ahead of the pending rows. No record cites
one, and no path exists for one to.

## Reach today

Unreached. A probe on the `EdgeEdge::Touch` arm of `recl::recl_edges`
found no declared-`Tangent` flank pair in about 6,300 touch reads: all of
topo's `all` binary, and sweep's and editor-core's `all` binaries until a
500 s cap (slow set not run). The same-side tangency short-circuit in
`recl::resolve_edge_edge` returns a germ before the membership reads
(`[ev]` PR 4723). An opposite-side tangent pair can reach the
membership, but no fixture puts one on the `Touch` arm.

## Owed

Once PR 4723's ruling retires the short-circuit, a same-side tangent
touch along a ruling reaches `EdgeEdge::Touch`. Its row then needs the
glue's tangency decision among its citations. Choosing how a record
cites a glue decision is the design question: a pending row that mirrors
the glue's, or a `Backing` that addresses the recorded coincidences.
Until then the row is backed by its end fusions alone.
