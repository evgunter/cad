---
id: loft-v-parameterization-is-the-first-strips-so-a-rolled-section-changes-the-body
kind: issue
title: loft_geometry takes the whole surface's v from the first strip, so a section rolled about its own normal builds a different body
status: dispatched
opened: 2026-09-12
priority: P0
cost: H
branch: carve/loft-v-is-the-whole-sets
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

## Weighed (2026-10-06): the body's v is a function of the whole section set, and a sweep's is its path parameter

Two designers (an Opus and a Fable lane) weighed this on the problem
alone, at CARVE's 2026-10-06 sitting. They converged on the same final
state, and both found no ratified text governing it: the first-strip
rule lives only in `skin.rs`/`loft.rs` comments. Ev's 2026-09-23 ruling
(PR 3102) made the section correspondence authored and did not touch
v. So the answer is built, not put to Ev.

- **One shared v-vector is forced.** The seam at vertex `j` is wall
  `j`'s `u = 0` iso and wall `j−1`'s `u = 1` iso. They are one curve
  only if both are interpolated at the same parameters. So the answer
  has to be a symmetric function of the walls, or of something none of
  them owns. "First" cannot be made right by picking better.
- **The comment's argument is false.** The skin interpolates, so
  section `k` is the surface at `v = params[k]` (in ℝ) for any shared
  parameters. Averaging does not make the sections "cross-sections of
  nothing in particular". The parameters shape only the surface between
  sections, which is shape, not labelling.
- **Loft: Book Eq. 10.8 over every compatible control row of every
  wall**, outer loop and holes alike. Each section's per-row chord
  shares are sorted before they are summed, so the parameters are a
  function of the multiset of rows. Every re-spelling (start vertex,
  sense, a roll by the section's own symmetry, sketch origin) then
  builds the bit-identical solid. A pinned row abstains as today, and
  "every row pinned" stays the existing refusal.
- **Sweep: v is the path's own parameter**, normalised
  (`(t_i − lo)/(hi − lo)`, today `i/(k−1)`). The stations are samples of
  the path at known parameters, and that is what the user wrote. The
  answer is invariant under every re-spelling and every roll of the
  start frame, because neither enters it.
- **Shape of the code.** `loft_geometry` takes the parameters as an
  argument, as `skin_on` already does one layer down. `loft_body`
  derives them by the whole-set rule, through the one helper
  `loft_parameters` also answers with. `sweep_body` derives them from
  the path. `Lofted::section_params` carries whichever was used.
- **Rejected.** Uniform `i/(k−1)` for a loft: it ignores spacing and
  can dip through a cap (sections at z = 0, 0.01, 10). Chord length
  between placement origins: the origin is a frame artefact, and D10
  stage 3 recasts placements. Between centroids: blind to rim motion. A
  user-written v slot: v is structure, not intent. Keeping first-strip
  and documenting it: a patch.
- **What moves** (re-baselined, not preserved): `nonuniform_loft`'s
  `NONUNIFORM_T` and volume pins in the tour and step-export (its closed
  form holds at the new `t`), the `loft_parameters` doctest value, the
  inflecting duct's `v = 0.5` location (it becomes the inflection), and
  curved-sweep goldens and renders.
