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

## 2026-11-25 — Guarded past the order's term; open at it

PR 7731 now guards the exposure the pivot adds. A window read past the
order's own term is read only where `reduce::junction_end_boundary_clear`
decides every boundary of the target face clear of the spread; failing
that, it refuses `JunctionEndUnread { NearBoundary }`. Three kinds of
boundary are excepted:
- the rim's own edges, decided on the rim at three points;
- edges leaving the end on the target's own side, read by the side
  margin toward the target out to `1.25σ` and by the distance floor
  past it;
- lone vertices, which are read by their distance.

Pinned by `a_short_elbow_near_a_neutral_point_refuses_near_its_far_rim`.

**Still open, at the order's own term**: the neutral point's order-4
spread (0.111 m at `ε = 1e-6`, 0.0195 m at `1e-9`) and every order-2
window past it, as on main. The row stays P1 for that.

## 2026-11-25 — Guarded at every pivot

`reduce::junction_window` now asks `junction_end_boundary_clear` of
every window, the order's own pivot included, and refuses
`JunctionEndUnread { NearBoundary }` where it fails.

Measured before extending it: the guard applied at every pivot, each
pivot-m read logged, over topo, sweep and editor-core (`ci` profile plus
every `elbow`/`junction` test in the slow set) at `ε = 1e-9`, `1e-6` and
`1e-12`. There were 1226 pivot-m reads, 1155 at order 2 and 71 at order 4.
Every one was decided clear, so no verdict moved. geom-brep doesn't
depend on topo, so the guard can't move its verdicts.

Pinned by `a_short_elbow_off_its_neutral_point_refuses_near_its_far_rim`:
`0.2` round the rim from the neutral point, the window is order 2 at
its own pivot, with `σ ≤ 3.6·√(ε/δ)`. The far rim at `0.4σ` and `0.8σ`
refuses `NearBoundary`, and `3σ` and the quarter turn build at the closed
form. With the guard restored to "past the order only", the same poses
pass the side reading and refuse in band at later doors
(`ArcCylinderRoots` at `0.4σ`, `VertexOnCurvedFace` at `0.8σ`), so
the pin goes red. No wrong body was built at those poses.
