---
id: radius-headroom-reads-a-cones-bend-at-the-edge-not-at-the-balls-foot
kind: issue
title: blend: fillet3_radius_headroom reads a cone's bend at the edge sample, where the ball's foot sits a setback away at a smaller radius
status: open
opened: 2026-10-06
priority: P3
cost: E
---



## Finding

`battery::radius_headroom` (`crates/sweep/src/blend/battery.rs`) takes
its arm from `geom_brep::min_radius_of_curvature_toward`
(`crates/geom-brep/src/implicit.rs`) at `p`, a sample of the EDGE. For
the sphere and cylinder the arm is a constant, and for the torus it is a
bound global over the ring, so where it is read does not matter. For the
cone it is local: the radial distance `ρ` of `p`. The ball's foot on the
cone is not at `p` but a setback along the generator away from it, at
`ρ_foot = ρ_p − s·sin α` where the cone narrows away from the edge (a
frustum's base rim, the ball inside the cone), and the bend there is
`cos α/ρ_foot`. `ρ_p` is a lower bound on the foot's osculating radius
`ρ_foot/cos α` only while `s·sin α ≤ ρ_p·(1 − cos α)`; a cone with a long
setback against a small half-angle breaks it, and then the edge read
claims more headroom than the foot has.

Found while siding the predicate (the
`radius-headroom-reads-a-ball-outside-a-concave-support-as-inside` unit),
which kept the cone's local read as it found it.

## What the taker owes

Read the cone's bound where the ball touches it — at the foot the arm
already derives (`EdgeBlend`'s trimline on the cone), or as the smallest
`ρ` over the band the setback spans — and pin a steep frustum's base rim
on which the edge read passes and the foot read refuses.
