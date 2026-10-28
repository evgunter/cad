---
id: main-red-cylinder-sphere-section-runs-out-of-cells-near-the-node-at-1e-6
kind: issue
title: Main red: the cylinder x sphere section exhausts its cell budget near the node at eps 1e-6 (pair_section x2, cylinder_sphere_frame x2) since PR 5872
status: closed
closed: 2026-10-28
opened: 2026-10-28
priority: P0
cost: M
branch: sect/near-node-at-1e-6
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

## A fuzz counterexample in the same file, at 1e-6

This PR's gate (head `eee69f222f`, a docs-only diff) drew a counterexample
from `pair_section::no_pose_off_the_band_moves_from_the_retired_formulas`
(`crates/geom-brep/tests/pair_section.rs:586`), a fuzzer that is in the
PR set at 1e-6:

    pose 1562 (off 1.9083673845532123e-1, r 0.19083387329975715,
    R 0.0000028651555640801796)
      left: (Tangent, false)
     right: (Touch, false)

It reproduces locally with `CAD_TOLERANCE_EPS=1e-6
CAD_FUZZ_SEED=0xf68fbbf2328f612f CAD_FUZZ_EFFORT=1 cargo nextest run -p
geom-brep --ignore-default-filter -E
'test(no_pose_off_the_band_moves_from_the_retired_formulas)'`. The pose is
a near-degenerate sphere (R ≈ 2.9e-6, about 3ε) whose offset almost
equals the cylinder radius. The section value calls it `Tangent` where
the retired formulas said `Touch`. Pin it as a deterministic row beside
the fix, or as the sampler's boundary if a sphere that small falls
outside the formulas' domain.

## 2026-10-28 — pinned on main (GREEN, `green/full-suite-pins`)

Nightly run 40637814390 (main `a1cfd1cfdb`), `full suite`; re-run on main `91c83c38`. All four rows still red at 1e-6. PR 5894 has since merged and renamed
`a_ball_across_a_cylinder_wall_builds_sound_or_stops_at_the_lane` into
one row per pose; only `a_ball_across_a_cylinder_wall_wider_than_the_wall`
is red (the other eight pass at 1e-6). Each red row is pinned at 1e-6 with
`test_utils::pinned_red::row` on its stop:

- `pair_section::one_loop_keeps_its_order_near_the_node`: `d 0.2, R 0.699: March(CellBudget`;
- `pair_section::two_loops_keep_their_sites_and_order_near_the_node`: `curve_carrier_piece_clear`;
- `cylinder_sphere_frame::a_ball_across_a_cylinder_wall_wider_than_the_wall`
  (written out of the `pose_rows!` macro to carry its pin) and
  `cylinder_sphere_frame::every_segment_joins_neighbouring_sites_along_its_loop`:
  `one loop, ball wider than the wall, A ∪ B: … GermSectionUnbuilt`.

Each pin goes red the day its row passes; closing this row removes them.

Un-pinning owes a re-measure: each pin stops its row at main's first stop, so whatever runs after it in the row is unmeasured. Known behind the pins: the poses after `d 0.2, R 0.699` and after `d 0.2, R 0.71, θ 4.27` in the two `pair_section` rows, and the operations after `A ∪ B` in the two `cylinder_sphere_frame` rows.

## Closed

Two kernel defects, neither the honest behaviour at 1e-6; fixed at the
root (branch `sect/near-node-at-1e-6`).

- **The cell budget**: limb 3 takes one rung for the whole chain, and
  at 1e-6 the march's samples (spaced as `ε^¼`) leave chain boxes up to
  3.4e-2 wide where the surfaces cross at a small angle near the far
  ruling. At `d 0.2, R 0.699` every rung down to 3.4e-4 read a zero
  graph margin, the tube was 1.7e-4 (5.5e-3 at the default ε), and
  `account_r3` spent its 200000 cells. `ssi::refine::refine_for_tube`
  now halves the gaps whose box is wider than the next rung up and not
  a graph there, and keeps the finer carrier where its tube is wider:
  the pose certifies at 5.5e-3 on 508 boxes at 1e-6.
- **The in-band site** (`d 0.2, R 0.71, θ 4.27`): `Curve3::foot` on the
  closed fitted loop, seeded at its start, kept the seed (gap 2.99e-3,
  nearer than any span sample) and polished it against the domain's
  start, while the true foot lay at `t = 0.9991` across the seam, 3.3e-7
  away; the piece walk then read a piece 9.56e-6 off. The foot now
  polishes from each end of the domain as well.
- **The fuzz counterexample** (pose 1562): its sphere radius 2.87e-6
  lies in the 1e-6 band, and the section escalates it by name
  (`RadiusEscalated { radius: Sphere }`, `cs_sphere_radius`), which the
  retired guard never read. The sampler now keeps radii above the band,
  and `pair_section::a_radius_in_the_band_escalates_by_its_name` pins
  the pose.
- `pair_section::a_seated_loop_is_the_loop_point_for_point` failed at
  1e-6 the same way (`CellBudget` at `d 0.2, R 0.699`) and is green.
  At 1e-12 it does not hang: it takes 472 s alone on main and 477 s on
  the branch, in `account_r3`'s tube scan (evidence added to
  `a-cylinder-sphere-march-near-its-node-costs-seconds-and-refuses-by-budget`).

Filed: `work/issues/a-closed-splines-projection-stops-at-its-start-short-of-the-seam.md`,
`work/ssimarch/the-chart-and-pair-tubes-are-held-by-their-coarsest-box.md`,
`work/ciw/the-slow-tests-a-diff-adds-run-at-the-default-eps-only.md`.
