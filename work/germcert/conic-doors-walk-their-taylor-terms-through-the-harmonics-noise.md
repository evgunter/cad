---
id: conic-doors-walk-their-taylor-terms-through-the-harmonics-noise
kind: issue
title: The ellipse x torus and conic x quadric doors walk their Taylor terms through the harmonics' noise, which grows like the carrier's size to the residual's degree
status: open
opened: 2026-10-14
priority: P3
cost: M
refs: [circle-torus-roots-refuse-large-circles-for-want-of-recentering]
---


Found by the sweep of the circle × torus recentering fix (branch
`germ/circle-torus-recentering`). That fix gave
`geom_brep::half_angle::certified_subdivision` a jet
(`SubdivisionFrame::jet`): `F` and its first three derivatives read at
the carrier point, each charged its own rounding, so the harmonics'
noise enters only the global `|F⁗|` bound. Only the circle × torus door
hands it one. Every `SubdivisionFrame` is constructed with a `jet:`
field, so this list is complete:

| site | noise it walks through | disposition |
|---|---|---|
| `topo::boolean::circle_torus::circle_torus_roots` | `rounding_charge(h.terms)`, `~ρ⁴` | fixed (`geom_brep::conic_torus_jet`) |
| `topo::boolean::ellipse_torus::torus_walk` | `rounding_charge(h.terms)` from `conic_torus_harmonics`, `~ρ⁴` | **open**: the same shape; `conic_torus_jet` already reads any conic (its unit row covers ellipses and a skewed frame), so the fix is `jet: Some(&…)` there, plus an oracle sweep of ellipses against the torus at the three ε |
| `topo::boolean::conic_quadric::ladder_frame` (a conic tilted to a wall, an ellipse against a sphere) | `rounding_charge(h.terms) / h.per`, `~ρ²` | **open**: the ρ = 1500 circle tilted to a unit wall of `circle-torus-meters-accept-an-unreadable-reading` is this pose; a quadric jet (`S`, `S′`, … of `|⊥q|²`) would read it at the point. Unmeasured how many refusals it would turn |
| `topo::boolean::section_cert::ruling` (`roots`) | `rounding_charge(t1² + 4·t2·t0)` | not measured: a ruling's quadratic, whose terms are the chart's own; no evidence of a far-centred expansion |
| `geom_brep::curve_pair::pose::coplanar_ellipse_circle` | the pair's harmonics | not measured, as above |
| `geom_brep::curve_pair::pose::coplanar_ellipses` | `rounding_charge(terms)` of the level polynomial | not measured, as above |

What the pattern cannot see: harmonic readings outside the walk — the
first-harmonic door (`topo::boolean::circle_roots::first_harmonic_roots`,
which charges its extremes' own bounds) and the ladders' quartics
(escalations only) — are not walks and carry no `jet`.

The circle × torus measurement (mpmath oracle on exact binary inputs,
grazing/crossing/clear poses, ρ from 1 to 1000 m, ε = 1e-6, 1e-9 and
1e-12) is in that branch's PR body; it is the shape a sweep of these
doors would take.
