---
id: the-ruling-question-is-read-in-two-units
kind: issue
title: Whether a line on a cylinder is a ruling is read in two places in two units: props meters its tilt over the edge, the mint reads its radial speed per unit parameter
status: open
opened: 2026-11-03
priority: P1
cost: M
---


Filed by PCERT unit E2 (`pcert/cylinder-line-lane`, PR 6470), from its
review (fix list R2).

## What

One question, "is this line a ruling of the cylinder", has two homes,
and they read it in different units:

- **The mint** (`pcurve_cache.rs`, `derive_class`'s `Surface::Cylinder`
  arm, `pcurve_chart_radial_moving`): `radial_amplitude` is
  `σ(a_r, b_r) + ‖l_r‖`, decided against the band with `Margin::of`.
  For a line, `l` is `dir`, so `‖l_r‖` is metres **per unit of the
  line's parameter**: its radial speed. The same line with its `dir`
  scaled reads a different margin, and the reading does not depend on
  the edge's span at all.
- **Props** (`props/curved.rs`, `cylinder_boundary`,
  `props_meridian_axial`):
  `Margin::metered(t1 − t0, InfSpeed::new(‖dir × axis‖))`, the line's
  deviation from the ruling through its start, **in metres over the
  edge**.

On one edge the two can disagree. A unit-speed line tilted `θ` over a
span `L` reads `θ` (dimensionless, compared as metres) at the mint and
`θ·L` at props. With `L ≪ 1` the mint can read the tilt definite (the
general lane, since E2) where props reads it Zero, and with `L ≫ 1` the
mint can read it Zero (the meridian) where props reads it past the band.
Two implementations of one reading is the P1 definition.

## Fix

One home for the ruling reading, metered over the edge's span (props'
form): the mint reads it from the span it is given (`Placement`'s
`span`), and props calls that home. The mint's own-interval reading
(`own_interval`, `[0, 1]` for a line) then needs the span made explicit
at the callers that hold none (`chart_pcurve`, `chart_iso_family`).
Each needs a row with a short and a long edge on one tilt, red today.
