---
id: main-red-dual-corpus-reshaped-rod-census-disagrees-at-1e-12
kind: issue
title: m10_di_dual_corpus at_rest_census_verdicts_match_f64_at_interval is red at ε 1e-12 on main: reshaped_rod's census disagrees between f64 and Interval
status: open
opened: 2026-10-28
priority: P3
cost: M
---


Found by `atrest/aligned-arc-or-curved-faces-that-abut-are-flush`
(PR 5884) while running its slow rows at three ε. Not that branch's:
the same failure reproduces with main's census sources at `765122f9`.

## What

`CAD_TOLERANCE_EPS=1e-12 cargo nextest run --ignore-default-filter -p
editor-core -E 'test(/at_rest_census_verdicts_match_f64_at_interval/)'`:

```
panicked at crates/editor-core/tests/m10_di_dual_corpus.rs:228:9:
reshaped_rod: the census disagreed across certifying scalars (f64 None vs Interval None)
  left: Some(false)
 right: Some(true)
```

Green at 1e-6 and 1e-9. The row is in the slow set, so only the
nightly reads it at 1e-12.

## What closing it takes

Find which reading of `reshaped_rod`'s census differs between the f64
and the Interval scalar at 1e-12, and whether it is a band edge both
should escalate at or a scalar-specific decision.

## 2026-10-28 — pinned on main (GREEN, `green/full-suite-pins`)

Nightly run 40637814390 (main `a1cfd1cfdb`), `full suite`; re-run on main `91c83c38`. Still red at 1e-12 with the same reading. The row is pinned at 1e-12
on `reshaped_rod: the census disagreed across certifying scalars`; it
goes red the day the row passes. Not bisected here.

Un-pinning owes a re-measure: each pin stops its row at main's first stop, so whatever runs after it in the row is unmeasured.


## Closed (ONECOPY, `onecopy/main-reds-triage`, 2026-11-09)

The row is green at 1e-12 since PR 6482 (`onecopy/angle-reads`, merge `a6f0b5e99c`: the F31 angle reads). Which of its reads moved this row was not traced. A first-parent bisect of the pinned row finds it "stopped otherwise" from at least `e9eaae651d` up to `a6f0b5e99c^1`, and "it passed" from `a6f0b5e99c` on. At `a6f0b5e99c` and on main (merged 2026-11-09) both scalars read `reshaped_rod` the same way: no could-not-look finding at f64 or at Interval (`Ok(false)` each). So the two scalars now agree, on the f64 reading. The pin is removed in `crates/editor-core/tests/m10_di_dual_corpus.rs`, and `at_rest_census_verdicts_match_f64_at_interval` passes unpinned at 1e-6, 1e-9 and 1e-12.
