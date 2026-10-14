---
id: census-curved-record-lanes-read-circles-only
kind: issue
title: The census confirms vertex-on-edge and edge-edge records on line and circle edges only; an ellipse, spiric or spline edge refuses CensusUnsupported
status: closed
opened: 2026-10-07
priority: P1
cost: H
branch: fuse/census-reads-every-curved-edge
pr: 4946
closed: 2026-10-13
---

## The finding

`crates/topo/src/census/curved.rs` confirms a `(vertex, edge)` record
whose edge is curved, and an edge-edge record with a curved side, on
`Line` and `Circle` carriers only (`read`); an `Ellipse`, `Spiric` or
`Nurbs` edge pushes `CensusUnsupported` ("a record on a curved edge is
certified on a circle edge only"). The curved join writes such records
whenever a recorded vertex is joined away into an ellipse arc (a tilted
section of a cylinder) or a spiric one. No `ci` row reaches it.

The edge-edge lane finds candidate meeting points where a line or
circle meets a circle through the circle's plane, plus each edge's
midpoint and the points halfway between one edge's end and the
other's ends for an overlap (`curved_interiors_meet`).

## What it needs

The point-on-carrier and crossing readings for the conic and spiric
kinds (`Curve3::param_near` already inverts them), and a spline foot.

## 2026-10-13, closed: PR 4946

Reproduced through the boolean in `crates/sweep/tests/census_reads_conic_records.rs`
(a cube corner, and a box edge, on the tilted drum's ellipse rim). The
lanes now read every carrier kind: `carrier_speed::speed_range` bounds
the speed, the foot is `param_near`'s or the search's, and
`census::curved::meetings` replaces the circle-only crossing finder.
