---
id: main-red-cylinder-sphere-section-runs-out-of-cells-near-the-node-at-1e-6
kind: issue
title: Main red: the cylinder x sphere section exhausts its cell budget near the node at eps 1e-6 (pair_section x2, cylinder_sphere_frame x2) since PR 5872
status: open
opened: 2026-10-28
priority: P0
cost: M
---


Found by nightly triage 2026-10-28, run
[40637814390](https://github.com/evgunter/cad/actions/runs/40637814390)
(main `a1cfd1cfdb`), job `full suite (every eps row, slow set included)`.
Red at `CAD_TOLERANCE_EPS=1e-6` only; green at the default ε and 1e-12.
The PR gate runs a touched crate's 1e-6 row, but these are slow-set rows.

## What

- `geom-brep` `pair_section::one_loop_keeps_its_order_near_the_node`
  (`crates/geom-brep/tests/pair_section.rs:675`): `d 0.2, R 0.699:
  March(CellBudget { budget: 200000 })`.
- `geom-brep` `pair_section::two_loops_keep_their_sites_and_order_near_the_node`
  (`crates/geom-brep/tests/pair_section.rs:650`): `d 0.2, R 0.71,
  θ 4.27: Escalated(Indeterminate { margin: 9.56e-6, band: (1e-6, 1e-5),
  predicate: "curve_carrier_piece_clear" })`.
- `sweep` `cylinder_sphere_frame::a_ball_across_a_cylinder_wall_builds_sound_or_stops_at_the_lane`
  (`crates/sweep/tests/cylinder_sphere_frame.rs:305`) and
  `cylinder_sphere_frame::every_segment_joins_neighbouring_sites_along_its_loop`
  (`:356`): "one loop, ball wider than the wall, A ∪ B" refuses
  `GermSectionUnbuilt { a_kind: Cylinder, b_kind: Sphere, why:
  March(CellBudget { budget: 200000 }) }`.

So near the cylinder × sphere node, at 1e-6, the section march runs out
of its cell budget, and one site sits in the band.

## Where it came from

PR 5872 (`sect/section-object`, merge `f70402dc41`) adds
`pair_section.rs` and edits `cylinder_sphere_frame.rs`; all four rows
are red at 1e-6 on that merge. The open PR 5894
(`sect/cylinder-sphere-chord`, head checked 2026-10-28) does not fix
them: both `pair_section` rows and `every_segment_joins…` still fail,
and its renamed `a_ball_across_a_cylinder_wall_wider_than_the_wall`
fails the same way.

## What closing it takes

Decide whether the budget exhaustion near the node at 1e-6 is the
march's honest limit (then the rows say so at that ε, fail-loud) or a
march that refines without converging there (then fix the march). The
in-band site at θ 4.27 is a separate question: whether a piece-clear
margin of 9.56e-6 at a 1e-6 band is a near-node configuration the row
should not sample, or a decision the section should take another way.

Reproduce: `CAD_TOLERANCE_EPS=1e-6 cargo nextest run -p geom-brep -p
sweep --ignore-default-filter -E 'test(/^pair_section::(one_loop_keeps|two_loops_keep)/)
| test(/^cylinder_sphere_frame::(a_ball_across|every_segment_joins)/)'`.
