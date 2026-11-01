---
id: ruled-cut-off-does-not-meter-the-cut-cycles-own-other-edges
kind: issue
title: blend: the ruled cut-off meters the cap's OTHER cycles against the sliver, not the non-rim edges of the cycle it cuts
status: closed
opened: 2026-09-25
closed: 2026-09-26
priority: P1
cost: D
---


## Finding (by reading; not measured)

The ruled cut-off's cap meter (`ring_clearance_pass` arm (c),
`crates/sweep/src/blend/surgery.rs`) meters every cycle of the cap
OTHER than the one the cut runs in (`CapSliver::cut`). Edges of the cut
cycle itself other than the link's two rims are not metered: a cap
outline with a notch reaching into the corner region between the
cut-off arc and the old vertex (the notch's edges belong to the cut
cycle) would have the arc `mef`'d across them. Nothing else meters it:
predicate 2's boundary-pair screen reads SUPPORT faces, and the cap is
not a support.

The cut cycle cannot simply join the other cycles in arm (c): that arm
meters each edge by its whole carrier, and a rim's neighbour can share
the rim's carrier (a wall circle split into several arcs at seams puts
every arc on the rim's circle, which touches the section circle at the
foot, so its whole-carrier margin is never positive). So the taker
needs a finite-edge meter — the edge's own window on its carrier — for
the cut cycle's non-rim edges.

## What the taker owes

A fixture first (an extruded D-rod whose flat carries a small notch
beside the upper crease, inside the sliver at `ROD_FILLET`), measured
against today's tree: does the carve return a body, and does tier 3
accept it? Then the finite-edge meter, or a typed refusal.

## Measured (2026-09-26, review of `band/cap-ring-in-sliver`)

Live, not hypothetical: an L-shaped channel cut into the D-rod's
outline from the flat below the upper foot, whose tip reaches
`x = 0.295, y ∈ [0.385, 0.39]` (inside the sliver, `≈ 0.1045` from the
ball centre), carved, passed tier 3, and returned exactly the
channel-free `ΔV = −2·A·L` — before and after the unit that added the
cap meter.

## Closed by (`band/cap-ring-in-sliver`)

The cap meter now reads EDGES, each over its own stored window
(`piece_distance` / `piece_along` in `crates/sweep/src/blend/surgery.rs`),
not whole carriers, so the rim's neighbours on the rim's own circle are
no obstacle: `ring_clearance_pass` arm (c) meters every edge of the cut
cycle except the two rims the cut shortens (`CapSliver::rims`,
`crates/sweep/src/blend/open/ruled.rs`) exactly as it meters the cap's
other cycles. The channel refuses `RingClearance` at the cap:
`band_ruled_cap_ring::a_channel_in_the_cut_cycle_reaching_into_the_sliver_refuses_ring_clearance`.

