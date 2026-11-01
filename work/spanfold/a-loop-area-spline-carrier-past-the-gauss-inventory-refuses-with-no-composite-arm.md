---
id: a-loop-area-spline-carrier-past-the-gauss-inventory-refuses-with-no-composite-arm
kind: issue
title: loop_area refuses a non-rational B-spline carrier of degree > 5 at the end of its exact Gauss inventory, with no composite fallback
status: open
opened: 2026-10-23
priority: P3
cost: M
---


Found by ISO's `trimmed-quadrature-composite-rounds` §5 sweep (the shape:
an exact rule's node inventory used as a fence with no composite arm
behind it). `geom_brep::props::loop_area`'s B-spline boundary-carrier
integral (`gauss_rule(degree)`, `crates/geom-brep/src/props/loop_area.rs`
near line 179) refuses a non-rational carrier of degree > 5 with
`QuadratureUnsupported { "a non-rational B-spline boundary carrier of
degree > 5 — the exact Gauss inventory covers the exportable degrees;
extend the table before integrating higher" }`.

The two other exact-rule windows in `props/` are not this shape:
`props/quad/patch.rs`'s `patch_flux_exact` and the rational lane's exact-v arm both
fall back to their composite rounds when `newton_cotes_weights` answers
`None`. The trimmed lane's outer rule did refuse, and now has a composite
arm (`chord_polygon_flux_composite`).

Whether this one needs an arm depends on what reaches it: a planar face
bounded by a degree ≥ 6 B-spline edge (a STEP import, or a section
whose fit degree rises). Nothing found reaching it today; either extend
the Gauss table (the refusal's own recourse) or add a composite arm with
a certified remainder.
