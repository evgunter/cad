---
id: a-closed-form-image-at-a-fuzz-seed-reads-its-coefficient-bound-in-band-at-eps-1e-12
kind: issue
title: A correct closed-form image reads its coefficient bound in band at ε 1e-12 at fuzz seeds, so the every-spelling row goes red by seed
status: closed
closed: 2026-11-12
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

## Closed

The bound has no excess at these seeds. Each refusal reads a real miss
of the stated image, and the bound is that miss to within a part in a
thousand. The residual's coefficients are exact to about 1e-14, so the
rounding of the coefficient sum, the slack terms (zero on a harmonic),
the phases and centring are each below 1e-13 of what the bound reads.
Two causes:

- **The fixture drew a frame off square** (`0x4661d077090b3597`). The
  plane's `u_ref` leaned 5.35e-14 off its normal, the residue of one
  Gram–Schmidt pass on a `b` near the axis. The chart therefore leaves
  the line, drawn square to the normal, by 5.35e-14 per metre of `u`.
  In rational arithmetic the stated image misses by 1.0181e-12 at the
  span's far end (`u` ≈ 20.7), and the bound reads 1.0195e-12. No image
  on that chart is within ε, so `Escalated` is the honest ending. The
  rows' frames (`closed_image_rows.rs` `frame`/`perpendicular`, and
  `analytic_image_rows.rs` `frame`) now square twice (`across`). Rows:
  `a_plane_line_on_a_frame_drawn_near_its_axis_certifies_in_every_spelling`
  (the seed, pinned) and `a_frame_drawn_near_its_axis_is_square_to_rounding`.
  Mutant `across-once` turns both red.
- **The derivation misses near the pole** (`0x8ffe04db921f5cff`, and
  every refusal of the 2000-seed sweep). The case is a sphere meridian
  whose start lies within about 1e-3 rad of the pole (`v` phase ≡ π/2).
  The derived azimuth reads `â_r` off an `a_r` of about 1e-4·r, so it is
  off by `ulp/‖â_r‖`, here 7.5e-13 rad, a miss of 1.43e-12 (dense
  1.436e-12, bound 1.435e-12). This is
  `a-derived-sphere-image-misses-its-circle-by-1e-11-near-five-half-pi-at-eps-1e-12`'s
  shape, landing in band instead of past it, and is that row's to fix.

Sweep at ε 1e-12, `f64` and `Interval`, the same 2000 seeds before and
after, over every closed-image and analytic-image fuzz: 9 refusals before
(8 `Escalated`, 1 `StatedImageMissed`) and 4 after (all `Escalated`).
All are sphere meridians near the pole, and the count moves with the
`u_ref` bits the fixture now draws. At ε 1e-9 there are none either way.
The every-spelling row stays red by seed at ε 1e-12 until that
derivation lands.

