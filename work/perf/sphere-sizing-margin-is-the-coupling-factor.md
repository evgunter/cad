---
id: sphere-sizing-margin-is-the-coupling-factor
kind: issue
title: the sphere arm holds the doubly-curved coupling factor as SPHERE_SIZING_MARGIN - PERF-1's derivation with R = 0 replaces a margin with a bound
status: open
opened: 2026-09-10
---

## The finding

Reported by the PERF-1 lane (PR 2307), outside its fence. The sphere
arm of `curved.rs` sizes both chart directions off one sagitta step
divided by `SPHERE_SIZING_MARGIN`; PERF-1's torus derivation proves
the doubly-curved coupling factor (2, from the hypotenuse midpoint)
that the margin holds by hand, and a sphere is the torus with R = 0.
A derivation replaces a tuned margin with a bound and may size the
sphere grid tighter or looser than the margin does — measure which.

## What a fix is

Derive per PERF-1 (`torus_grid_steps`'s doc comment is the register),
pin with the same shape of sweep row, re-cut the baseline's sphere
rows. S-MESH territory; numeric; small.
