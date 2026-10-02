---
id: validate-material-side-zero-mints-an-indeterminate
kind: issue
title: validate's material-side check mints an Indeterminate by hand after a definite Zero
status: open
opened: 2026-10-01
---


## What

`crates/topo/src/validate.rs`, inside `tier3_local_checks_marked`'s
material-side jet check: `decide("material_cusp_side",
Margin::sagitta(signed, arm), band)` answering `Ok(Sign::Zero)` pushes
`ValidationError::SliverDihedral { cause: Indeterminate { margin:
MarginDiag::INVALID, predicate: Some("material_cusp_side"), .. } }`
built by hand. The comment argues the arm cannot be reached (the same
quantity's magnitude decided positive one decision above) and
announces it anyway; the announcement is right, but the escalation it
carries is on no frame's escalation log.

## Shape

`geom_core::k_stats::decide_nonzero("material_cusp_side", …)` is the
door: it reads a side off the sign and escalates a definite `Zero`
inside the funnel, so the `Cusp`/`Slit` match loses its third arm.
`crates/topo/src/boolean/rim_wedge.rs`'s `Sign::Zero` arm on the same
reading is the same shape on cleave's ground, appended to
`work/cleave/topo-mints-indeterminates-outside-the-funnel.md`. Found
by the `linalg/decided-not-minted` sweep of `MarginDiag::INVALID`
literals.
