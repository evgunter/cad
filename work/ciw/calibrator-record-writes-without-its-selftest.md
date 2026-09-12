---
id: calibrator-record-writes-without-its-selftest
kind: issue
title: nightly.yml runs opt-level-calibrate.py record with no selftest before it, on refs the merge gate never saw
status: open
opened: 2026-09-11
refs: [criterion-selftest-nightly-only, 2124]
---


Disclosed by the unit that promoted `scripts/criterion-emit.py --selftest`
into the per-PR gate and KEPT the nightly's copy. The reason written at that
row applies to the calibrator too, and the calibrator has no such row — so
this item is the asymmetry that reasoning creates, stated rather than left
implicit.

## The finding

`.github/workflows/nightly.yml`'s `criterion` job runs
`scripts/criterion-emit.py --selftest` immediately before the emit, and
`.github/workflows/ci.yml`'s `discipline` job runs it on every PR. The
calibration lane has only the second half: `scripts/opt-level-calibrate.py
--selftest` runs at `ci.yml:1483` and `local-scripts/ci-local.sh:470`, and
nothing runs it inside `nightly.yml`'s `opt-level` job, whose `record` step
(`nightly.yml:1273-1284`) appends a sample to `docs/perf-data/opt-level/`.

## Why the gate row is not the whole answer

Both nightly jobs also run on a `workflow_dispatch` `ref` — an arbitrary SHA,
which is the point of that input — and the merge gate cannot have proved a
tree that predates the row. A dispatched ref writes no history (both commit
steps refuse one), so what is at stake there is the artifact and the step
summary a reader takes for a measurement rather than the append itself.
`main` is additionally a composition of merges each gated against the base it
was opened on, which is the cost Ev's 2026-09-07 F3 ruling accepted on the
record.

## The two shapes

1. **Add the row**, next to `record`, for the reason the criterion job's
   selftest step now states at its own key. It is stdlib python against
   fixtures and costs no minutes in a job that builds the suite twice.
2. **Delete the criterion job's copy instead**, and say that a guard proved by
   the merge gate is proved, dispatched refs included. That is the reading the
   calibrator's per-PR row already asserts in prose — *"a second copy at 03:00
   would re-prove a tree this row already proved"* (`ci.yml:1464-1465`) — and it is
   the sentence this unit declined to follow.

Whichever is taken, the two lanes should give the same answer: today they
differ only because one of them was wired first.

`ci.yml:1467-1475` now marks that sentence as contested at the sentence itself,
so a reader of the workflow alone does not get two rules with no sign that one
is disputed. The marker names this item; neither row waits on it.
