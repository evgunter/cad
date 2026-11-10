---
id: main-red-two-dual-interval-corpus-rows-overflow-the-stack
kind: issue
title: Two slow-set Dual<Interval> rows overflow the test thread's stack on main
status: open
opened: 2026-11-10
priority: P2
cost: E
---


Found by `onecopy/dual-tier3` (PR 6735) and reproduced on main at `a7a2f9698e`, on its own build. Both rows are in the slow set (`--ignore-default-filter`), and both abort with `has overflowed its stack` / `SIGABRT` on the test thread:
- `editor-core` `m10_di_dual_corpus::dual_interval_evaluates_with_the_interval_value_channel` (the `die` and `loft_prism` documents at `Dual<Interval>`);
- `editor-core` `r2_m10_di_probes::deep_value_channel_identity_interval_vs_dual_interval`.

Unmeasured: the depth the evaluation reaches at `Dual<Interval>` against `Interval` on the same documents, and since when the rows overflow. Next: run each with `RUST_MIN_STACK` raised to see whether it is depth or recursion that does not end, then bisect on main.
