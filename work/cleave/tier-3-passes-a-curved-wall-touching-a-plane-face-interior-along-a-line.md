---
id: tier-3-passes-a-curved-wall-touching-a-plane-face-interior-along-a-line
kind: issue
title: tier 3 passes a body whose curved wall touches a plane face's interior along a line with no edge for the contact
status: open
opened: 2026-10-02
priority: P2
cost: H
---


## What

A split piece in which a hole's wall touches the cut face's INTERIOR
along a ruling passes `validate_closed` and `validate_geometric`.
It is a zero-thickness, undeclared tangent contact, and there is no
edge for it. `topo::contact_marks` cannot report it either, since it
has no edge to mark.

## How to reach it

Only under a plant, so far. In `splitting/rules.rs` `wall_graze`,
change `(false, WallBend::OutOfMaterial) => side.opposite()` to
`=> side`, so a concave graze is sent with its neighbours. Then a
round hole (r = 0.5 in a 4 × 4 plate, h = 1) grazed from inside at
any azimuth, a conical socket, or a counterbore
(`crates/sweep/tests/split_tangent_edge_curved.rs`, the concave rows)
answers the true volumes. For example 6.0 / 9.2146 for the hole at
y = 0.5. Both pieces pass tier 3, and neither carries a plane-to-curve
edge marked `Tangent` or `SmoothUnderdetermined`: the contact runs
through the cut face's interior.

The review that found this read the contacts as knife edges marked
`SmoothUnderdetermined`. Measured, those marks are the hole's own seam
edges, cylinder to cylinder, which the operand carries too.

No unplanted input has been measured to reach this. The split refuses
every concave graze it was tried on, and the guards
`a_concave_graze_of_a_round_hole_refuses` and
`a_concave_graze_of_a_revolved_hole_refuses` hold that. A Boolean or a
placement that leaves a wall tangent to a face's interior is the
other route, and has not been tried.

## Why it matters

Tier 3 is what the at-rest gate trusts. A contact it cannot see is a
body it calls valid that a downstream mesh or offset will treat as
manifold material where there is none.

## Found by

CLEAVE DR-51's review of PR 3892, measured in its fix pass.
