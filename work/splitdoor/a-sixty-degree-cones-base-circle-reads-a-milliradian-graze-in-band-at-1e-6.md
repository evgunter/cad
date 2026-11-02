---
id: a-sixty-degree-cones-base-circle-reads-a-milliradian-graze-in-band-at-1e-6
kind: issue
title: a 60-degree cone's base circle reads a plane a milliradian off a ruling's tangency in band at 1e-6 (split_conic_belly_graze)
status: open
opened: 2026-10-28
priority: P3
cost: E
---


## What

A full cone of half-angle 60° (profile `(0, 0), (tan α, 0), (0, 1)`
revolved whole about `y`), split by a plane through its apex holding
the ruling at azimuth θ ∈ {0, 0.3, 2} and turned `t = 10⁻³` off
tangency about it, either normal, refuses at ε 1e-6
`Reduce(CrossingEscalated { edge, fault: BellyGraze(..) })` on
`split_conic_belly_graze`, margin `1.73e-6` in the band
`(1e-6, 1e-5)`, on the base circle: the plane nearly grazes the base
circle at the ruling's foot. Identical on main before the apex was
read (the probe's main logs) and after (`splitdoor/cone-apex`). At 30°
and 45°, and at 1e-9 and 1e-12, the crossing reads definite.

Pinned as a typed refusal in
`crates/sweep/tests/a_plane_through_a_cones_apex.rs`.

## Where to look

`splitting/classify.rs`'s conic crossing lane (`split_conic_belly_graze`):
whether an in-band belly beside a vertex the plane holds is the
escalation's to keep, or a sliver within the band that lands whole.
