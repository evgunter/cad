---
id: a-correct-stated-image-far-from-the-origin-reads-in-band-at-a-fine-band
kind: issue
title: A correct stated image on a plane or ruling a kilometre from the origin reads in band at ε 1e-12, where the samples alone certified it
status: open
opened: 2026-11-11
priority: P3
cost: M
---



Filed by `pcert/analytic-image-fidelity` (PR 6836), from its review's
far-image probe.

## What

A correct stated image (an iso line or a degree-1 curve in UV) on a
plane, or on a cylinder's ruling, stated against its own polyline or the
line it is, certifies at ε 1e-9 and 1e-6 from 100 m to 10 km out, but
at ε 1e-12 it reads in band at 1 km and 10 km: 9 of the 24 reads of
`pcurve_cache/analytic_image_rows.rs`
`a_correct_image_far_out_certifies_or_ends_in_its_lever` end
`Escalated { StatedImage }` on the composite's `ssi_hull_sup_chart`
(f64, margin ~1.01e-12 at 1 km). They end in the lever, so nothing
false is certified and no miss is claimed. On main the same reads
certified, because the image was metered at the nine samples alone:
this is a false refusal the bound over the span introduced.

## Trace

- `pcurve_cache.rs` `fitted_lane`, the `FittedChart::Patch` arm: the
  patch's corners are the chart evaluated in interval arithmetic, and
  the composite (`ssi::certify::chart_round`) reads the image's chart
  coordinates (hundreds of metres) against a patch window as wide.
  Their rounding is a few ulps of the coordinates, ~1e-13 m at 1 km,
  and the composite's tensor products grow it past a 1e-12 band.
- The composite already shifts its spatial channels to the carrier's
  first control (`surface_curve_residual`'s centre-shift); the chart
  channels are not shifted.

## Class

Every patch read whose chart coordinates are large against the band:
planes and rulings far from their chart's origin. At ε 1e-12 the same
1 km poses also reach the endpoint pins' and the net's incidence
read's own rounding first (the row records those as outside the stated
route).

## Fix direction

Write the patch in coordinates local to the image: shift `u, v` by the
image box's centre (exact in `f64` structure) before the composite, so
the window and the image's chart channels are small, and fold the
shift into the corners.

## Evidence from `pcert/closed-form-image-bounded`

The same class reaches the two doors that PR adds, at ε 1e-12 and only
there, by the size of the image rather than its distance alone. Each
read ends in its lever (or the composite's gap), never as a miss and
never certified:

- `pcurve_cache/closed_bound.rs`, a focal section's residual: a cone
  section about 25 m across, stated by its derived image, reads
  `pcurve_closed_bound` at 1.8e-12 at `Interval`. The bound sums the
  enclosures of about thirty coefficient slots of a residual whose terms
  are tens of metres, then divides by `(1 − |β|)²`.
- `pcurve_cache/patch.rs`, the enclosing patch: a torus meridian or
  parallel 8–10 m from the origin, stated as an iso line, reads
  `ssi_hull_sup_chart` at 1.2–3.7e-12 (f64 and `Interval`). A sphere
  meridian there ended `NoGeneralLane { Envelope }`. The patch's own
  controls are 4e-14 wide; the composite's tensor products at degree
  6 + 6 take the rest.

The rows that read them
(`closed_image_rows.rs` `a_spiric_or_focal_section_on_its_carrier_certifies`,
`a_curve_in_uv_on_its_carrier_certifies_on_a_curved_chart`) admit the
lever at bands finer than `1e-10` and nothing else. No product row
reaches either door's fine-band edge: the product's census through that
PR found no stated focal section and no curve in UV off its ruling, and
its harmonic and spiric reads certify at ε 1e-12.

The fix direction above holds for the enclosing patch too (centre the
chart coordinates and the spatial ones on the image). For the closed
forms, centre the residual's spatial terms on a point of the carrier,
which removes the distance from the origin though not the image's own
size.
