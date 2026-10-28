---
id: a-projected-boundary-on-an-analytic-chart-has-no-volume-lane
kind: issue
title: A curved face bounded by a spline chord (its pcurve a Projected image on an analytic chart) has no volume lane: the measure refuses QuadratureUnsupported, so every cylinder x sphere boolean that builds refuses at the result gate
status: open
opened: 2026-10-28
priority: P1
cost: M
---


Found by SECT step 1b (`sect/cylinder-sphere-chord`, 2026-10-28), the
lane that cuts a cylinder × sphere boolean's chords from the section's
fitted carrier (`cylinder-sphere-germ-pair-has-no-join-lane`).

## Measured

The chords on the drum's wall and on the ball are arcs of a fitted
NURBS carrier, so each face's pcurve of them is the projected image
(`Pcurve::Projected`, minted and certified at the edge door). The
result gate's measure then reads the face's boundary through
`topo::props::quad_lane` (the curved-cut face lane), which takes each
half-edge's image channel by channel out of its harmonic form
(`geom_brep::Pcurve::harmonic_form`) and refuses every other image:

```
ResultInvalid { errors: [VolumeUncomputable { solid, source: Face { face,
  source: QuadratureUnsupported { what: "curved-cut face half-edge carries
  a pcurve with no harmonic form on an analytic chart (a FITTED or
  PROJECTED image, ...) — its Green-form boundary integral is not wired" } } }] }
```

A sphere face that holds a section loop as a ring refuses one step
earlier on the same ground, in `topo::props::face_flux_about` (the ring gate before the
face lanes): the sphere reads rings only where circles bound it
(`MassPropsError::RingOnCurvedFace`).

`crates/sweep/tests/cylinder_sphere_frame.rs` pins where each of its
54 runs ends: 24 at the face lane above (`Door::Volume`), 4 at the
ring gate (`Door::Ring`, the two-loop poses' `B ∪ A` and `B ∖ A`), and
the other 26 earlier, at the spline edge join
(`splice/joining-a-spline-carrier-is-unbuilt`), which reach this door
next.

## What it needs

A Green-form boundary integral over a projected image on an analytic
chart (cylinder and sphere first): the image is `ψ(C(t))`, the chart
map of a spline carrier, so the integrand has no closed form in the
image's own channels and the lane needs a certified quadrature of it
over the carrier's knot spans, as the fitted lanes on a spline chart
have. The rows go SOUND against the fixture's volume oracle once it
lands and the join does.
