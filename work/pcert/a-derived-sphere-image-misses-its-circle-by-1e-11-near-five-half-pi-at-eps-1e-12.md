---
id: a-derived-sphere-image-misses-its-circle-by-1e-11-near-five-half-pi-at-eps-1e-12
kind: issue
title: chart_pcurve_over derives a sphere image that misses its circle by 1.05e-11 near t = 5π/2 at ε 1e-12, so the every-spelling row goes red by seed as a shown miss
status: open
opened: 2026-11-12
priority: P1
cost: M
refs: [a-closed-form-image-at-a-fuzz-seed-reads-its-coefficient-bound-in-band-at-eps-1e-12]
---

Filed by `pcert/cone-image-fine-band`, from PR 6857's FULL review
(fix list UU2). It was split out of
`a-closed-form-image-at-a-fuzz-seed-reads-its-coefficient-bound-in-band-at-eps-1e-12`
because it has a different shape: here the derived image is not
accurate, and the bound is fine.

## What

`pcurve_cache/closed_image_rows.rs`
`a_closed_form_on_its_carrier_certifies_in_every_spelling`, at ε 1e-12,
seed `0x6c321abd1c3c3bb8`:

- The carrier is a circle of radius 2.13 about 4.6 m from the origin.
- Its image on a sphere is derived by `pcurve_cache.rs`
  `chart_pcurve_over`: a harmonic with `u` phase 2.678 and `v` phase
  −10.9955, winding 1.
- The span is `(6.681, 12.964)`.

At f64 the read ends
`StatedImageMissed { sample: NOT_A_SAMPLE, margin: 1.0493e-11 }`. That
is a located miss past the escalation edge, not an in-band read. The
review's probe shows the derived image really misses its carrier by
about 1.05e-11 near t ≈ 7.85 (≈ 5π/2), and the mint refuses the same
image. Nothing false is certified: the certificate is right, and the
derivation is what is wrong at this band.

    CAD_TOLERANCE_EPS=1e-12 CAD_FUZZ_SEED=0x6c321abd1c3c3bb8 CAD_FUZZ_EFFORT=1 \
      cargo nextest run -p geom-brep --lib -E 'test(a_closed_form_on_its_carrier_certifies_in_every_spelling)'

It fails the same way on main at `9550b98e39`, and with PR 6857, which
does not touch the derivation.

## Why P1

The row asserts that a derived image certifies, and the seed is drawn
per run, so main goes red by seed. No change to the bound can make it
green, because the miss is real. The fix is in the derivation's
accuracy near `t ≡ π/2 (mod 2π)` on a sphere, or else the row has to
admit that a derived image may be refused at ε 1e-12 (as a miss the
mint also refuses, never as certified).
