---
id: nonzero-signs-readings-live-in-k-stats-apart-from-signs
kind: issue
title: NonzeroSign and its unit/flip live in k_stats.rs while Sign's readings live in predicate.rs
status: closed
opened: 2026-11-09
closed: 2026-11-09
branch: onecopy/sign-homes
pr: 6688
---


Found by the FULL review of PR 6641 (S5). It was recorded on the closed row `a-decided-signs-unit-value-has-no-home`, and the delta review asked for its own row.

`geom_core::k_stats::NonzeroSign` (`geom-core/src/k_stats.rs:924`), with its readings `unit` (`:937`) and `flip` (`:946`) and `From<NonzeroSign> for Sign`, lives in the funnel module, beside `decide_nonzero`, which returns it. `Sign` (`geom-core/src/predicate.rs:195`) and its readings, including the door `Sign::nonzero` (`predicate.rs:238`), live in `predicate.rs`. A reader looking for "the sign types and what they read as" finds them in two modules.

**The question.** Should `NonzeroSign` move beside `Sign` in `predicate.rs` (re-exported from `k_stats` for `decide_nonzero`'s callers), or is "the type the gate returns lives with the gate" the rule? This is a placement question with no behaviour in it.

## Closed

PR 6688. `NonzeroSign` and its readings moved to `predicate.rs` beside
`Sign`, re-exported from `k_stats` for `decide_nonzero`'s callers and from
the crate root beside `Sign`.
