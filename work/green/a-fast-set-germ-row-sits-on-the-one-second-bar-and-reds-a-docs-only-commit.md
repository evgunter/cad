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
