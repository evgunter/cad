---
id: a-full-circle-starting-a-hair-off-its-least-radius-point-reads-across-the-axis-by-its-sense
kind: issue
title: A full-circle profile whose vertex sits a rounding off its circle's point of least radius refuses ArcCrossesAxis for one sign of the angle and pinches for the other
status: open
opened: 2026-11-22
priority: P2
cost: M
---

Found by LATHE's one-side-of-the-axis lane (branch
`lathe/one-side-of-the-axis`) while pinning a profile and its mirror
across the axis
(`crates/sweep/tests/a_revolve_takes_a_profile_on_either_side_of_its_axis.rs`).

## Measured

One full-circle segment, centre `(1, 0)`, radius 1, its vertex written
as `centre + r·(cos π, sin π)` = `(0, 1.22e-16)`: the circle touches the
sketch `y` axis at its vertex, up to the rounding of `sin π`. Revolved
about the `y` axis at `Tol::witness()`:

| revolution | refusal |
|---|---|
| `Partial(1.0)` | `NonManifoldAxisContact` (the horn's cusp at both ends) |
| `Partial(-1.0)` | `ArcCrossesAxis`: "the arc ... dips across the revolve axis" |
| `Full` | `NonManifoldAxisContact` |

With the vertex written exactly `(0, 0)` every row refuses
`NonManifoldAxisContact`. The circle never crosses the axis.

## The defect

`revolve::axis::classify_segment`'s arc arm reads the interior with
`turn_past(s.a, arc, turn, −ê_r)` (`ClearanceDecision::ArcInterior`):
the turn from the arc's start to `m`, its circle's point of least
radius, less the arc's span. For a FULL circle the span is `2π`, so a
start a hair BEFORE `m` in the traversal's sense reads `δ − 2π`,
definitely negative ("passes `m`"), while a start a hair after reads
`−δ`, in band. The swept traversal's sense follows the sign of the
angle (`reverse`), so the same profile reads definite crossing for one
sign and not the other. A full circle holds every point of its circle;
whether it crosses the axis is its reach (`ArcReach`, `rc − radius`)
alone, so its interior reading measures where its vertex happens to
sit, not a fact of the circle.

Same class as
`a-revolve-refuses-a-profile-on-one-side-of-its-axis-as-across-it`: a
refusal that states a fact the input does not have, decided by a sign
the author chose.
