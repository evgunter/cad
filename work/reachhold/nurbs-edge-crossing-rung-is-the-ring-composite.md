---
id: nurbs-edge-crossing-rung-is-the-ring-composite
kind: unit
title: a NURBS operand edge gets one crossing rung keyed on the carrier: the spline::compose ring composite
status: parked
opened: 2026-10-03
priority: P1
cost: H
blocked_on: [cylinder-sphere-germ-pair-has-no-join-lane]
refs: [3984]
---


Decided by the REACH orchestrator on the NURBS/spiric operand fork,
adopting both designers' reconciled reports
(`analysis/design-fork/nurbs-spiric-operands-d1` and `-d2`,
`design.md`, round 1). The sweep's two crossing arms refuse a NURBS
edge typed (`BooleanError::CrossingCarrierUnsupported`, PR 3984), and
`delete-the-boolean-operand-edge-gate` retires the body-scoped gate in
front of them; this rung is what then lets such an edge through.

## What to build

One crossing rung for a NURBS edge, keyed on the carrier, not one root
lane per face kind. `geom_core::spline::compose` already builds the
rational Bernstein composite `f ∘ C` of a plane, sphere, cylinder, cone
or torus residual along a NURBS curve, in certification arithmetic,
with certified coefficient hulls per span (it serves `ssi::certify`
today).

- **Clearance**: the numerator hull one-signed over the span (the
  denominator is positive by the weight invariant).
- **Roots**: knot insertion in certification arithmetic
  (`insert_knot_plan`, `apply_certified`) until a piece is one-signed
  (clear), or changes sign with a one-signed derivative hull
  (`derivative_coeffs`: one root, bisected on the residual and
  confirmed ON the surface), or shrinks into the band (a tangency,
  refused typed). It is the clear / monotone / split ladder
  `circle_roots::certified_subdivision` runs, with hulls for Taylor
  bounds.
- The plane case is the linear composite and replaces the planar
  lane's `Unlaned` refusal; the curved arm's the same.
- The edge's box becomes its control hull (`nurbs_curve_aabb`), the
  obligation `boxes::EdgeBoxRule::NoSoundBox`'s docs defer; the
  continuation scan's unreadable face extent retires with it.
- The join's germ frame and ring run and the vertex sector walk each
  need their NURBS arm too
  (`join-and-continuation-sites-blame-the-edge-gate-for-a-spline-edge`).

## Why it waits

Its first consumer is frontier (d)'s cylinder×sphere join window
(`join/cylinder-sphere-germ-pair-has-no-join-lane`; the pair's frame
landed in PR 4025, and the window is the lane), which will
mint fitted NURBS seams on analytic faces. From then on DESIGN's
"every boolean output is a legal boolean operand" (the maximal-faces
paragraph) is owed, and the gate must be gone by then. Before (d)
there is no caller: the corpus's STEP
files carry no NURBS edge, and every loft or sweep operand refuses on
a NURBS face first (d1's measured table). Build it with (d), as (d)'s
consumer-side half.
