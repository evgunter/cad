---
id: a-curved-faces-domain-is-read-from-every-loop
kind: unit
title: a curved face's domain is its outer loop's lift less its rings', and the lane triangulates that
status: parked
opened: 2026-11-14
priority: P0
cost: H
design: true
blocked_on: [one-chart-table-and-the-lane-takes-every-analytic-chart]
---


Unit 3 of the chart lane (`work/tesslane/plan.md`, The chart lane). Follows unit 2.

## What it is

A curved face's domain is one value: its outer loop's lift (the stored chart images placed end to end, `topo::Body::loop_lift`) less its rings' lifts. The lane triangulates that region: boundary points are the shared chord points, interior points are the lattice, and inside or outside is the planar lane's even-odd count over all loops. Both curved lanes' ring guards go, and `TessellateError::RingOnCurvedFace` with them (its `Display` says no construction makes a ring on a curved face). A loop that does not close in its chart refuses typed, as a capability, in one place.

- `trim_polygon` reads `face.outer` only today.
- `memo::FaceInputs::gather` folds the rings' rows into its key.

## The open part (`design: true`)

Nothing in `topo` places one loop's lift relative to another's: `pcurves::Lift`'s doc says a chain's start is gauge. A ring's lift may sit whole periods from the outer loop's window (and on a sphere, on the other side of a pole row). Placing it is a decision about the lift, on `crates/topo/src/pcurves.rs`, which is not this program's ground. Whether it is needed at all, and how often, is being measured (branch `tesslane/measure-lift`); what the placement is, and where it lives, is weighed with that file's owner once the measurement is in.
