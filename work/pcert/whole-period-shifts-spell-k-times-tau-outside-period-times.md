---
id: whole-period-shifts-spell-k-times-tau-outside-period-times
kind: issue
title: Two whole-period shifts still spell k·τ by hand instead of Period::times: Pcurve::shift_branch and fidelity's branch
status: closed
opened: 2026-10-18
priority: P4
cost: E
closed: 2026-10-18
---

Found by the delta review of PCERT B2a (PR 5244), and deferred from it: both sites predate B2a.

## What is there

C4 says a whole-period integer becomes a length only through the channel's `Period::times`. Two sites still multiply by hand:

- `Pcurve::shift_branch(k, period)` (`crates/geom-brep/src/pcurve_cache.rs:1573`) translates by `k * period`, with `k` a `T`;
- fidelity's `branch` (`:5763`) decides `k` through `Period::decide` and then turns it into a length with `T::from_f64(f64::from(k))`, multiplied by `tau` at its callers.

Each is a whole-period count made a length outside `Period::times`.

## The fix

Have `shift_branch` take `k: i32` and a `Period`, and translate by `period.times(k)`; have fidelity's branch return the `i32` and its callers take `Period::times`. B2b's S1 sweep (`shift_branch(k: i32, Period)`) is where this lands; this item records it until then.

## Discharged by PCERT B2b

B2b's S1 sweep makes `Pcurve::shift_branch` take `k: i32` and a `Period`, translating by `period.times(k)`, and every caller passes the channel's `Period`. Check 4 decides no whole period any more, so fidelity's `branch` and its `T::from_f64(f64::from(k))` are gone. Neither site spells `k·τ` by hand.

