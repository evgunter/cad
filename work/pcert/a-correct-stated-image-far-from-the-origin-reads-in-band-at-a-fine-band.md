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
