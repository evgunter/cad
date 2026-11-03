---
id: a-spindle-sheet-exports-as-degenerate-toroidal-surface
kind: issue
title: step-export: a Surface::Spindle face refuses UnsupportedSurface; AP214's twin is DEGENERATE_TOROIDAL_SURFACE (|R| < r) or TOROIDAL_SURFACE (horn)
status: open
opened: 2026-11-03
priority: P3
cost: M
---

(SPIRICARM, stage 1 of `horn-and-spindle-tori-have-no-representation`, the consumer rows its ruling names.)

## What

`step-export`'s surface printer (`crates/step-export/src/writer.rs`) refuses a `Surface::Spindle` face with `StepExportError::UnsupportedSurface`. Its AP214 twin is `DEGENERATE_TOROIDAL_SURFACE(|R|, r, select_outer = R > 0)` for `|R| < r` and `TOROIDAL_SURFACE(r, r)` for the horn, the placement the kernel frame as for `Surface::Torus` (the designers' mapping, PR 6464 body; its `v` convention needs checking against ISO 10303-42).

## Done when

A spindle face exports and round-trips through the import (`degenerate-toroidal-surface-imports-as-a-spindle-sheet`, on `exch`'s slate).

