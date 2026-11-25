---
id: a-small-hole-in-a-sphere-face-reads-its-solid-angle-levered-by-the-radius-against-the-band
kind: issue
title: The sphere face's mass properties read a hole's solid angle levered by the radius against the band, a length of order ρ²/R, so a hole of radius 1e-3 escalates at ε = 1e-6
status: open
opened: 2026-11-25
priority: P2
cost: M
---


Found by `heldrecords/parting-division` (PR 7783, its fix pass), on
`heldrecords/split-on` at `80f11deb86` with that branch's parting.

## What

`crates/geom-brep/src/props/curved.rs` `sphere_face` requires each
ring's hole to have positive solid angle
(`require_solid_angle_as("props_sphere_hole_area", hole, arm, band)`),
reading `Margin::levered(angle, arm)` with the arm the sphere's radius.
A hole's solid angle is about `π ρ² / R²` for a hole of radius `ρ`, so
the levered reading is a length of order `ρ² / R`: second order in the
hole's size, against a band in metres. A hole far larger than the
tolerance reads in band.

## Witness

The ball of radius `√2` about the origin (`ball_poled_z`) less a full
revolve about `y` of a triangle whose apex sweeps a circle of radius
`ρ` on the sphere, both its cones going into the ball
(`crates/sweep/tests/a_closed_rider_lying_in_a_face.rs`, the small-rim
rows). The Boolean divides the sphere face along the rim, and the
result gate's mass properties refuse at ε = 1e-6:

- `ρ = 1e-3`: `ResultInvalid { VolumeUncomputable { source: Escalated { margin 2.22e-6, predicate props_sphere_hole_area } } }`;
- `ρ = 5e-4`: `ResultInvalid { VolumeUncomputable { source: SphereLoop { what: "props_sphere_hole_area" } } }`.

The same rows build SOUND at the default ε and at 1e-12, and at every
`ρ ≥ 3e-3` at 1e-6.

## Done when

The hole's premise is read on a first-order length (the hole's reach,
or its solid angle's square root levered by the radius). The witness
then builds SOUND at ε = 1e-6 for `ρ ≥ 5e-4`, and its pin in
`a_closed_rider_lying_in_a_face.rs` is raised to match.
