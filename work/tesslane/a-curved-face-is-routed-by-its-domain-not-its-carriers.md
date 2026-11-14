---
id: a-curved-face-is-routed-by-its-domain-not-its-carriers
kind: unit
title: lane_of routes a curved face on its domain's shape, so a notch or a ring reaches the lane that meshes it
status: parked
opened: 2026-11-14
priority: P0
cost: M
blocked_on: [a-curved-faces-domain-is-read-from-every-loop]
---


Unit 4 of the chart lane (`work/tesslane/plan.md`, The chart lane). Follows unit 3.

## What it is

`tessellate::lane_of` routes a curved face on its domain: a face with no ring whose outer loop is an iso-rectangle of its chart goes to the rectangle lane (`curved.rs`); every other curved face goes to the chart lane. The carrier-kind predicates (`has_trim_carrier`, `has_tilted_circle`, `takes_sphere_arm`, `face_target`) go, and `props`' rectangle predicate is read as a route where `curved::require_iso_rectangle_face` reads it as a refusal. The chord pass reads the same route, from the same door, so the two cannot disagree.

A boundary run on a chart iso line gets a lattice line through it, so a notched wall has every boundary point on the lattice and sizes at δ with no headroom, as a rectangle does.

## Closes

`a-notched-or-ringed-cylinder-wall-does-not-tessellate`: the notch meshes and passes `check_mesh`, and its `reach_probes` row is re-pinned.

## The ring half moved to unit 3 (2026-11-14)

A face that holds a ring is routed to the chart lane by unit 3, with the lane that reads it: a guard left standing between the two units would have needed a refusal text nobody could state truthfully. So unit 3 closes `valid-ringed-torus-faces-build-but-cannot-be-meshed` and the rings on a cylinder, cone, sphere and torus, and removes `TessellateError::RingOnCurvedFace`. This unit keeps the notch, the iso-line lattice rule above, and the retirement of the carrier-kind predicates.
