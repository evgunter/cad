---
id: criterion-selftest-fixture-cannot-see-a-mean-for-median
kind: issue
title: criterion-emit.py's selftest fixture plants mean == median, so a median/mean swap in collect() passes it
status: open
opened: 2026-09-11
refs: [criterion-selftest-nightly-only]
---


Found by the unit that promoted `scripts/criterion-emit.py --selftest` into
the per-PR gate, by injecting failures into the script and watching which ones
the row caught.

## The finding

`scripts/criterion-emit.py:265` `plant()` writes one number into three places:

    "median": {"point_estimate": median, ...},
    "mean":   {"point_estimate": median},

so every fixture row has `mean == median`. `collect()` reads the two into
separate columns at `scripts/criterion-emit.py:121` and `:123`
(`median_ns`, `mean_ns`), and the selftest's assertion at `:290` is

    if rows["a/one"]["median_ns"] != 100.0:

which cannot distinguish them. MEASURED, not argued: editing `:121` to read
`mean.get("point_estimate")` — the exact shape of a regression that would fill
the history's `median_ns` column with means — leaves `criterion-emit.py
--selftest` printing `ok` and exiting 0.

The `median_ci_ns` pair IS discriminated (the fixture derives the bounds as
`median * 0.9` / `* 1.1`, and swapping `lower_bound` for `upper_bound` at
`:122` fails the row), so the defect is in the two point estimates only.

## Why it matters more now

The selftest is a merge-gate row as of this item's sibling, so what it cannot
see is what the gate cannot see, on a script whose output is appended to a
history under `docs/perf-data/criterion/` that cannot be edited afterwards. A
`median_ns` column silently carrying means is exactly the shape of drift the
environment block exists to make readable — and a reader has no way to tell
from the file which estimator produced a number.

## The fix

One line in the fixture: plant a `mean` that differs from the `median` (the
real ones do — a criterion row's mean sits above its median), and assert
`mean_ns` as well as `median_ns`. It is the same repair the confidence
interval already has.
