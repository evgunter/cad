---
id: green-row-floor-has-no-watcher
kind: issue
title: DESIGN.md's green-row floor (engineering convention 3) has no watcher in the repo that asserts it
status: open
opened: 2026-09-04
---


## Finding

`docs/DESIGN.md`, D9's engineering convention 3, states: *any
merge-gating checks watcher asserts a minimum green-row count equal to
the current full CI matrix, bumped in the same PR that grows the
matrix*. Nothing in this repository implements that floor. A sweep of
`scripts/`, `local-scripts/` and `.github/workflows/` for a green-row
count, `MIN_GREEN` or an equivalent finds none; the only `gh pr checks`
caller is `local-scripts/bt-wait-pr.sh`, whose header says it is a
temporary investigation helper that blocks until checks settle and
asserts no count. The floors that do exist are different things:
`scripts/check-ci-mirror-parity.py`'s `MIRROR_MARKER_FLOOR` (a
citation-marker count) and `scripts/gates/probe-suite-census.sh`'s
`CENSUS_FLOOR`.

The convention was earned by a stale-matrix trap (#113: a branch
predating new persistence rows showed green on the old matrix). If any
watcher outside this repo (the orchestration tooling) gates merges on a
row count, it is not visible from the tree, and the convention cannot
be checked here.

## Options

1. **Implement the floor in-repo**: a gate under `scripts/gates/` (or
   the parity checker) that reads the workflow's job list and refuses
   when a merge-gating consumer's expected row count is below it.
   Cheapest if a consumer exists; pointless if none does.
2. **Restate the convention** to what is actually enforced: the change
   filter fails closed and the `CI half parity + gate wiring` job
   already pins the matrix/mirror shape, so the honest sentence may be
   "the matrix shape is pinned by the parity job" rather than a
   green-row floor. That is a DESIGN.md revision and therefore Ev's.

Raised in PR #1842 (the `[ev]` DESIGN.md editing pass), which keeps the
convention as a one-line rule pending the answer.
