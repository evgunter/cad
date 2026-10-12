---
id: main-sweep-loft-does-not-compile-after-the-scalar-knots-merge
kind: issue
title: main does not compile: sweep's lift_loop_walls builds a NurbsSurface<T> from f64 knots and weights after #4591 moved them into the scalar
status: closed
pr: 4700
closed: 2026-10-11
priority: P0
cost: E
opened: 2026-10-11
---


Found by the FLUX cone-vector-area lane (PR 4696), 2026-10-11. Main at
`94fa7e507` does not compile: `cargo check -p sweep` fails in
`crates/sweep/src/loft.rs`'s `lift_loop_walls` (E0308, E0277). The
function lifts an f64 wall's control net to `T` and rebuilds the wall
with `NurbsSurface::new(w.knots_u().clone(), w.knots_v().clone(), net,
w.weights().to_vec())`; since #4591 (`intent/from-f64-sweep`) holds
knots and weights in the scalar, those arguments are `f64` and the
surface comes out `NurbsSurface<f64>`. A semantic conflict between
`73e852f2a` (the shared seam column) and #4591; every PR's gate is red
on its merge with main until it lands.

Partial fix, checked in a main-only worktree: lift the wall first,
`let w = w.map_scalar(T::from_computed);`, and pass
`w.knot_values_u().clone()`, `w.knot_values_v().clone()`,
`w.weights().to_vec()`. That leaves one error: `NurbsSurface::new`
sits in `impl<T: SpanLocate>`, so `lift_loop_walls` (and its callers up
`loft`'s generic chain) need `T: SpanLocate`, or the rebuild goes
through a constructor that does not re-validate. Which is this
program's call. (FLUX cone-vector-area lane)

Closed: PR 4700 (`flux/main-compiles-sweep-loft`) fixed it — the
`SpanLocate` bound is free, since loft's one caller is `T: Decide` and
`Decide: SpanLocate`.
