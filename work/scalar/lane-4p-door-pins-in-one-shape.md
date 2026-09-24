---
id: lane-4p-door-pins-in-one-shape
kind: unit
title: LANE-4P: the four door values' pins in one shape; the certified-enclosure census counts doors as well as scalars
status: dispatched
opened: 2026-09-24
branch: scalar/lane-4p
---


## What

The first of LANE-4's two cuts (H5 ruling 3's last trait). The four
door values — `geom_brep::OffsetFitLane`, `topo::QuadLane`,
`topo::RegionLane`, `topo::ShellDoor` — get their fn-pointer wiring
pinned in one helper shape each, and
`crates/topo/tests/certified_enclosure_impl_census.rs` learns the axis
it lacks: it enumerates door VALUES (every door constructor in
`crates/*/src`) and requires a roster entry for each, with each entry
saying whether its door is formed at every certifying scalar or at
`f64` only. Takes the filed row
`the-shell-door-is-a-third-door-value-the-certified-enclosure-census-does-not-know`.
Test code and prose only. Spec: `docs/LANE-4P-SPEC.md` (deleted at
merge). Survey: `/home/user/scalar-briefs/survey-lane4.md` §7.

**Review tier: SINGLE** (one Opus reviewer) — small, test-only, no
design decision; the risk is a census matcher that looks strict and is
blind, which the red-first rows and a reviewer's own plants check.
