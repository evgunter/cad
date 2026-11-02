---
id: a-saddle-cut-by-its-tangent-plane-builds-at-its-closed-form
kind: issue
title: A saddle cut by its tangent plane refuses at the spline × plane pair, where its closed form is a straight section of two crossing lines
status: open
opened: 2026-11-02
priority: P3
cost: H
refs: [a-spline-wall-cannot-say-whether-it-holds-a-pierce-point]
---

Filed by SPLITDOOR on `splitdoor/section-line-components`, from the
saddle witness of
`splitdoor/a-straight-section-of-two-crossing-lines-off-a-cone-pairs-germs-across-the-crossing`.

## What

The block `x, z ∈ [−1, 1]`, `−2 ≤ y ≤ x·z`, lofted along `z`
(`sweep::loft_body`, degree 1), has the hyperbolic paraboloid
`y = x·z` for its top wall. The half-space `y ≥ 0` (a brick with its
bottom face on `y = 0`) is tangent to that wall at the origin and cuts
it along `x = 0` and `z = 0`: a straight section of two lines crossing
at a point. Closed forms: the block holds `8`, its part above the plane
`1/2`, so `∩ = 0.5`, `s ∖ h = 7.5`, `h ∖ s = 255.5`, `∪ = 263.5`.

Every op refuses at the spline × plane pair, before the join:

- `∩` and `∖`, either way round: `CurvedPairUnsupported { site:
  RevertRoster, kind: Nurbs, other_kind: Plane }` (the spline × plane
  arm is on the union's roster only, `boolean/reduce.rs`
  `boolean_arm_exists`).
- `∪`, either way round: `HeldPierceUnbuilt { kind: Nurbs, pierced_kind:
  Plane }`, this program's
  `a-spline-wall-cannot-say-whether-it-holds-a-pierce-point`.

`crates/sweep/tests/a_saddle_cut_by_its_tangent_plane.rs`
`a_saddle_cut_by_its_tangent_plane_refuses_at_the_spline_plane_pair`
pins those refusals.

Past them the pair is a tangency between distinct carriers at a point:
two smooth carriers meet in two crossing lines only where they are
tangent, so the section has a singular point there, a C7 construction
the section does not mint. Where it builds, the join pairs its germs
by the line each rides (`boolean::join::germ_branch`,
`Branch::Line`), which today only the plane × cone apex line pair
mints: a spline × plane section of crossing lines would mint its line
ids the same way.

## Done when

The saddle builds every op at its closed form, membership held at the
lattice, at ε 1e-6, 1e-9 and 1e-12, or refuses typed at the tangency
by name; the pinned row turns into that.
