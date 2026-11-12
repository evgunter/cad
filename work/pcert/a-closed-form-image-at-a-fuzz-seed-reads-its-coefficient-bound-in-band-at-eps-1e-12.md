---
id: a-closed-form-image-at-a-fuzz-seed-reads-its-coefficient-bound-in-band-at-eps-1e-12
kind: issue
title: A correct closed-form image reads its coefficient bound in band at ε 1e-12 at fuzz seeds, so the every-spelling row goes red by seed
status: open
opened: 2026-11-12
priority: P1
cost: M
refs: [a-correct-stated-image-far-from-the-origin-reads-in-band-at-a-fine-band]
---


Filed by `pcert/cone-image-fine-band`, from its class sweep of the
image fuzzes at ε 1e-12. The sweep ran 400 seeds through each
closed-image fuzz.

## What

`pcurve_cache/closed_image_rows.rs`
`a_closed_form_on_its_carrier_certifies_in_every_spelling` asserts that
a correct closed-form image certifies. At ε 1e-12, 2 of 400 seeds end
`Escalated { StatedImage, sample: NOT_A_SAMPLE }` on
`pcurve_closed_bound`, at `f64`, with every other reading certified:

- `0x4661d077090b3597`: a plane's derived harmonic image of a line
  about 10 m from the origin, over `(4.80, 10.13)`. The bound reads
  1.0195e-12.
- `0x8ffe04db921f5cff`: a sphere's derived harmonic image, on the
  mirror, of a circle of radius 1.9 about 18 m from the origin, over
  `(3.82, 8.26)`. The bound reads 1.4349e-12.

To reproduce:

    CAD_TOLERANCE_EPS=1e-12 CAD_FUZZ_SEED=0x4661d077090b3597 CAD_FUZZ_EFFORT=1 \
      cargo nextest run -p geom-brep --lib -E 'test(a_closed_form_on_its_carrier_certifies_in_every_spelling)'

Both seeds fail the same way on main at `9550b98e39` and with
`pcert/cone-image-fine-band`, which does not touch the closed-form
door.

## Why P1

The seed is drawn per run, like the cone row's
(`a-cone-stated-image-at-a-fuzz-seed-reads-in-band-at-eps-1e-12`), so a
PR whose diff puts geom-brep in the ε 1e-12 step can go red here by
chance.

- PR 6857's FULL review ran its own 400 seeds and found 7 refusing,
  about 1 in 57, all identical on main. This branch's 400 seeds found
  the 2 above.
- Each refusal of the bound's kind ends in its lever (`Escalated` on
  `pcurve_closed_bound`), so nothing false is certified.
- One of the review's seeds, `0x6c321abd1c3c3bb8`, is a different shape.
  It ends `StatedImageMissed` because the derived image really misses,
  so no fix to the bound turns it green. It is filed as
  `a-derived-sphere-image-misses-its-circle-by-1e-11-near-five-half-pi-at-eps-1e-12`.

## Regime

This is the closed-form half of the P3
`a-correct-stated-image-far-from-the-origin-reads-in-band-at-a-fine-band`.
That row's evidence section records a focal section at 1.8e-12. The
bound sums the enclosures of the residual's coefficient slots, whose
terms are the carrier's coordinates, about 10–18 m here. That P3
records the closed-image rows that admit the lever at bands finer than
1e-10, but this row admits nothing but `Ok`. So either the row admits
the lever at ε 1e-12, as its siblings do, or the bound centres the
residual's spatial terms on a point of the carrier, as that P3's fix
direction says. Centring is the preferred fix, because a false refusal
10 m from the origin is not the far-image regime.
