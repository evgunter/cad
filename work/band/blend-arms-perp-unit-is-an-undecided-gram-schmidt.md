---
id: blend-arms-perp-unit-is-an-undecided-gram-schmidt
kind: issue
title: blend arms' perp_unit is a hand Gram-Schmidt on an undecided normalize
status: open
opened: 2026-10-01
---


## What

`crates/sweep/src/blend/arms.rs` `perp_unit(x, a)` is a Gram–Schmidt
step written by hand — `(x - a * a.dot(x)).normalize()` — whose
residual length is never decided. Its doc says poison propagates when
`x` is parallel to `a` ("the honest answer"), and that is D2 row 3 for
a value; but the result is the seam reference of every chart built
below it, so a residual that is merely SHORT (inside the band, not
exactly zero) normalizes to a definite direction made of rounding,
and nothing downstream can tell.

## Shape

`geom_core::OrthoFrame::from_aim_and_reference` (or `UnitVec3::new` on
the residual) is the ladder written once: the residual's length
decided under a K name the blend lane owns, refusing typed
(`OrthoFrameError` naming `OrthoAxis::V`) where it decides zero or
escalates in band. Found by the `linalg/decided-not-minted` sweep for
the hand Gram–Schmidt shape (`- a * a.dot(x)).normalize()`); the
`linalg` branch fixed the same shape in `geom-brep`'s
`torus_meridian_orient`.
