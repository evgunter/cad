---
id: validate-material-side-zero-mints-an-indeterminate
kind: issue
title: validate's material-side check mints an Indeterminate by hand after a definite Zero
status: open
opened: 2026-10-01
---


## What

`crates/topo/src/validate.rs`, `MaterialStations::after_positive` (check
4's material hook inside `geom_brep::second_order_walk`):
`decide("material_cusp_side", Margin::sagitta(signed, station.arm), band)`
answering `Ok(Sign::Zero)` breaks the walk with an `Indeterminate {
margin: MarginDiag::INVALID, predicate: Some("material_cusp_side"), .. }`
built by hand, which `tier3_local_checks_marked` pushes as
`ValidationError::SliverDihedral { check: WedgeCheck::MaterialSide, .. }`. The comment argues the arm cannot be reached (the same
quantity's magnitude decided positive one decision above) and
announces it anyway; the announcement is right, but the escalation it
carries is on no frame's escalation log.

## Shape

`geom_core::k_stats::decide_nonzero("material_cusp_side", …)` is the
door: it reads a side off the sign and escalates a definite `Zero`
inside the funnel, so the `Cusp`/`Slit` match loses its third arm.
`crates/topo/src/boolean/rim_wedge.rs` reads the same side through
the same hook (`validate::MaterialStations::after_positive`, since
`encl/rim-wedge-one-walk`), so this one repair covers both callers.
Found by the `linalg/decided-not-minted` sweep of `MarginDiag::INVALID`
literals.
