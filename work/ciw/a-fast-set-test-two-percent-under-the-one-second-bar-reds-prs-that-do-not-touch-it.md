---
id: a-fast-set-test-two-percent-under-the-one-second-bar-reds-prs-that-do-not-touch-it
kind: issue
title: A fast-set test two percent from the 1.0 s bar reds a PR that does not touch it
status: closed
opened: 2026-11-20
priority: P3
cost: E
closed: 2026-11-22
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
  - `geom-brep curve_pair::walk::tests::a_held_window_reads_each_end_on_its_own_spans_side`, 0.500 s against the 0.5 s bar for a test "new in this diff". The test is main's: the PR edits neither it nor its file (`curve_pair/walk.rs`), and its diff against main does not spell its name. **Why it read as new:** the PR's diff adds three `mod tests {` lines elsewhere in geom-brep, and `--new-since` keys an added module by its bare name (`pkg:mod:tests`), then calls a test new when "a module on its path is one of its package's added `mod`s". So one added `mod tests` anywhere in a package holds every `…::tests::…` test of that package to the new-test bar. Keying the token by the module's path (the file's module plus the declared name) would fix it.
- **2026-11-23, PR 7203, run 45076908929**: `cert_m2r1_passes::m2r1_passes_interval`, 1.145 s against 1.0 s, on GREEN's row `a-fast-set-germ-row-sits-on-the-one-second-bar-and-reds-a-docs-only-commit`.

Three units of one program met this bar on other programs' tests in one day, each after waiting over two hours for the runner that read it.

## Closed

2026-11-22, CIW triage (stale). Duplicates GREEN's P1 `work/green/the-one-second-bar-reds-prs-off-their-path-several-times-a-day.md` (opened 2026-11-22: five PRs redded by the 1.0 s bar in one evening). GREEN's row covers the class and the remedy. Also overlaps GREEN's P2 `a-fast-set-germ-row-sits-on-the-one-second-bar-and-reds-a-docs-only-commit` (four sightings). The row's test (`the_valley_between_two_touching_tubes_fills`) is still in the fast set: only a sibling, `the_lune_fillets_place_every_lattice_point`, is in `.config/nextest.toml:483`. The 2026-11-23 sightings above, including the `mod tests` keying defect in `--new-since`, are carried to GREEN's row (`work/green/log.md`, 2026-11-23).
