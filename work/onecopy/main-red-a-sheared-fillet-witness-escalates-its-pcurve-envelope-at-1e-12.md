---
id: main-red-a-sheared-fillet-witness-escalates-its-pcurve-envelope-at-1e-12
kind: issue
title: main red at 1e-12: band_planar_cut_off_interval's atan2-cut fillet witness (shear 0.6, concave) escalates its pcurve envelope at Interval
status: open
opened: 2026-11-04
priority: P0
cost: M
---


Found by `onecopy/meet-interval` (PR 6516) in its fix pass. It is red at base `354f9b3f` and at PR head `ed74313f07`, with the same row.

## What

`crates/sweep/tests/band_planar_cut_off_interval.rs:336` `a_fillet_foot_on_atan2s_cut_at_an_oblique_end_carves_at_the_certified_scalar` fails at `CAD_TOLERANCE_EPS=1e-12` on `shear 0.6, convex false`. The `Interval` build escalates its pcurve envelope.

The test's own doc (lines 320–334) admits that escalation at 1e-12 for a *control*, a negative-shear row, and never for a *witness*. Shear 0.6 is a witness: the fillet's foot is the end ellipse's major vertex, on `atan2`'s cut.

The lane says this is not the polish meter, PR 6516's subject. PR 6516 removes the test's `tier_3_prime` admission, because that meter no longer fires; this row is untouched by that.

## Owed

1. Reproduce it on main at 1e-12.
2. Find out why the witness's pcurve envelope escalates.
3. Fix it at its root, either by re-posing the read that loses digits or by correcting the admission's premise. Do not admit the witness.
