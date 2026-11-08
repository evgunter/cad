---
id: main-red-a-lofted-arc-walls-rim-zeros-count-eighteen
kind: issue
title: Main red: a_lofted_arc_wall_is_a_function_of_the_sections_bulge counts 18 symbolic rim Zeros where it pins 10
status: open
priority: P1
cost: E
opened: 2026-11-04
---


Found by SECTARMS' PR 6479 fix pass, while running the sweep crate's slow set.

`crates/sweep/tests/a_loft_computes_its_walls_in_t.rs:116`
(`a_lofted_arc_wall_is_a_function_of_the_sections_bulge`) asserts
`rim_theorems == vec![true; 10]`, the `pcurve_iso_boundary` Zeros the
loft records over the bulge. It reads 18 of them now, every one
`true`, so the count moved and the proofs did not.

- It is red on main: nightly run 41779026337 (main at `63cbd7c300`), the
  `full suite (eps default, slow set included)` job.
- It was green on the nightly at `2d9dc809d8`.
- Between the two, PCERT merged several changes to
  `crates/geom-brep/src/pcurve_cache.rs`: E2's narrowed iso guard and
  E5's T1, among others. One of these is the likely mover. Unconfirmed.

The row is in the slow set, so a PR's CI does not run it.

What closes it: whoever owns the iso guard re-pins the count with its
reason, or restores the old count if the extra Zeros are a regression.
