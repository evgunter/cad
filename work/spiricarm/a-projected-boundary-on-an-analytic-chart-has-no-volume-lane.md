---
id: a-projected-boundary-on-an-analytic-chart-has-no-volume-lane
kind: issue
title: A cylinder or sphere face bounded by a spline chord (its pcurve a Projected image on the analytic chart) has no volume lane, so every cylinder x sphere boolean that builds refuses at the result gate
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

That is the cylinder's arm. A sphere face bounded by a chord refuses
in the same function before it, on the kind: the lane's chart-normal
flux algebra is the cylinder's (`what: "conic trim on a
cone/sphere/torus chart — ..."`). And a sphere face that holds a
section loop as a ring refuses one step earlier still, in
`topo::props::face_flux_about` (the ring gate before the face lanes):
the sphere reads rings only where circles bound it
(`MassPropsError::RingOnCurvedFace`).

`crates/sweep/tests/cylinder_sphere_frame.rs` pins where each of its
54 runs ends: 14 at the cylinder's arm (`Door::Projected`), 10 at the
sphere's (`Door::Sphere`), 4 at the ring gate (`Door::Ring`, the
two-loop poses' `B ∪ A` and `B ∖ A`), and the other 26 earlier, at the
spline edge join (`splice/joining-a-spline-carrier-is-unbuilt`), which
reach this door next.

## What it needs

A Green-form boundary integral over a projected image on an analytic
chart, the cylinder's and the sphere's first (the sphere a lane of its
own, with its rings): the image is `ψ(C(t))`, the chart map of a
spline carrier, so the integrand has no closed form in the image's own
channels and the lane needs a certified quadrature of it over the
carrier's knot spans, as the fitted lanes on a spline chart have. The rows go SOUND against the fixture's volume oracle once it
lands and the join does.

## Four door rows wait on it too (SECT `sect-seamslow`, 2026-10-30)

The seam-region door rows
(`crates/sweep/tests/the_door_reads_seam_regions_as_tier_3_does.rs`)
build a ball's booleans with three cylinders. On main `25d3906bca` all
20 that pass the join refuse at the result gate's volume or before it:
9 at a curved-cut half-edge with no harmonic form (a projected image),
5 at a conic trim on the sphere's chart, 4 at `RingOnCurvedFace`, and 2
at the spline edge join (`JoinCarrierUnsupported`). The four rows
holding them (`…_against_the_rod`, `…_the_unit_ball_against_the_turned_rod`,
`…_the_tilted_ball_against_the_turned_rod`, `…_against_the_stub`) log
`VACUOUS` and assert nothing until those bodies build; they start
reading the door on chord-bounded sphere faces when this lane lands.
