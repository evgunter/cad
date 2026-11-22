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

## Closed

2026-11-22, CIW triage (stale). Duplicates GREEN's P1 `work/green/the-one-second-bar-reds-prs-off-their-path-several-times-a-day.md` (opened 2026-11-22: five PRs redded by the 1.0 s bar in one evening). GREEN's row covers the class and the remedy. Also overlaps GREEN's P2 `a-fast-set-germ-row-sits-on-the-one-second-bar-and-reds-a-docs-only-commit` (four sightings). The row's test (`the_valley_between_two_touching_tubes_fills`) is still in the fast set: only a sibling, `the_lune_fillets_place_every_lattice_point`, is in `.config/nextest.toml:483`.
