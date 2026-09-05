---
id: smooth-arm-siblings-disagree-on-the-in-band-case
kind: issue
title: sweep: the two must-carry Smooth arms disagree on the in-band case and on how much of the edge they read
status: open
opened: 2026-09-05
---


## Finding (FILLET-H6's lane, PR 1891, not changed there)

The must-carry rule now has one home, `geom_brep::tangent_second_order`
(`crates/geom-brep/src/dihedral.rs`), returning a VERDICT; the two callers
keep their own escalation policy and they disagree:

- `crates/sweep/src/extrude.rs`'s strut arm escalates the in-band case
  typed (`SliverJoin`), reading ONE point with no lane gate;
- `crates/sweep/src/revolve/upgrade.rs::jet_determinate` folds `Err` into
  `false` and KEEPS the conventional description, gating on
  `tangent_certificate_lane` and sampling seven interior points.

`docs/FILLET-H6-SPEC.md`'s summary ("in-band escalates typed") was wrong
about revolve. For a strut the two agree in fact (κ_rel is constant along a
ruling; a `Line` on plane/cylinder pairs is in the lane) — an argument, not
a shared spelling. Unifying is a behaviour change: a revolve that builds
today would start refusing in-band. Making the strut sample seven points
multiplies its rows in the K stream. Needs a decision (which policy is the
rule's), then one wrapper. `folded_lever_arm`'s doc still names issue 1439's
six hand-rolled siblings; H6 removed two — the tier-3 validator's and the
boolean rebuild's remain.
