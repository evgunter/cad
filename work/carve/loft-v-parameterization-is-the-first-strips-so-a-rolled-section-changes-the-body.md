---
id: loft-v-parameterization-is-the-first-strips-so-a-rolled-section-changes-the-body
kind: issue
title: loft_geometry takes the whole surface's v from the first strip, so a section rolled about its own normal builds a different body
status: open
opened: 2026-09-12
priority: P0
cost: H
---


## Finding

`loft_geometry` (`crates/sweep/src/skin.rs`) computes the v-parameters
once, from the FIRST STRIP — `first_strip_parameters(&validated, places)`
— and reuses them for every wall, so that all walls agree on where their
sections sit. The comment states the choice and cites Book §10.3.

The consequence nothing states: **which strip is first is the caller's
profile-vertex order, and rolling a section by one of its OWN symmetries
changes the body.** Measured on `sweep/tests/turning_orientation.rs`'s
inflecting duct (a centred square swept along two opposed quarter arcs,
13 stations, v-degree 3), built twice from start frames 90° apart about
the path's start tangent. A centred square is invariant under that
rotation, and `sweep_places` left-multiplies (`places[i] = T · Rᵢ ·
place`), so **every station's ring is the same set of world points in
both builds** — only which edge is strip 0 differs:

| | volume | ring centroid at v = 0.5 |
| --- | --- | --- |
| strip 0 = the edge the old test cone picked | 1.570494587359220 | (0, 2.000000000000, 2.000000000000) |
| strip 0 = the edge `path_start_frame` picks | 1.570508341333644 | (0, 1.647843453955, 1.965783296757) |
| continuum A·L | 1.570796326794897 | (0, 2, 2) — the inflection |

The two solids differ in the fifth digit of volume and by 0.354 m in
where `v = 0.5` lands on the spine; the second is the closer of the two
to the continuum. A user who spells the same square starting at a
different corner gets a different solid, and nothing in the door's docs
says so.

That sensitivity broke a row in S393's PR: the inflecting-duct
anti-vacuity condition split the spine at the halfway INDEX, which was
the inflection only under the old parameterization. The row now sums the
turn's negative and positive increments apart, which is invariant to
where the split lands — but the underlying sensitivity is this row's.

**Confidence**: sure (measured, both builds, same station rings).
**Where**: `crates/sweep/src/skin.rs`, `loft_geometry`'s
`first_strip_parameters` call and the comment above it; the door docs on
`loft_geometry` / `loft_body` / `sweep_geometry`.

**Verdict:**

## Built (2026-10-06)

On `carve/loft-v-is-the-whole-sets`, as weighed: `loft_geometry`
takes the parameters; `loft_body` and `loft_parameters` share one
whole-set helper (Eq. 10.8 over every row of every wall, per-section
shares sorted, the mean taken exact when the rows agree); `sweep_body`
and `sweep_geometry` put each station at `i/(k − 1)`. Pinned by
`loft_v_is_the_section_set` (four spellings, and a symmetry roll, build
bit-identical walls) and
`turning_orientation::the_inflecting_duct_is_one_solid_whatever_the_start_frames_roll`
(the issue's two frames: bit-identical walls, `v = 0.5` on the
inflection).

**Open: the rule's "holes alike" clause meets the tour's tube cell.**
`demos/tour/src/skinned.rs` asserts `V_tube = (1 − λ²)·V_solid` for an
annular loft and its hole-less twin over the same stations. With hole
rows in the average, the hole moves the outer skin and the identity is
off by 5.1e-5 relative; with the outer loop's rows only, it holds to
1.9e-16. The PR lays out the options; the cell is left as it was.
