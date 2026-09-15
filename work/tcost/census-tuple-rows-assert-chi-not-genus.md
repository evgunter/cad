---
id: census-tuple-rows-assert-chi-not-genus
kind: issue
title: sweep's counts() tuple rows assert χ on door-read counts rather than genus() because the tuple carries no s
status: open
opened: 2026-09-12
refs: [no-public-census-or-genus-query, 2131]
---

## Finding

PR 2131 (`topo::readback::euler_counts`, the Euler–Poincaré census
door) converted every hand-written copy of the identity in
`crates/sweep/tests`. Two files keep a `counts(body) -> (v, e, f, r)`
tuple helper — `extrude_acceptance.rs`'s `counts` and
`review_m2_pr4.rs`'s `counts` (and `revolve_common::counts`, which the
`revolve_*` rows compare only against literal tuples) — and their rows
still assert the characteristic `v − e + f − r` by hand on the door's
numbers:

- `extrude_acceptance.rs`: `extruded_l_profile_passes_all_tiers`,
  `extruded_profile_with_hole_builds_the_ring_path`,
  `rounded_square_exercises_tangent_line_arc_joins`,
  `disc_extrudes_to_a_shared_carrier_cylinder`,
  `both_extrusion_directions_build_outward_solids` — five
  `assert_eq!(v - e + f - r, 2 | 0)` rows.
- `review_m2_pr4.rs`: `survives_two_arc_hole_hand_traced_cycles`,
  `survives_hole_near_outer_canonical_start`,
  `survives_multiple_holes_genus_h` (twice) — four rows asserting
  `0`, `−2`, `−4` with the genus in a trailing comment.

The tuple carries no `s`, so the rows cannot say `genus() == Ok(h)`;
each states χ and leaves the shell term implicit (every one of these
bodies is one shell, so χ = 2 − 2h). The conversion kept the rows'
statements as they were (the S-TCOST seam: rows are ordinary tests and
do not move), which is why this is a residue rather than part of that
PR.

## The same shape elsewhere (the class, not the instance)

The χ-on-door-counts spelling is not only these two files. Every row
below reads its counts through the door and then asserts the
alternating sum by hand, for a reason of its own:

- `crates/topo/tests/review_m3_pr1.rs` — `chi(body)` (`c.vertices −
  c.edges + c.faces − c.rings` over the door-fed `EulerCensus`) and its
  rows `cross_shell_kfmrh_connected_sum_and_genus_addition` and the
  detached-component rows around it, which state χ per component sum
  where the whole-body `genus()` would fold two shells into one number.
- `crates/topo/tests/review_m1_pr5.rs` —
  `nested_detachment_detached_component_with_genus`'s `v − e + f − r ==
  2` is a component-aware statement (`c = 2` components in one shell)
  the whole-body door cannot make.
- `crates/topo/src/review_m1_pr2/mod.rs` — `euler_poincare_holds(body,
  shells, genus)`, the caller-supplied `(s, h)` probe form used by
  `cube_independent` and `degenerates_and_sequences`; its header asks
  that the reviewer's re-derivations not be simplified to match the
  implementation.

The tcost/tint owner decides which of these move to `genus()` (the
first file's single-shell rows and this file's nine can), which keep χ
because they state something per component the door does not carry,
and whether the review module's form is off-limits by its header.

## What to do

Read `EulerCounts` whole at those rows (or have `counts` return it) and
assert `counts.genus() == Ok(h)` with the `h` the comment names; the
door's parity refusal then fails the row typed instead of an odd χ
passing through a hand subtraction. Mechanical; nine rows in two files.
