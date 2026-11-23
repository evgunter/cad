---
id: the-conic-centre-census-holds-only-the-crates-a-scoped-lint-run-selects
kind: issue
title: On a scoped PR run the conic-centre census holds only the crates the lint job selected: a centre read added in an unselected crate is seen at the unscoped run, not at its PR
status: open
opened: 2026-11-23
priority: P3
cost: E
---


Filed by ONEROUND's `oneround/conic-readers` (PR 7510, 2026-11-23);
here because `scripts/conic_centre_census.py`, the lint job's clippy
step in `.github/workflows/ci.yml` and `scripts/gates/` are this
program's ground.

## What is true

The lint job's clippy runs twice on `$SCOPE` with
`--force-warn clippy::disallowed_methods`; the second run replays
cargo's cache as JSON into `scripts/conic_centre_census.py --check`,
which holds the production reads of a conic's centre doors per file at
`scripts/gates/conic-centre-reads.counts`.

On a scoped tier `$SCOPE` is a list of `-p` flags. The census holds a
file only where its package was SELECTED (known by the test-harness
artifact among the package's artifacts): a crate checked only as a
dependency reports a production read once or not at all, so its pins
cannot be read from that run and are skipped. Measured locally, as the
workflow spells it: `-p topo` reads 75 reads in 22 files, "1 crates
checked", against 203 in 58 files and 19 crates unscoped.

So a read added in a crate the PR's scope does not select is not seen
at that PR. It is seen at the first run that selects the crate: the
unscoped tier, and whatever nightly runs clippy over the workspace with
the same flag (none does today: `nightly.yml`'s clippy rows do not
carry `--force-warn`, and nothing there runs the census).

Also not held by the census on any tier: a library whose manifest sets
`test = false` has no harness, so it is never "selected" and its reads
are reported once; no workspace crate is so today.

## What would close it

- The census in the nightly, over the whole workspace; or
- the scope filter selecting every crate whose files the diff touches
  for the lint job (if it does already, this row is the nightly half
  only: not checked against `scripts/ci-filter.py`).
