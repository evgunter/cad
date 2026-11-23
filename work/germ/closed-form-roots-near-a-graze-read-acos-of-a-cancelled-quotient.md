---
id: closed-form-roots-near-a-graze-read-acos-of-a-cancelled-quotient
kind: issue
title: Two closed-form conic roots read acos of a quotient that nears one at a graze: the torus parallel arm's contours and the conic-through-the-apex deflated form
status: open
opened: 2026-11-23
---


Found by U4a's sweep (`germ/u4a-local-windows`) for the shape U4a
re-posed in `curve_face::pose::plane_conic` and `pose::arc`: a root or an
arc read as `acos(x)` where `x` nears ±1 at a graze, so the subtraction
inside `1 − x` keeps few of the depth's digits ([simple.conditioned]).

## Sites

- `curve_face::circle_torus::parallel_roots`
  (`crates/geom-brep/src/curve_face/circle_torus.rs`, `spread = c.acos()`):
  a circle parallel to the torus's axis meets a contour at
  `c = (contour² − offset² − ρ²)/(2ρ·offset)`, a law-of-cosines quotient
  that nears ±1 where the circle grazes the contour. It is charged
  (`rounding_charge(contour² + offset² + ρ²)`), which
  `[simple.conditioned]` forbids at `f64`.
- `curve_face::conic_quadric::deflated_roots`
  (`crates/geom-brep/src/curve_face/conic_quadric.rs`,
  `spin = acos(−k/R)`): a conic through a cone's apex, its deflated
  form's roots, where `|k|` nears `R` at a graze of the deflated form.

## Settles when

Each root is read from its own depth with no subtraction from one (the
half-angle form `2·asin(√((1 − |x|)/2))` over a depth formed from the
stored terms, as `plane_conic` now reads `atan2(√(h·(R + |d₀|)), −d₀)`),
with a row per site against a double-double root at depths down to
`10 Kε`, red with the `acos` restored.
