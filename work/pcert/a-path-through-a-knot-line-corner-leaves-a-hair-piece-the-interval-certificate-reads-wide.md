---
id: a-path-through-a-knot-line-corner-leaves-a-hair-piece-the-interval-certificate-reads-wide
kind: issue
title: A bending rim's path through a corner of two knot lines leaves a hair-wide image piece, which the Interval certificate reads 0.15 m wide
status: open
opened: 2026-11-11
priority: P2
cost: M
---



Filed by `pcert/curved-general-image` (PR 6532, delta review N1, KK6).

## What

`edge_nurbs::curved_chart_image` breaks a bending carrier's image at
each crossing of a wall knot line (`crossings`, `pieces`, near
`crates/geom-brep/src/edge_nurbs.rs:3145` and `:3192`). Breaks are kept a
quarter step clear of the domain's ends, but not of each other. A path
through a corner of two knot lines crosses both at one parameter, so
two breaks land a hair apart, about 2e-13 of the domain. The image then
holds a knot span that wide.

At `f64` the image certifies. At `Interval` the certificate evaluates the
image at a sample whose `t` enclosure is wide compared with that span,
and reads the map residual as `[0, 0.154]` m (`Escalated { MapResidual,
sample 4 }`). The rim ends "not yet", which is honest but incomplete,
and it does so at every ε.

## Witness

The reviewer's probes on branch `review/6532-delta` (`3c9199c47d`,
`crates/geom-brep/tests/pcurve_general.rs`, `review_6532_delta_*`):

- The ring wall's `v` direction is split at `z = 0` by a degree-1 knot
  (`review_ring_v`).
- The ellipse `z = 0.4·(x − cos 0.6)` crosses `u = ½` and `z = 0` together
  at `t = 0.6`.
- Case "corner, between feet": span `(0.05, 1.15)`.

## Tried in PR 6532, and why neither landed

- **Merging breaks closer than the clearance** (the quarter-step rule
  between breaks). This avoids the hair span, but it drops the second
  kink inside a piece. Lifted `dz = 1e-9` off the corner, the f64
  certificate then reads the hull in band: margin 1.06e-9 against
  ε 1e-9 (`FittedEscalated { Limb(HullValueChart) }`). With the chord
  variant below, which keeps both breaks, it certified at f64 at
  ε 1e-9 and 1e-6.
- **Imaging a short piece by the chord between its breaks.** This
  replaces the fit through projected feet a rounding's width apart. The
  Interval reading is unchanged at 0.154 m, so the width comes from the
  span itself, not from the piece's control points.

## Fix direction

Either an Interval evaluation that reads a hair span without the `t`
enclosure's width blowing up (an enclosure of the image over a span
rather than through de Boor's ratios), or a break placement that keeps
both kinks while giving them a span the certificate can read, such as
one break at the corner and a carrier reparameterization over the
hair.
