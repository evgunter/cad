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
- **Loft: Book Eq. 10.8 over every compatible control row of the
  outer loop's walls; holes read the outer loop's parameters** (the
  orchestrator's ruling after PR 4193's review: the outer loop is
  structurally unique, so this is label-free, the seam argument holds
  within a loop, and a hole no longer moves the outer walls). Each
  section's per-row chord shares are sorted before they are summed, so
  the parameters are a function of the multiset of rows. Every
  re-spelling (start vertex, sense, a roll by the section's own
  symmetry, sketch origin) then builds bit-identical parameters and
  walls. A pinned row abstains as today, and "every row pinned" stays
  the existing refusal.
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

## Built (2026-10-06)

On `carve/loft-v-is-the-whole-sets` (PR 4193):

- `loft_geometry` takes the parameters as an argument.
- `loft_body` and `loft_parameters` share one helper. Following the
  orchestrator's ruling (option B, after the review), it is Eq. 10.8
  over the OUTER loop's compatible control rows only, per-section
  shares sorted, the mean exact when the rows agree. Holes are
  interpolated at the outer loop's parameters: the outer loop is
  structurally unique (its validated role), so this is still
  label-free; a hole cannot move the outer walls; and a hole that is
  the outer loop scaled lofts to the outer surface scaled (the tour's
  tube identity holds to 1.9e-16, where holes in the average put it
  5.1e-5 off).
- `sweep_body` and `sweep_geometry` put each station at `i/(k − 1)`.

Pinned by `loft_v_is_the_section_set` (corners, symmetry roll,
order-free means, holes, arcs: bit-identical parameters and walls;
the cap planes are not, see
`a-loft-caps-plane-is-summed-in-the-authored-vertex-order`) and
`turning_orientation::the_inflecting_duct_is_one_solid_whatever_the_start_frames_roll`.

**Open: the path-parameter sweep rule makes the tour's square coil
escalate.** `demos/tour/src/projectbox.rs`'s 6-turn coil refuses at
assembly with `Escalated { check: ParamSpan, predicate:
"nurbs_span_meter", margin −0.055 }`. Root cause, measured: the spine
advances uniformly between stations (centre chord 0.02756 at every
step), but `sweep_places`' frame is the minimal rotation from the START
tangent. On a helix that rotation swings the section about its own
tangent where the tangent nears anti-parallel to `t₀`, every half-turn
(stations 15–18 of each 32: a corner's step goes 0.0214 → 0.0607
while `tangent·t₀` reaches −0.98). Chord-length v absorbed the spin;
it deviated from uniform by 6.1e-3, more than a station step. Uniform
v squeezes it into one step, so the cubic through the corner rows
overshoots, and two seams' `speed_lower_bound` goes negative (−0.055,
−0.016; +1.7 under chord-length v). The round-wire spring's wall 1
holds as before.
