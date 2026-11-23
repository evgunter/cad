---
id: a-surface-of-revolution-is-refused-at-import
kind: issue
title: STEP import refuses every SURFACE_OF_REVOLUTION as an unsupported entity, so a spheroid the kernel writes cannot be read back
status: open
opened: 2026-11-22
priority: P3
cost: M
---



Filed by PROFILE's cut of the ovoid build (PR 7289), 2026-11-22.

## What

`step-import` has no reader for `SURFACE_OF_REVOLUTION`
(`git grep SURFACE_OF_REVOLUTION crates/step-import` is empty), so it
falls to `StepImportError::UnsupportedEntity` (`crates/step-import/src/entities.rs`).
`docs/STEP-BANK.md` lists two bank files that carry the entity
(`framework/FW13_camera_module.stp` and `freecad/Schenkel.stp`, four
each).

It matters more once OVOID's `a-spheroid-writes-exact-step-as-a-surface-of-revolution`
lands: the kernel will write a spheroid as `SURFACE_OF_REVOLUTION` over
an `ELLIPSE`, and its own file will not read back.

## The fix, as far as it is written

Read the entity, and recognise the cases that are kinds (a line or
circle swept about an axis is a cylinder, cone, sphere, torus or
spindle sheet; an ellipse centred on the axis with a principal
direction along it is a spheroid); the rest is an exact rational NURBS
or a typed refusal.
