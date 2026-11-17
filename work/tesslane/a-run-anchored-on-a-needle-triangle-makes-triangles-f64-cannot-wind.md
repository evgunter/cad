---
id: a-run-anchored-on-a-needle-triangle-makes-triangles-f64-cannot-wind
kind: issue
title: a run of merged ruling points whose ruling-side triangle is a needle adds triangles f64 cannot wind
status: open
opened: 2026-11-17
priority: P2
cost: M
---


Found building `a-run-of-ruling-points-the-development-merges-is-re-split-after-the-triangulation`; its reach at the default ε measured by that unit's review (PR 7054).

## Finding

A run of ruling points the cone's development merges (`trimmed::Run`)
is anchored by its neighbour along the ruling: the triangles between the
run's points are cut from the CDT triangle on the polygon side to that
neighbour, and fan about that triangle's third corner. Such a triangle
has height

    g·d / L

with `g` the run's gap, `d` the third corner's distance from the ruling
and `L` its distance from the run. The winding rule
(`validate::resolved_normal`) refuses a triangle whose height is under
its allowance, about `1.8·10⁻¹⁵·M` with `M` the magnitude of its
coordinates. So the face refuses `UnresolvedWinding` where

    g·d / L < 1.8·10⁻¹⁵·M.

Where the run is anchored by the apex, a whole ruling away, the third
corner is a node of the lattice's first row, at slant `s₁`, one column
`Δu` from the ruling: `d / L ≈ (s₁ / L)·sin(sin α·Δu)` (the review's
reading). It is smallest on a slender cone (small `sin α`), at a fine δ
(a first row close to the apex, narrow columns), and the allowance is
largest on a large part or one placed far from the origin.

These faces are what is left, on large slender parts, of faces main
meshed before PR 7049 sent every cone face with a split ruling to the
chart lane. Before the unit they refused `DevelopedMerge`.

## Measured

At the default ε:

- the cone `(0, 0), (1000, 0), (0, 1000)` revolved a full turn about `y`,
  one ruling split `10⁻⁸` along it from its rim end, placed obliquely, δ
  1: height `3.3·10⁻¹³` against allowance `1.78·10⁻¹²`. Pinned.
- a cone of half-angle 2° and height 100, split `3·10⁻⁸` from its rim
  end, δ 0.01: `8.9·10⁻¹⁴` against `1.78·10⁻¹³` (the review's).
- over the review's family (heights 100 and 1000; half-angles 2°, 10°,
  45°; full turns, half turns and a wedge of 0.5; gaps `10⁻⁸` to
  `10⁻⁶`): 170 of 808 cells with a run refuse.

At ε `10⁻¹²`:

- the cone `(0, 0), (1, 0), (0, 1)`, one ruling split `10⁻¹⁰` of its
  range from its rim end: δ 0.1 and 0.01 mesh, δ 0.001 refuses, height
  `5.15·10⁻¹⁵` against allowance `9.14·10⁻¹⁵`. Pinned. 11 of the 12 such
  cells the unit probed at δ 0.001 (full turn, wedge of 0.5; the split
  `10⁻¹⁰` from the rim end, or twice `10⁻¹⁰` apart at 0.99).
- cones of height 100 and 1000 at 2° and 10° with a gap of `10⁻¹⁰`
  refuse at every δ the unit tried from the part's height down to a
  thousandth of it; the review found the 0.01° cone through its apex
  refusing every run at every δ.

At ε `10⁻⁶` a split is a split from `10⁻⁵`, and no body the unit tried
reaches the refusal.

Pinned: `split_rulings::a_run_anchored_on_a_needle_triangle_refuses_unresolved`.

## How near the allowance the meshing faces come

`check_winding` passes every face that meshes, and that is all it says.
On the unit's own draft cones (the wall and the solid, half-angles 0.1°
to 26.6°) the least height over allowance of an added triangle was
`1.6·10⁵` at the default ε and 163 at ε `10⁻¹²` with a gap of `10⁻¹¹`.
Those are the least of the cells measured, not a margin: the review's
stricter test (16 ulp) flags 29 newly meshing cells at ε `10⁻¹²` and 45
of the needle family that `check_winding` passes, where the height is
within single digits of the allowance.

## The cures seen

Neither is built.

- **Keep a run's points apart inside the development**, on successive
  representable radii of the same exact line, so the CDT triangulates
  them itself and there is no anchor and no needle (the review's first
  choice). Unchecked: whether it survives neighbours in the adjacent
  bins, which the moved points would run into.
- **Cut the fan along its squarest edge.** Every triangle of the run's
  fan can carry the added triangles: cut along any fan edge, the
  triangles before it take the run's first point and those after it the
  last. The lane takes the fan's first or last edge, by which neighbour
  is along the ruling. Taking the fan edge that stands most squarely on
  the ruling in the development makes the added triangles as high as the
  fan allows, and is a choice read from developed coordinates where
  today's is read from the polygon's order.
