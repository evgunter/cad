---
id: a-fast-set-germ-row-sits-on-the-one-second-bar-and-reds-a-docs-only-commit
kind: issue
title: sweep's germ_conic_plane_roots::every_op_answers_its_closed_form_when_the_outside_root_comes_first ran 1.019 s against the fast set's 1.0 s bar on a hosted runner and reded a docs-only commit's run
status: open
opened: 2026-11-18
priority: P2
cost: E
refs: [the-ring-lane-interval-row-runs-at-the-one-second-bar-and-reds-a-pr-off-its-path]
---


## Finding

From EMBED, 2026-11-18. Run 44220807533, on commit `7797e1d83b` of
`embed/graph-piece` (PR 7080's state-sync commit: a model-compare row
and a row header, no code), failed in the fast set on a timing bar:
`sweep::all germ_conic_plane_roots::every_op_answers_its_closed_form_when_the_outside_root_comes_first`
took 1.019 s against the 1.0 s bar. The same tree passed on the commit
before it (`4de6b47929`, run 44218451555), and nothing EMBED changed
touches that row (a conic × plane Boolean row; `work/germ/conic-plane-sweep-examines-only-the-first-root.md`
names the test).

The PR merged on the green code head under the docs-only rule, so
nothing was held. The finding is the class
`the-ring-lane-interval-row-runs-at-the-one-second-bar-and-reds-a-pr-off-its-path`
already names: a fast-set row whose hosted time sits within a few
percent of the bar reds a PR that does not touch it. One sighting.

## What a fix needs

Its hosted time over a few runs; then the row moves to the slow set
with its measurement in `.config/nextest.toml`, or is made cheaper.

## A second sighting, another row (EMBED, 2026-11-18)

`sweep::all join_pierce_runs_sweep::six_crossing_corners_build_every_op`
took 1.000 s against its 1.0 s bar on the first attempt of run
44233155772 (PR 7086's head `0b4c04bac5`, a merge of main into a
branch that changes only `injective.rs`, which nothing in that row's
path calls). The lane timed the row the same locally with main's
`injective.rs` and its own, re-ran the failed job once, and it passed.

## A third sighting, a third row (EMBED, 2026-11-18)

`stl::all review_m3_pr55_e2e::die_from_raw_extrudes_watertight_stl`
took 1.024 s against its 1.0 s bar in run 44309772803 (PR 7120's head
`c209f8d46`, whose diff reads a clearance window off its edges' covers
and adds rows; nothing on the STL export or the extrude path). Every
test in the run passed; the kernel job failed on the bar alone. The
PR's previous run (44279494043, head `c1e39531a`) was green.

## A fourth sighting, a row new on main (EMBED, 2026-11-22)

`step-import::all an_edge_is_read_on_its_description_whatever_recognition_says::a_straight_rim_recognition_declines_is_carried_on_its_column`
took 1.314 s against its 1.0 s bar in run 44967278913 (PR 7120's head
`6cf518d0f`, a merge of main that brought the row in with
`e79727d30`; PR 7120 touches nothing in `step-import`). Every test in
the run passed, 12,649 at the default ε and 6,985 at each of 1e-6 and
1e-12; the kernel job failed on the bar alone. At 31% over the bar this
row is not jitter: it wants the slow set.

The second run (44969619240, head `34a244086`) read the same row at
1.347 s. PR 7120 moves it into the slow set with both times in
`.config/nextest.toml`, as the gate's message prescribes, so its gate
can conclude; making the row cheaper is its owner's.

## Measured (2026-11-22, green/fast-set-margin)

Over 40 PR runs (`docs/perf-data/fast-set-times/`):
`germ_conic_plane_roots::every_op_answers_its_closed_form_when_the_outside_root_comes_first`
p90 0.896 s, median 0.659 s, max 0.931 s; `-j1` at the gate's build
0.38-0.41 s; nightly opt 2 0.349 s. It stays in the fast set: the interim
move took only the rows at gate p90 >= 0.9 s. The second sighting's row,
`join_pierce_runs_sweep::six_crossing_corners_build_every_op` (p90
0.904 s, max 1.067 s on a 1.33x runner, `-j1` 0.39-0.43 s), moved to the
slow set. The third's, `review_m3_pr55_e2e::die_from_raw_extrudes_watertight_stl`,
read p90 0.750 s and stays; the fourth's is in the slow set (PR 7120).
PR 7397 decides whether the bar should red a PR for a row it does not touch.

## Another sighting (2026-11-23, PR 7203, from the ONEROUND orchestrator)
Run 45076908929: `test (kernel)`'s default-ε step went red on the bar
alone, with no failing test: `sweep::all
cert_m2r1_passes::m2r1_passes_interval` at 1.145 s against 1.0 s, an
existing test the PR does not edit. The same step was green on the same
code in run 45061549494. Timed locally, five runs each, alternating:
the PR's head 2.936 s median, main at its merge base 2.885 s, inside a
0.4 s spread. Hosted runners were queueing about an hour per stage that
night. The PR merged with the red named.
