---
id: holed-sphere-side-consults-no-pole-where-a-pole-would-decide-it
kind: issue
title: A holed sphere face's side is read by Gauss-Bonnet alone, consulting no pole, though a pole off its loops now decides it
status: open
opened: 2026-10-29
priority: P4
cost: E
---

Filed by RESTCURVE's `validity-refuses-an-interior-chart-singularity` (branch `restcurve/chart-singularity-is-a-vertex`), on NEARPOINT's slate because the sphere arms of `crates/geom-brep/src/props/curved.rs` are NEARPOINT's.

## What

`side_of_sphere_loops` (`curved.rs`, near `:3570`) reads a face with holes by which side of its loops has positive Gauss–Bonnet area (`props_sphere_face_area`) and consults no pole. With tier 3 refusing a pole inside any face (`topo::ValidationError::ChartSingularityNotAVertex`), a pole off every loop of a valid holed face lies outside it, so the pole reading `pole_free_side` takes for a one-loop face would decide the holed face's side too, with a margin that does not shrink with the face's area.

## What a fix owes

Decide whether the holed path should read a pole where one lies off every loop (and keep the area reading where none does), or say why the area reading alone is the one to keep. Not a soundness defect: the area reading is correct on every valid body; this is the spec's "arm reachable only through an invalid body" class, filed for the owner's call.
