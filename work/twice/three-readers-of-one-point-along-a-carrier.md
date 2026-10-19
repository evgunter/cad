---
id: three-readers-of-one-point-along-a-carrier
kind: issue
title: Three readers decide whether points are one point, each its own way, and splitting/order.rs keeps the sub-ε chain residual the carrier cluster now escalates
status: open
opened: 2026-10-15
priority: P3
cost: M
---

Found by PR 5188's review (style Q1 and Q4).

## Three readers of "these points are one point"

Three readers decide whether points are one point, each its own way:

| reader | reads | closes | in band | definitely apart within a chain |
|---|---|---|---|---|
| `crates/topo/src/boolean/carrier_cross.rs` `one_point_at_end` (5188) | 1-D metred gaps at a span end | one spread decision, `max − min` over the end's cluster | escalates `Crossing(Order)` | escalates |
| `crates/topo/src/boolean/vtxfac.rs` `ring_groups` | 3-D distances, `bool_contact_vertex` | union-find over pairwise Zero | escalates `VertexOnVertex` | joins transitively |
| `crates/topo/src/splitting/order.rs` `sort_along` / `group_coincident` | 1-D gaps along an edge | successive pairs only | escalates the comparison | the residual below |

They share a policy (no pair of one point in band) and nothing else. One helper cannot serve all three without moving outcomes: `ring_groups` partitions by union and must keep joining through a chain, while `one_point_at_end` identifies a cluster with its end, so a chain wider than the band contradicts it.

## The residual `order.rs` keeps

`splitting/order.rs:338-345` discloses the case the carrier cluster now escalates: a chain of sub-ε gaps whose successive gaps each read Zero, while their span lands in `(ε, K·ε)`. Those crossings sort in an order that can depend on insertion order, the same kind of operand-order choice 5188 closed in `carrier_cross.rs`.

Closing it, with one spread decision per run of Zero gaps that escalates in band, would turn the residual's builds into escalations. That needs its own fence.

## 2026-10-15 — `carrier_cross`'s reader is gone (FUSE, PR 5154)

The curve-pair reader (PR 5154) replaced `carrier_cross`'s candidate search with `geom_brep::curve_pair::meet`. `one_point_at_end` and `bool_carrier_cross_cluster_spread` (5188) go with it, so the table's `carrier_cross.rs` row no longer stands.
- A meeting at a span's end is now `Hit::End` on the span reading's raw chord, and the endpoint arms own it.
- The operand-order dependence 5188 closed is closed in the pose instead: two lines' tilt is levered by the overlap of their spans' projections, so a split does not move the verdict (`meet_rows::two_lines_read_whole_or_in_pieces_meet_alike`, and the glued-rod row building alike).

The other two readers in the table are untouched.
