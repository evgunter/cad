---
id: a-mirrored-cone-wedge-refuses-departure-at-its-apex-off-the-plain-frame
kind: issue
title: A cone wedge revolved by a negative angle past a half turn refuses Departure at its wedge plane and cone off the plain frame, at one scalar or both, where the same wedge by the positive angle is embedded in every pose
status: open
opened: 2026-11-22
priority: P1
cost: M
---


## Finding

Filed by lane `embed/apex-separator` (2026-11-22, the PR 7315 fix pass),
on its head after merging main `7669fcfdd` (PRs 7120 and 7321).

The triangle with a leg on the axis, revolved about `y` through the
verdict door (`topo::clearance::embedded`, the default budget). By `+θ`
every pose below is `Embedded` at both scalars, and
`self_intersection_proves_injectivity::a_cone_wedge_past_a_half_turn_is_embedded_at_its_apex`
pins it. By `−θ` only the plain frame is:

| `−θ`, pose | f64 | `Interval` |
|---|---|---|
| −4.7224, plain | Embedded | Embedded |
| −4.7224, turned (axis (1,2,3), 0.7) | Embedded | `Uncertified { 1v1, 4v1, Edge(6v1), Departure }` |
| −4.7224, moved (64, −57, 52) | Embedded | `Budget(Pairs)` |
| −4.7224, moved (640, −570, 520) | `Uncertified { 1v1, 4v1, Edge(6v1), Departure }` (381 discharged) | `Budget(Pairs)` |
| −4.7224, a hundred times the size | `Uncertified`, the same pair | `Budget(Pairs)` |
| −4.7224, a needle (r 0.05) | `Uncertified`, the same pair | Embedded |
| −4.7224, flat (h 0.05) | `Uncertified`, the same pair | Embedded |
| −5.5, plain | Embedded | Embedded |
| −5.5, turned | `Budget(Depth)` | `Budget(Depth)` |
| −5.5, moved (64, −57, 52) | Embedded | `Budget(Pairs)` |
| −5.5, moved (640, −570, 520) | Embedded | `Budget(Pairs)` |
| −5.5, a hundred times | `Uncertified`, the same pair | `Budget(Pairs)` |
| −5.5, a needle | `Uncertified`, the same pair | Embedded |
| −5.5, flat | Embedded | Embedded |

The pair is the wedge's plane `1v1` (the start plane, normal ≈ x) and
the cone `4v1`, across the ruling `6v1`, near the apex vertex `3v1`.
Several poses read one class at f64 and another at `Interval`; that is
part of the finding, not tuned away.

**Traced once** (−4.7224 moved (640, −570, 520), f64, `CV_SIGN` prints,
not committed). In the last pair `departure` presents both, the plane
`1v1` at `u ∈ (0, 0.125)`, `v ∈ (0.875, 1.0)` by the ruling's side,
the cone `4v1` at `u ∈ (1.5708, 2.3562)`, `v ∈ (−0.1768, 0)` by the
apex's side. Against the apex alone the plane is its corner
(`Zero::Point`), and the cone's rectangle is widened past its cell to
`u ∈ (π/2, π)`. The plane through the apex reads the cone's rate
`[−0.0216, 0.1446]` across the apex side: `None`. The run refuses with
381 discharged and 8 refused, at a coarse cell, not at the depth
budget. Which exit refuses is not traced.

**Unchecked:** why the mirrored wedge reads differently from the
`+θ` one. The `+θ` wedge's cone is the face over `u ∈ [−π, 1.58]`;
the `−θ` one's is another sheet. Whether the cone's rectangle is
widened to a line at the cell's reach (`present_chart`'s "a line the
cell stops short of … is widened to it") or is the face's extent, and
whether a smaller cell is reached before the refusal, are the two
probes to take first.
