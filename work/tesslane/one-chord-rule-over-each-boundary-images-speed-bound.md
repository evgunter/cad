---
id: one-chord-rule-over-each-boundary-images-speed-bound
kind: unit
title: the chord pass tightens every curved face's boundary by one rule - an image's speed bound per chart axis over the face's lattice step
status: spec
opened: 2026-11-14
priority: P0
cost: H
---


Unit 1 of the chart lane (`work/tesslane/plan.md`, The chart lane).

## What it is

A curved face's boundary is chorded finely enough for that face's lattice by one rule: on each half-edge, the chord count is at least the stored image's speed along each chart axis, times the parameter span, over the face's lattice step on that axis. The image's own facts sit with the image.

- **Three facts per image form**, beside `Pcurve::eval` in `geom_brep` (`pcurve_cache.rs`): its value at a parameter, a box over a parameter span, and a speed bound per chart axis over a span. Today `closed_form_span_box` answers `Harmonic` and `FocalSection` only, and the speed bounds live in `mesh::chords` for four forms (`Harmonic`, `IsoLine`, `IsoArc`, `General`) plus the sphere's projected conic (`sphere_uv_speeds`). Owed, each in closed form with its derivation and a row that pins it against the sampled image: `Spiric` on a torus, `FocalSection` on a cone, and `Projected` off the sphere.
- **One tightening door in `mesh::chords`**, replacing the circle arm of `sizing::grid_boundary_step`, `sphere_arm_tighten`, `nurbs_tighten` and `edge_target`'s headroom read. `chords.rs`'s header lists those doors and says nothing enforces the list; the list goes.

## What it is not

No new arm in `trimmed::Lane`, no ring, no routing: the faces that refuse today still refuse, at the same sites. A mesh that moves is re-baselined with its cause named; that nothing moved is never the argument for the change.

## Before it is specced further

Read `crates/mesh/README.md` `[mesh.sizing]`'s rim-chord bullet: it names today's three mechanisms. The rule it states (a rim chords at its face's grid step) is what this unit implements once; the bullet's list is re-worded with the code.

`pcurve_cache.rs` is not this program's ground; the seam is announced on its owner's log at dispatch.
