---
id: the-junction-side-reading-assumes-no-other-target-boundary-within-its-reach
kind: issue
title: The junction's side reading assumes the target face has no boundary but the rim within the spread's reach of the end, and never checks it
status: open
opened: 2026-11-24
priority: P1
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
takes spreads up to 0.35 m at `ε = 1e-6`, 0.063 m at `1e-9` and 0.011 m at
`1e-12`.

## Done when

The side reading proves a lower bound on the distance from the end to
every edge of the target face but the rim's own edges through it
(for example, from each edge's carrier enclosure), exceeding `σ`
under a named predicate. Short of proven, it refuses
`JunctionEndUnread`. A row shows a target face with a short edge within
the spread refusing.

## 2026-11-25 — Re-banded P1: the exposure

PR 7731's pivot makes this premise load-bearing over a whole range of
turns. Every edge ending within about `15ε` to `5.6√ε` of a neutral
point is now read by its side, at spreads up to 0.35 m at `ε = 1e-6`,
where before that edge refused. Before the pivot it carried this
weight only at a neutral point itself. A target face with an edge of its
own inside that reach, other than the rim through the end, could put a
root of the spread on the target face. The side reading would then miss
a crossing: a wrong body, not a refusal.

Not guarded in PR 7731, because the guard carries design content of its
own:
- which edges count as boundary: a periodic face's seam edge through
  the end is not one, the rim's own edges are not, and an edge leaving
  the end at a corner `junction_end_corner` decided within π is already
  read;
- a distance lower bound read through the edge's cover, which
  `scripts/gates/cover-kind-match` registers;
- a body-level pose with a short edge of the target face inside the
  spread, to pin it.
