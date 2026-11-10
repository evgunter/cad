---
id: an-iso-arcs-envelope-omits-its-in-band-weight-margin
kind: issue
title: The arc-rim certificate decides its weight margins Zero in band and never pays them into the envelope, so a near-circle row certifies with an envelope far under its miss
status: open
priority: P1
cost: M
opened: 2026-11-04
---

Filed by the PCERT main-reds lane from PR 6528's FULL review (fix EE4). It
predates that PR, which added the per-sub-arc margins but not this
omission.

## What

`run_iso_arc_checks` (`crates/geom-brep/src/pcurve_cache.rs`, check 4's
weight margins) decides each sub-arc's `w − cos(h/2)` as Zero, metered
into metres by the radius (`Margin::metered(margin, InfSpeed::new(radius))`),
and never adds the decided margin to `envelope`. The envelope is
`hull + slack_v + slack_affine + slack_knots`. Its hull term compares
`B` with `Ĉ`, the circle built in `B`'s own weights, so a weight that is
not `cos(h/2)` makes `Ĉ` a conic other than the circle. The hull sees
that only through the control points, and the miss the weight causes is
paid nowhere.

## Measured (the reviewer's probes, `review/6528-full` at `5a1b5e4b88`)

- **The weight's own miss is unpaid.** A one-span quarter row on
  `R = 5 mm` at weights `(1, 1 + 1e-7, 2)` certifies with envelope
  `8.7e-19`. Its true radial miss is `8.6e-11` (mpmath). The margin
  decides Zero in band (ε 1e-9), and the envelope reports a miss eight
  decades under the real one. The same holds at one end weight.
  `geom-brep rev6528_probes::probe_a_near_circle_one_span` certifies at
  every δ up to 1e-7, with the same envelope.
- **The radius meter stops bounding the miss on wide sub-arcs.** By the
  reviewer's derivation, the radial miss of a weight error `δw` is
  `R·δw·(1 − c)/(c(1 + c))` at the sub-arc's middle, `c = cos(h/2)`. The
  meter charges `R·δw`, so it under-reads wherever that factor exceeds 1.
  The factor crosses 1 at `c = √2 − 1`, a sub-arc of about 131.06°
  (checked by this lane), and is 2.27 at 150° and 9.6 at 170°. Confirm
  the factor against an exact sup before building on it.

## Why P1

A certificate whose envelope understates its carrier's miss is a false
bound for any consumer that reads the envelope: the volume's chord
envelopes, the mesher's boundary-tightening contract and the
`unproven-coincidence` lint. No false CLASS follows, because the
margin's band still holds the weight inside ε, and the dm1 and loft rows
sit at an exact weight. So it is not P0.

## Fix

Pay each decided margin into the envelope, converted by the factor
above: `R·|w − cos(h/2)|·max(1, (1 − c)/(c(1 + c)))`, or the exact
sup of the radial miss over the sub-arc. Add a row at `(1, 1 + 1e-7, 2)`
whose envelope must hold the true miss, and one at a 150° sub-arc.

