---
id: ellipse-edge-crossing-a-torus-has-no-root-lane
kind: issue
title: An ellipse edge crossing (or near) a torus face refuses at the frontier - its residual is an octic in the half-angle
status: open
opened: 2026-10-02
priority: P1
cost: H
refs: [non-circle-conic-edge-refuses-against-every-curved-face]
---

Found by the REACH lane that gave the ELLIPSE carrier its clearance
and root lane (`work/reach/non-circle-conic-edge-refuses-against-every-curved-face.md`).

## Measured (by reading)

An ellipse arc that the conic rung's enclosures cannot clear of a TORUS
face falls through to the root arms, and `reduce::wall_crossing`
answers `SpanVerdict::Unsettled` for the ellipse × torus cell, so every
arm refuses `CurvedPierceUnsupported`. A clear ellipse IS cleared (the
torus arm of `geom_brep::conic_arc_residual_range`, its curvature bound
read at the semi-major axis, pinned by
`geom_brep`'s `the_arc_enclosures_hold_a_dense_ellipse_sampling`); a
near or crossing one is not.

## Why there is no lane

Along an ellipse `C(θ) = C₀ + a·û cos θ + b·v̂ sin θ` the torus's
`F = (S + R² − r²)² − 4R²(S − h²)` has `S = |C − c|²` of trigonometric
degree TWO (on a circle the second harmonic of `S` vanishes because
`|û cos θ + v̂ sin θ| = 1`) and `h` of degree one, so `F` has degree
FOUR: an octic in the half-angle, where the circle's is a quartic
(`circle_roots::half_angle_roots`). No certified octic ladder exists in
the tree.

## What a fix has to supply

A certified real-root count for a degree-4 trigonometric polynomial on
an arc — e.g. a Sturm sequence or Descartes/Bernstein subdivision on the
half-angle octic, with the same pole and conditioning discipline the
quartic ladder uses — or a subdivision of the arc into pieces on which
the sampled enclosure and a monotonicity bound (`|F′|` from below)
certify at most one root each. No real shape reaches it yet that is
known: the tilted cut of a torus-walled body would.
