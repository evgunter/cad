---
id: two-coincident-curved-edges-touching-record-no-edge-edge-row
kind: issue
title: Two coincident curved edges touching along their length record no edge-edge row, and the census does not ask for one
status: open
opened: 2026-10-11
priority: P3
cost: M
---


Filed by the `ee` writer of
`the-boolean-records-no-line-touch-and-the-census-does-not-demand-one`.

## What

D1's record paragraph says every pair whose interiors meet is
recorded, and that a curved one is confirmed as a curve touch.
`boolean::touch_rows` (`crates/topo/src/boolean/mod.rs`) writes the
edge-edge row only where both edges are lines. Two arcs leaving a vertex
along one tangent need not coincide, so an arc pair would need its own
coincidence decision before any row is written. On the census side,
`census::ee_collinear_lane` reads line edges only (`edge_is_line`), so
nothing asks for the row either.

## Reach today

Unreached. A probe on `touch_rows`' curved skip fired in none of about
6,300 touch reads: topo's `all` binary, and sweep's and editor-core's
`all` binaries until a 500 s cap (slow set not run).

## Owed

A coincidence decision for two curved edges on one carrier (carrier
identity through the ladder, then the span overlap), the row written
from it, and a census arm that asks for the row of a curved overlap.

## Measured 2026-10-19 (`cleave/keel-slit-ring`)

The census half has moved: pass 5′ (`census.rs`, `sweep_curved_edge_edge`)
now asks a curved overlap for its edge-edge row, and the split's curved
slits and rings reach it with none
(`work/cleave/a-curved-slit-fails-tier-3-prime-on-its-two-coincident-arcs.md`).
