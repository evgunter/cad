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

## What to do

Read `EulerCounts` whole at those rows (or have `counts` return it) and
assert `counts.genus() == Ok(h)` with the `h` the comment names; the
door's parity refusal then fails the row typed instead of an odd χ
passing through a hand subtraction. Mechanical; nine rows in two files.
