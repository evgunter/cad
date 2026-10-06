---
id: a-rotated-cylinder-at-1e3-scale-refuses-its-rigid-move-at-1e-12
kind: issue
title: transform_rigid of a 1e3-scaled cylinder under a 0.7 rad skew rotation refuses on the pcurve envelope at ε 1e-12
status: open
opened: 2026-10-03
priority: P3
cost: M
refs: [boolean-door-adopts-the-finished-body-type]
---


Found by PR 3987's dual review (lane `reach-dual3987-r1`,
`probes/review3987_e2e.rs` on `analysis/reach-dual/3987-r1`), and
pre-existing: it is a fixture-side move, before any boolean.

The probe extrudes a circle into a cylinder, scales the scene by 1e3,
and moves it with `topo::transform_rigid` (`crates/topo/src/transform.rs`)
by a rigid map with a 0.7 rad skew rotation. At ε 1e-12 the move refuses
on the pcurve envelope; at 1e-9 and 1e-6, and at scales 1e-3 and 1, it
moves. A rigid map preserves every distance, so a body valid before it
is valid after it; a refusal is the re-certification's, and whether its
envelope is read in metres at the scaled radius is the question. The
probe skipped that pose. Reproduce with the probe's `cyl` and `moved`
at `s = 1e3`, `CAD_TOLERANCE_EPS=1e-12`.
