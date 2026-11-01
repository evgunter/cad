---
id: a-closed-splines-projection-stops-at-its-start-short-of-the-seam
kind: issue
title: A closed spline's projection seeded at its start stops there, short of a foot just across the seam
status: open
opened: 2026-10-28
priority: P2
cost: E
---


Found by the sweep of `main-red-cylinder-sphere-section-runs-out-of-cells-near-the-node-at-1e-6`
(SECT, 2026-10-28), which fixed the same shape in `Curve3::foot`
(`crates/geom/src/curves/on_carrier.rs`).

## Measured

`NurbsCurve3::project` (`crates/geom/src/curves/projection.rs`,
`project_seed` then `project_from_seed`) on a closed quadratic loop,
knots `[0, 0, 0, 0.25, 0.5, 0.75, 1, 1, 1]`, controls `(1, 0)`, `(1, 1)`,
`(−1, 1)`, `(−1, −1)`, `(1, −1)`, `(1, 0)`, unit weights, at points
on the loop:

| point | answer `(t, distance)` |
|---|---|
| `C(0.99)` | `(0.0, 7.84e-2)` |
| `C(0.995)` | `(0.0, 3.96e-2)` |
| `C(0.999)` | `(0.0, 7.98e-3)` |
| `C(0.01)` | `(0.01, 0.0)` |

The seeding sweep samples both ends of every span, and on a closed
carrier the domain's two ends are one point: the strict `<` keeps the
start, Newton from it is clamped at the start, and the parameter
stagnation accepts the domain-end foot. The distance carried is honest,
but it is not the nearest point, for a point within one span of the
seam on the end side.

## What a fix has to supply

A Newton run that stagnates at a domain end, continued from the other
end, the nearer foot kept (what `Curve3::foot` does now), or a seeding
rule that breaks the end tie toward the nearer neighbour. The callers
(about forty in `crates/*/src`) that read the foot as nearest on a
closed carrier are the ones whose answers move.
