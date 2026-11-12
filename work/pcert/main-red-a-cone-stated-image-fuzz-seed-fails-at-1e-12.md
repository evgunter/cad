---
id: main-red-a-cone-stated-image-fuzz-seed-fails-at-1e-12
kind: issue
title: Main red: analytic_image_on_its_carrier seed 0xbc20d21ae8add37e fails a cone's General stated image at eps 1e-12
status: open
opened: 2026-11-12
priority: P1
cost: M
---


Filed by the `sectarms/join-reads-spline-implicit` lane (PR 6862), whose
gate drew the seed. Not this lane's change: the same seed fails with
main's `crates/geom-brep/src` checked out.

## What

`crates/geom-brep/src/pcurve_cache/analytic_image_rows.rs`
`a_stated_image_on_its_carrier_certifies` (the fuzzer
`analytic_image_on_its_carrier`) panics at its per-case assertion
(line 561) at `CAD_TOLERANCE_EPS=1e-12` for

    CAD_FUZZ_SEED=0xbc20d21ae8add37e

on a cone's `General` image, a degree-1 `NurbsCurve2` with interior
knots `0.6698164519876383, 0.6718021384325337`. Replay:

    CAD_FUZZ_SEED=0xbc20d21ae8add37e CAD_TOLERANCE_EPS=1e-12 \
      cargo test -p geom-brep --lib \
      pcurve_cache::analytic_image_rows::a_stated_image_on_its_carrier_certifies

Not traced. `a-correct-stated-image-far-from-the-origin-reads-in-band-at-a-fine-band`
is the nearest known 1e-12 refusal of a correct stated image; whether
this is the same class (a cone's chart coordinates rounding past a
1e-12 band) is unchecked. The genuine counterexample should be pinned
beside its fix.
