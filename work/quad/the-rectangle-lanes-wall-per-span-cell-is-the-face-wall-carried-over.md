---
id: the-rectangle-lanes-wall-per-span-cell-is-the-face-wall-carried-over
kind: issue
title: the rectangle lanes' work wall per knot-span cell is the face wall carried over, so a many-span face that cannot meet its target spends spans times it
status: open
opened: 2026-10-30
---


(QUAD per-span lane, `quad/per-span-round-zero`.)

## What

The rectangle lanes lay their cells per knot span, so a round's work is the sum of its span cells'. Their work wall is therefore `FACE_WORK_WALL` once per span cell (`span_work_wall`, `crates/geom-brep/src/props/quad/run.rs`), which is what each per-span tile of FOREIGN's C⁰-knot split gets as a face. The value per span cell is the old face wall, calibrated on whole faces (`lily_leaf_a`), not on span cells.

## Measured

At ε = 1e-9, over the geom-brep, topo, sweep and step-import suites in release, the costliest accepted rectangle-lane face spent 7.6e6 work per span cell (a one-span exact-v face at round 12); the costliest multi-span one 8.7e5 per span cell (6 cells). So the one-span faces converge up to about half the wall, and lowering the per-span value would cost them rounds.

A face that cannot meet its target now spends `span cells × 14.25e6`: `cert5_r2_probes`' 12 + 12-knot rows (`many_dyadic_knots_per_axis_stay_sound`, `many_offgrid_knots_per_axis_stay_sound`; 169 span cells) ran past 120 s each in release at ε = 1e-12, where they stop at the wall, against seconds on main.

## Owed

Recalibrate the wall per span cell against the corpus (the tour's `lily_leaf_a` and the python suite included, which this lane did not run), and decide whether the 1e-12 rows of the many-knot probes should be bounded by a window rather than by the wall. The nightly runs them in the dev profile.
