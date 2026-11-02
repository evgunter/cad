---
id: a-family-has-no-member-ceiling
kind: issue
title: A member family has no size ceiling, so a headless run with a huge count runs out of memory instead of refusing
status: open
priority: P2
cost: M
opened: 2026-11-01
---


Split out of `a-pattern-count-has-no-upper-bound-at-the-loop-that-uses-it`
(closed by PR 6267, which made `eval_family` check the cancel token before
each member, `crates/editor-core/src/eval/members.rs:222`). Cancelation
does not cover a headless run (pncad, scripts, CI, replay on load): nobody
cancels it, and running out of memory aborts instead of refusing (D9).

Measured on PR 6267 (debug, a ring of N copies of one block): about 1.1 ms
and 17.5 KB per member, linear; N = 100,000 took 113 s and 1.85 GB RSS.
`ADMITS` (`eval/repetition.rs:88`) allows 4.3e9 members per index.

The lane's recommendation, for a designer pair (Opus + Fable) before any
build: a total member count per family, `MEMBERS_PER_FAMILY` near 2^16,
refused in `eval_family` from a running count beside the cancel check, with
`Doc::member_envs` (`repetition.rs:355`) using the same bound, as a typed
`FamilyTooLarge { at_least, bound }` whose recourse is lowering a count or
splitting the family, in line with `IndexCountOutOfRange`. A per-index
range alone would not bound nested indices.

## Design fork (2026-11-02, WIRE orchestrator)

Weighed by two designers over three reconciliation rounds; fork-log row 235, `[ev]` PR on branch `wirehold/a-run-holds-a-member-budget`. Agreed: no cap in the document. A run-wide member budget in the evaluation options counts the index values bound while families are listed, charged at admission in the schedule's order before anything is built. Running out is `EvalOutcome::Exhausted { at, held, budget }` beside `Canceled`, keeping the completed prefix. Evaluations run inside a door take the caller's stop condition (`kernel-run-evaluations-mint-their-own-cancel-token`). Cheaper members are a separate row. Open for Ev: whether the budget has a recorded default.
