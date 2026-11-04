---
id: main-red-a-lofted-arc-walls-rim-zeros-count-eighteen-not-ten
kind: issue
title: main red: a_loft_computes_its_walls_in_t::a_lofted_arc_wall_is_a_function_of_the_sections_bulge counts 18 symbolic pcurve_iso_boundary Zeros where it pins 10
status: open
opened: 2026-11-04
priority: P2
cost: E
---



Found by `onecopy/foot-certificate` (PR 6483) running sweep's full suite. It reproduces on `origin/main` at `1ee90563a` with the same numbers, in its own worktree and target dir.

`sweep::all a_loft_computes_its_walls_in_t::a_lofted_arc_wall_is_a_function_of_the_sections_bulge` (slow set) fails at `crates/sweep/tests/a_loft_computes_its_walls_in_t.rs` (~116):

> every Zero the arc rims' checks answer over the bulge, the weight margin `w − cos(h/2)` among them, is a symbolic Zero
> left: [true × 18]
> right: [true × 10]

Every Zero is still a symbolic one: what moved is the count of `pcurve_iso_boundary` Zero decisions whose margin reads the bulge, from 10 to 18. Some PR on main runs the iso-boundary checks over the lofted arc wall more times, or over more rims. If that is right, the pin's `vec![true; 10]` moves with it, and the row should say why.

Reproduce: `cargo nextest run -p sweep -E 'test(=a_loft_computes_its_walls_in_t::a_lofted_arc_wall_is_a_function_of_the_sections_bulge)'`.
