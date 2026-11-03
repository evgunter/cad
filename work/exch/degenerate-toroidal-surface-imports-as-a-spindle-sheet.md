---
id: degenerate-toroidal-surface-imports-as-a-spindle-sheet
kind: issue
title: step-import: DEGENERATE_TOROIDAL_SURFACE has no arm; it is one sheet of a spindle torus, Surface::Spindle with R signed by select_outer
status: open
opened: 2026-11-03
priority: P3
cost: M
---

(SPIRICARM, stage 1 of `horn-and-spindle-tori-have-no-representation`, the consumer rows its ruling names.)

## What

`step-import`'s entity reader (`crates/step-import/src/entities.rs`, the `TOROIDAL_SURFACE` arm) reads both radii verbatim into `Surface::Torus`, and has no `DEGENERATE_TOROIDAL_SURFACE` arm, so a file stating a lemon or an apple refuses as an unsupported entity, and one stating a spindle as `TOROIDAL_SURFACE` lands a `Surface::Torus` that tier 3 refuses (`DegenerateTorus`).

The designers' mapping (PR 6464 body, "likely; check the schema's `v` convention"): `DEGENERATE_TOROIDAL_SURFACE(|R|, r, select_outer)` is `Surface::Spindle` with `R = |R|` when `select_outer` (the apple) and `R = −|R|` otherwise (the lemon); a horn (`R = r`) is a `TOROIDAL_SURFACE` and becomes `Surface::Spindle` with `R = r`. The `u_ref`/`v` conventions need checking against ISO 10303-42.

`step-import`'s `adopt` and `chart` carry `Surface::Spindle` arms already (a signature tag; `u` periodic, `v` closed).

## Done when

The import reads both entities onto the kind they state, pinned by a file of each.

