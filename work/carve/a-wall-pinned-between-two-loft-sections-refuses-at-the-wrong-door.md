---
id: a-wall-pinned-between-two-loft-sections-refuses-at-the-wrong-door
kind: issue
title: a loft section hinged on one of its own edges collapses that wall, and refuses at the attach gate (or as NoParameterStep on strip 0) rather than naming the pinned wall
status: open
opened: 2026-10-06
priority: P1
cost: H
design: true
refs: [loft-v-parameterization-is-the-first-strips-so-a-rolled-section-changes-the-body]
---


Found by `carve/one-door-for-coincident-sections`'s sweep (2026-10-06).

A loft section rotated about one of its own edges, so that edge is
the same world segment in it and its neighbour, passes the stacking
fold (`crates/sweep/src/loft.rs` `stacking_fold`: the centroid moves
along the base normal) and leaves the wall over that edge with two
identical section curves. What happens next depends on which strip
the edge is, not on the fact:

- **The edge is segment 0 of loop 0** (the strip
  `first_strip_parameters` measures v on) **and the hinge is
  bit-exact**: the parameters do not ascend and the skin refuses
  `SkinError::NoParameterStep` naming the section. True, but it says
  nothing about the wall, and the same section authored from another
  start vertex takes the next arm.
- **The edge is segment 0 of loop 0, pinned only up to rounding**
  (PR 4186's review, measured): under a generic placement (rotation
  about (0.3, 0.4, 0.866) by 0.7 plus a translation) the strip-0 hinge
  moves by about 7.9e-17. The parameters come out `[0, 7.68e-17, 1]`,
  pass the exact ascent check, and `loft_geometry` (and so
  `sweep_geometry`) return `Ok` with a largest control coordinate of
  about 3.46e15 for a 2-unit section: a silent wild answer at a public
  door. `loft_body` refuses at the attach gate. Pinned as measured by
  `one_door_for_coincident_sections.rs`
  `a_hinge_pinned_only_up_to_rounding_passes_the_exact_ascent_check`,
  which goes red when this is fixed.
- **Any other edge**: the parameters ascend, the skin builds, and the
  assembly refuses at the attach gate. Measured on three 2×2 squares
  at z = 0, hinged about the edge y = +1 by −0.3 rad, z = 2, v-degree
  2: `an Euler operation of the assembly refused: geometry attachment
  gate: the stored interval's span (not a sampled check) escalated:
  margin -8.468018830476105e-1 lies past the ambiguity band (1e-9,
  1e-8). Recourse: move the geometry so this edge is not vanishingly
  short` — a refusal about an edge's span, past the band, for a wall
  that collapsed to a segment.

The pinned-row decision itself is a bare compare: `skin.rs`
`skin_parameters`' `!(total > 0.0)` decides a control row is pinned
and abstains from the average, unbanded, so a row that moves by
rounding counts as moving.

The fixture is `one_door_for_coincident_sections.rs`'s hinged row,
with the hinge moved off strip 0. Whether a pinned wall should refuse
at all (a loft that touches itself along an edge) or build, and with
what refusal, is a design question; the v-parameterization item
(`refs`) changes which arm strip 0 takes.

## Evidence (2026-10-06, after `loft-v-parameterization-…` landed on its branch)

The loft's v is now Eq. 10.8 over every row of the OUTER loop, so a
section hinged on one of its own edges no longer pins the
parameterization: the loop's other rows step. Both hinges in
`one_door_for_coincident_sections` (the exact strip-0 hinge, and the one
pinned only up to rounding) now parameterize normally (`[0, 0.1487, 1]`
for the rounding one), `loft_geometry` answers walls the size of the
sections, and `loft_body` refuses at the attach gate. The exact hinge:
`Euler(Certification { Escalated { check: ParamSpan, predicate:
"nurbs_span_meter", margin −0.349 } })`. The hinge's pinned corners make
their seams double back. Before, the refusal depended on WHICH edge was
the hinge (strip 0: `NoParameterStep`; any other edge: this one). It is
now the same for every edge, and still at the wrong door.
