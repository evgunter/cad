---
id: a-curved-spline-carrier-or-a-non-affine-line-ends-its-general-image-as-a-kernel-bug
kind: issue
title: A curved spline carrier, or a line whose foot path is not affine in t, ends its degree-1 General image's refusal with a tolerance offer and the kernel-bug ending
status: open
opened: 2026-11-10
priority: P1
cost: M
---


Filed by `pcert/curved-general-image` (PR 6532), from its review's FF4.
The same on main.

## What

The mint's `General` producer (`pcurve_cache::general_image_lane`) climbs
`edge_nurbs::curved_chart_image`'s rungs only for a carrier that
`bends`: curved and not a spline. Every other carrier takes the degree-1
image of 33 feet (`edge_nurbs::chart_image`), which certifies only where
the foot path is affine in the carrier's parameter to ε, and its refusal
keeps its own term, a tolerance offer with the kernel-bug ending, against
D3. Two carriers reach it through `nurbs_iso_derive`'s `iso_schedule`
(`RowSpace::of` offers a spline a row only in an equal knot space), then
`general`:

- **A curved spline carrier**: the rational one-span arc of a ring's
  circle on that ring. `FittedLimb { HullValueChart }` 2.8e-5 m.
  Probe: `review_6532_curved_spline_carrier_ending`
  (`crates/geom-brep/tests/pcurve_general.rs` on `review/6532-full` @
  `5ca8d9f6e5`).
- **A line on a flat wall whose `v` is not affine in height** (control
  rows at `z = 0, 0.1, 1`). `FittedLimb` 5.8e-4 m. Probe:
  `review_6532_line_on_a_nonuniform_wall_ending`, same file.

## Why not fixed in PR 6532

A spline's degree-1 image is the bits its plane × NURBS edge certificate
reads (`edge_nurbs::chart_image`'s "two consumers, one producer"), and a
line's foot path is straight on every chart the shipped rows mint. Moving
either onto the curved rungs splits that producer, or reads the degree-1
image's miss first and climbs only where it misses. Both are wider than
the unit, which widened the producer to bending carriers alone.

## Fix direction

Climb on the measured miss, not the carrier's class: take the degree-1
image, read its miss between feet as `curved_chart_image` reads a rung's,
and climb the curved rungs where it exceeds the rung's share of the band.
The relabel (`minted_refusal`, `LaneImage::General`) then keys on the
image the mint produced rather than on `bends`. The edge certificates
that read the same degree-1 image are ssiedge's
`plane-nurbs-limb-two-refuses-a-non-row-section`.
