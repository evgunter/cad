---
id: a-fast-set-test-two-percent-under-the-one-second-bar-reds-prs-that-do-not-touch-it
kind: issue
title: A fast-set test two percent from the 1.0 s bar reds a PR that does not touch it
status: open
opened: 2026-11-20
priority: P3
cost: E
---


Found on PR 7190 (SCALAR, `certification-interiors-are-closed-to-a-dual-by-a-marker-bound`), 2026-11-20.

## What

`check-fast-set-times` bars an existing fast-set test at 1.0 s. On PR 7190's run 44842407658 (head `95005e02fc`, `test (kernel)`), all 12118 tests passed at the default ε and the job failed on one line:

> sweep::all a_knife_edge_fillet_builds_where_its_ball_is_defined::the_valley_between_two_touching_tubes_fills took 1.020 s (existing test: 1.0 s)

The same test was under the bar on the same PR's two earlier heads (runs 44810676172 and 44829855393, both `test (kernel)` success), and the PR's diff changes bounds and types only: no statement that test executes moved. So the reading is the runner's, not the change's.

## What it asks

Either the test into the slow set (it costs about a second on a hosted runner, twice the bar a new test is held to), or a bar that does not fail a run on one reading within a few percent of it. `crates/sweep/tests/a_knife_edge_fillet_builds_where_its_ball_is_defined.rs`, `the_valley_between_two_touching_tubes_fills`.

## Further sightings

- **2026-11-23, PR 7221, run 45088390543 (head `6862af6efe`, `test (kernel)`).** Two bar lines on tests the PR does not edit:
  - `sweep::all rest_zip_admission::a_box_dipping_into_a_plate_at_a_tangent_site_never_ships_the_overlap_twice`, 1.009 s against the 1.0 s bar. The PR touches nothing in the sweep crate.
  - `geom-brep curve_pair::walk::tests::a_held_window_reads_each_end_on_its_own_spans_side`, 0.500 s against the 0.5 s bar for a test "new in this diff". The test arrived on main (`0cb2daf789`) and reached the branch through a merge of main; no commit of the PR's own touches it. **Why the script read it as new is not established**: `--new-since` calls a test new when an added line of the diff spells its leaf name, and which added line did is unread.
- **2026-11-23, PR 7203, run 45076908929**: `cert_m2r1_passes::m2r1_passes_interval`, 1.145 s against 1.0 s, on GREEN's row `a-fast-set-germ-row-sits-on-the-one-second-bar-and-reds-a-docs-only-commit`.

Three units of one program met this bar on other programs' tests in one day, each after waiting over two hours for the runner that read it.
