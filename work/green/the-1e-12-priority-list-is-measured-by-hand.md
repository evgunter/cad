---
id: the-1e-12-priority-list-is-measured-by-hand
kind: issue
title: The ci profile's priority list of the longest 1e-12 tests is chosen by hand from junit reports, with no record that re-derives it
status: open
opened: 2026-11-14
priority: P4
cost: E
---

Filed by GREEN's `green/nightly-jobs` lane (2026-11-14), from the review
of PR 6932.

## What

`.config/nextest.toml`'s `[[profile.ci.overrides]]` gives `priority = 100`
to twelve `sweep::all` tests that each took 500 s or more at eps 1e-12 in
the nightly's junit reports, so the 1e-12 shards do not end on one of
them started last. `scripts/check-slow-set-names.py` holds the names to
the listing, so a rename is red. What nothing does is re-take the
measurement: a test that grows past 500 s is not added, one that shrinks
is not dropped, and every edit to the list sends that PR's gate to tier
`all` (`scripts/ci-filter.py` bails on an edit outside the slow-set
filter).

Neither costs a test, only a shard's wall time.

## What would settle it

The 1e-12 legs' `full-suite-junit-1E-12-<k>of3` reports carry every
test's time. A step that reads them and reports the tests at or past the
threshold that the list lacks (and the listed ones under it) would make
the list a checked record rather than a hand measurement; whether ci-filter
should treat an override-only edit like a slow-set edit is the other half.
