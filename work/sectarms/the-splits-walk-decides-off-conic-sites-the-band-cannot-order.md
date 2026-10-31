---
id: the-splits-walk-decides-off-conic-sites-the-band-cannot-order
kind: issue
title: The split's conic walk decides an order for off-conic sites the band cannot order, and refuses it as NotAlternating
status: open
opened: 2026-10-31
priority: P3
cost: M
refs: [the-walk-order-is-spelled-twice, 6126]
---


Found by PR 6126's walk-order rows (`sectarms/walk-order-held`),
which hold the split's conic walk against the boolean's turn order.

## What

`crates/topo/src/splitting/join.rs` `conic_pairs` reads each crossing's
walk coordinate as the eccentric anomaly of its point
(`conic.param(c.point)`, around line 702) and decides the gap between
two crossings as `Δw·|C′(w_mid)|` (`gap`, around line 729) through
`order::sort_along_runs`. A crossing is a vertex, within the band ε of
the conic but not on it. A point `δ` across the conic moves its
eccentric anomaly by about `δ cot ψ / |C′|`, the same `δ cot ψ` that
`crates/topo/src/boolean/join.rs` `off_conic_slack` holds the boolean's
turn against (`travel` escalates at `bool_join_arc_clear` unless the
margin clears it). The split reads no slack, so on a steep flank it
decides an order the band cannot support.

Measured, with an ellipse of aspect 30 on a cylinder (`cot ψ` ≈ 15 at
`θ = π/4`), a flank pair moved 0.9ε to opposite sides of the conic,
between a germ and one more site (`join_walk_order_rows.rs`
`off_a_steep_flank_the_lanes_refuse_rather_than_order_two_ways`):

| Δ (bands of arc) | split's walk | boolean's turn |
|---|---|---|
| 12, 16 | the pair decided in reverse; refused as `NotAlternating` | escalates (`bool_join_arc_clear`) |
| 24, 32 | escalates (`split_join_line_order`/`_gap`) | escalates |
| 48, 64 | orders correctly | escalates |
| 96, 128 | orders correctly | orders correctly |

No order comes out wrong: a reversed neighbour pair always breaks the
alternation, so the walk refuses. But it refuses as
`ConicCrossingsCase::NotAlternating`, a claim that the face's
crossings do not alternate, where the cause is two sites the band
cannot order. The disabled red
`off_a_steep_flank_the_walk_escalates_where_it_cannot_order` holds the
first.

## Done when

The walk decides its gaps against the sites' off-conic slack, as
`travel` does, or reads them some other way that is not moved by it,
so the pair above escalates rather than read as a crossing case; the
disabled red is lifted.
