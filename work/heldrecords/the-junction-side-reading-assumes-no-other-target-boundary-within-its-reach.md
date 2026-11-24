---
id: the-junction-side-reading-assumes-no-other-target-boundary-within-its-reach
kind: issue
title: The junction's side reading assumes the target face has no boundary but the rim within the spread's reach of the end, and never checks it
status: open
opened: 2026-11-24
priority: P2
cost: M
---


Found by `heldrecords/belt`'s design, 2026-11-24.

## The premise

`reduce::junction_end_window` reads every root inside the end-root
door's spread as off the target face once the edge stands on its own
face's side of the rim (`junction_end_side`). The argument works by
projection onto the junction's tangent plane: within the reach, the
target face is the part of its carrier on the far side of the rim. That
holds only if no other boundary of the target face passes within the
spread's reach `σ` of the end, meaning an edge of the target face other
than the rim through the end. If one does, a root on the edge's side of
the rim may still be on the target face, where that face wraps past its
other edge.

Nothing checks this, at any spread. At `ε = 1e-6`, the neutral point's
own order-4 spread on main is 0.111 m. The belt's pivot (the row
`the-junction-end-window-refuses-in-a-belt-around-a-neutral-point.md`)
takes spreads up to 0.28 m, and 0.05 m at `ε = 1e-9`.

## Done when

The side reading proves a lower bound on the distance from the end to
every edge of the target face but the rim's own edges through it
(for example, from each edge's carrier enclosure), exceeding `σ`
under a named predicate. Short of proven, it refuses
`JunctionEndUnread`. A row shows a target face with a short edge within
the spread refusing.
