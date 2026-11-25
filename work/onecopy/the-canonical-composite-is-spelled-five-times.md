---
id: the-canonical-composite-is-spelled-five-times
kind: issue
title: A surface's canonical form composed along a curve is spelled five times, each with its own polynomial kit
status: open
opened: 2026-11-23
priority: P1
cost: M
---


Found on the fix pass of `heldrecords/elbow-ends` (PR 7463), 2026-11-23.

A surface's canonical implicit form (plane, cylinder, sphere, cone,
torus) composed with a point or a curve is written out five times:

1. `geom_brep::implicit_residual` (`crates/geom-brep/src/implicit.rs`):
   the form at a point, linearized.
2. `geom_core::spline::compose::canonical_composite`
   (`crates/geom-core/src/spline/compose.rs`): the form along a 3-D NURBS
   curve in the surface's chart frame, rational Bernstein, `Interval`
   only.
3. `geom_brep::curve_face::line_quadric` (`line_quadric_roots`,
   `line_torus_roots` and their kin): the form along a line, as the
   coefficients its root rows read.
4. `geom_brep::curve_face::circle_torus` (`circle_torus_roots`,
   `torus_data`): the torus's form along a circle.
5. `geom_brep::curve_face::end_root`'s `composite`: the form along a
   line or a half-angle circle `X(τ)/W(τ)`, as Taylor coefficients at an
   edge's end, generic in `T`, with its own polynomial kit (`add`,
   `scale`, `sub`, `mul`).

Why the fifth was not folded into one of the others in PR 7463: the
only general home, `canonical_composite`, is `Interval`-only and
Bernstein over knot spans, while the end-root door needs generic-`T`
monomial coefficients at a point, so it does not fit as it stands.

Done when the canonical forms and the polynomial arithmetic over them
live in one home that each of the five reads (or each remaining copy
says in-file why it cannot), with a row comparing the copies on one
pose per kind until they are one.
