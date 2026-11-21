---
id: a-meridian-ending-an-ulp-of-its-far-end-off-the-apex-does-not-cover-the-apex-cells-side
kind: issue
title: A cone meridian whose image ends an ulp of its far end off the apex does not cover the apex cell's side, so a large cone's half-cones read Local at the apex at every depth
status: open
opened: 2026-11-18
priority: P1
cost: M
---


## Finding

From the fix pass of PR 7115 (EMBED, lane pole, 2026-11-18), tracing
the cases of the review's cone sweep
(`self_intersection_proves_injectivity::review_sweep_cone_tips_off_the_origin`)
that still refuse once the apex is placed.

**Two built bodies, f64**, the triangle with `r = 20`, `h = 50` revolved
a full turn about `y`:
- apex down at `y0 = 101.3`, the body:
  `Uncertified { 3v1, 4v1, Edge(2v1), Departure }`,
  16,842 / 4,404 / 21,240 (discharged / refused / splits);
- apex up at `y0 = 0`, its axial-door cavity at `t = 1`:
  `Uncertified { 1v1, 2v1, Edge(1v1), Departure }`,
  16,800 / 3,573 / 20,367.

Both refused `Budget(Pairs)` on PR 7115's first head, where the apex
was not placed at all.

## Trace

- **The pair** is the two half-cones across a shared meridian. In about
  23,000 cells of each body one half-cone is presented and the other
  reads `Err(Local)`; in about 400 both do.
- **The raising site** is `present_chart`'s `sides_are_the_faces`
  (`crates/topo/src/injective.rs`): 50,216 of the 50,231 `Local`s of
  the two runs.
- **The stored extents** are `v = (7.105427357601002e-15, 53.85…)` on
  the first body's half-cones and `(-49.87…, -6.59722368031097e-15)` on
  the second's. `7.1e-15` is one ulp of `53.85`: a meridian line's `v`
  at the apex is `c + l·t`, rounded at the scale of the edge's far end.
  `face_extent` moves the extent to `0` (the gap is inside
  `chart_slack`), and the apex's collapsed side is placed at `v = 0`.
- **The cover fails.** The rectangle's meridian side then runs from
  `v = 0`, and the meridian's own image starts at `7.1e-15`. `covered`
  takes the side's ends `few_ulps` in, and `few_ulps(0)` is
  `1.78e-15`, so the edge does not cover its side and the cell is
  `Local`, as is every smaller cell at the apex.
- **Unit probe (uncommitted):** the sector of
  `a_cone_face_whose_extent_starts_a_rounding_off_its_apex_places_the_apex`
  with the low end at `1.5e-15` presents the apex cell; at `2.0e-15`
  and at `7.1e-15` it reads `Local`. The flip is `few_ulps(0)`.

## What is needed

The meridian ends at the apex's vertex, and the collapsed side is that
vertex. So the edge reaches the collapsed side by the body's topology,
whatever its image's end rounded to: the reading `run_on` already makes
for an edge at a corner with a line on the other axis ("structure, not
a concession, as the corner is on both"). Run a side's edge on to a
collapsed side whose vertex is the edge's end, and the cover holds with
no slack widened.

The other way, a cover slack at the scale of the edge's own parameters,
is an ε concession's scale and belongs with
`few-ulps-is-taken-at-several-scales-across-injectives-readings`.

A sphere's meridian has the same shape at a pole, and its end is within
an ulp of `π/2`, inside `few_ulps`, so no sphere body is known to reach
it.
