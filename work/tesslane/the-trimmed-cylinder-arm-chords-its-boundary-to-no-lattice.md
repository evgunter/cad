---
id: the-trimmed-cylinder-arm-chords-its-boundary-to-no-lattice
kind: issue
title: The trimmed cylinder arm is the one lattice the chord rule does not size a boundary against: its boundary chords at its target alone
status: open
priority: P2
cost: M
opened: 2026-11-14
---

Filed by `one-chord-rule-over-each-boundary-images-speed-bound`.

## What

`mesh::chords::tighten` holds every boundary chord to one lattice step
per chart axis on each face whose lane lays a lattice
(`FaceLattice::of`): the curved lane, the trimmed sphere arm, the spline
arm. The trimmed cylinder arm answers `None` there. Its lattice
(`trimmed::tessellate_trimmed`, `Lane::Cylinder`: `h_u` the sagitta step
at the face's target, rows every `r·h_u`) is sized beside a boundary
chorded by each carrier's own count at that target and nothing else.

Two of its three image forms are already within one column:

- a rim (`Harmonic`, `u` winding 1) is chorded at the sagitta step on
  the cylinder's radius, which is `h_u`;
- a tilted section ellipse (`Harmonic`, `u` winding 1) is chorded at
  `ellipse_step`, the curvature step on `sup|C″| ≥ r`, finer than `h_u`;
- a ruling moves in `v` only, along which the cylinder's cell bound
  reads no extent.

The third is not: a `Projected` image of a B-spline carrier (a
cylinder's section by another curved surface, a tee's) is chorded at
its carrier's curvature step, which says nothing about how far its
image moves in azimuth per chord. A chord spanning several columns
puts CDT triangles beside it whose azimuth extent the lattice's
sagitta does not bound; the certificate refuses typed where one
exceeds δ (`work/tesslane/the-trimmed-cylinder-lane-has-no-worst-triangle-theorem.md`).

## Why it is not in the rule yet

Adding the arm with steps `(h_u, +∞)` moves no rim or ellipse count and
raises the spline sections' counts to `⌈s_u·Δt/h_u⌉`, with `s_u` the
projected image's azimuth bound (`|C′|/ρ_min`, `geom_brep::Pcurve::speed_bound`),
which reads the whole carrier speed where only its horizontal part
moves the azimuth. That moves the meshes of every trimmed cylinder cut
by a spline (booleans of cylinders against cylinders, cones, tori), and
which count is right there is the arm's lemma's question (the plan's
unit 5), not a refactor's.

## Fix

Once the arm has its containment lemma (or the decision is taken to
hold it to the rule at its headroom like the sphere arm), answer
`(h_u, +∞)` for it in `FaceLattice::of` and re-baseline what moves with
that cause named. A tighter azimuth bound for the projected image
(`|C′_xy|/ρ_min`) belongs with that change.
