---
id: a-curved-faces-domain-is-read-from-every-loop
kind: unit
title: a curved face's domain is its outer loop's lift less its rings', and the lane triangulates that
status: parked
opened: 2026-11-14
priority: P0
cost: H
blocked_on: [one-chart-table-and-the-lane-takes-every-analytic-chart, a-rings-lift-is-placed-against-its-outer-loop-in-one-place]
---


Unit 3 of the chart lane (`work/tesslane/plan.md`, The chart lane). Follows unit 2.

## What it is

A curved face's domain is one value: its outer loop's lift (the stored chart images placed end to end, `topo::Body::loop_lift`) less its rings' lifts. The lane triangulates that region: boundary points are the shared chord points, interior points are the lattice, and inside or outside is the planar lane's even-odd count over all loops. Both curved lanes' ring guards go, and `TessellateError::RingOnCurvedFace` with them (its `Display` says no construction makes a ring on a curved face). A loop that does not close in its chart refuses typed, as a capability, in one place.

- `trim_polygon` reads `face.outer` only today.
- `memo::FaceInputs::gather` folds the rings' rows into its key.

## The open part (`design: true`)

Nothing in `topo` places one loop's lift relative to another's: `pcurves::Lift`'s doc says a chain's start is gauge. A ring's lift may sit whole periods from the outer loop's window (and on a sphere, on the other side of a pole row). Placing it is a decision about the lift, on `crates/topo/src/pcurves.rs`, which is not this program's ground. Whether it is needed at all, and how often, is being measured (branch `tesslane/measure-lift`); what the placement is, and where it lives, is weighed with that file's owner once the measurement is in.

## Measured (2026-11-14; `crates/mesh/tests/lift_probes.rs`) — the open part is settled

- **A ring's lift is in its outer loop's polygon as stored on 23 of 24 rings, and one whole `u` period off on the other.** The rings of the donut, the pipe walls, the cone and ball ∪ tube sit inside. Ball ∖ tube's ring, a whole circle carried as one `Projected` row, is stored at `u ∈ (−2.44, −0.70)` against an outer loop at `u ∈ [π, 2π]`; the same circle cut into two arcs (ball ∪ tube) is stored inside. `v` is never displaced, and no row of a ball ring or its outer loop carries the pole involution.
- **Nothing in `topo` places one loop against another on an analytic chart, but a placer exists for spline charts**: `Trim::with_rings` in `topo::boolean::spline_region` moves a ring by the one whole number of periods per closed channel that puts its first point in the outer loop's cover, or answers `None`. `cover::TorusChartWindow::open_loop` abandons its window on any ring, `chart_region::extract_face_uv` keeps rings only as deductions, and `props`' quadrature folds each loop on its own.
- Every ring is clockwise where `sense` is true, the reverse of its outer loop, so the even-odd count has what it needs.
- **A loop that does not close exists at rest, through the Euler doors only.** No door-built pipe, cone, ball, washer or donut has one: a full rim always sits in a loop that closes, its wall either two half faces or seamed with the seam walked twice a period apart. A seamless cylinder pipe and a seamless cone frustum built through the Euler doors finish, pass every validator, and refuse `RingOnCurvedFace` in the mesh. That is the capability refusal this unit states in one place; `work/pcert/a-seamless-band-mints-and-passes-tier-3-though-loopwraps-says-no-constructor-makes-one` is the row about whether such a body should be at rest at all.

So the placement is one integer per closed channel, and it is cut out as its own unit, `a-rings-lift-is-placed-against-its-outer-loop-in-one-place`, which this unit now also waits on. Both designers put it in `topo`, beside the lift; with a placer already there for spline charts the question left is only that the two become one, which is that unit's.
