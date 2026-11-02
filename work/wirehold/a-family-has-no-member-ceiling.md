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

Ev on PR 6287 (2026-11-02): "this plan broadly sounds good". On the default: "many defaults is far worse than one default or always explicit". With no stronger preference stated, the build takes one recorded default in the evaluation options (drive's `DEFAULT_MAX_LEAVES` stands), and no caller restates it.

## Build (2026-11-02, lane wire-budget, branch `wirehold/a-run-holds-a-member-budget-built`)

- `EvalOptions::member_budget`, default `DEFAULT_MEMBER_BUDGET = 65_536` (`eval/mod.rs`), the reason beside it: 17.5 KB × 65,536 ≈ 1.1 GB in debug, the drive's leaf budget's size.
- Admission (`eval/mod.rs`, `Admission`) charges in `sched.order` through a cursor. The level schedule charges through the level's last node *in that order*, so it also charges nodes of later levels that precede it there. Charging only the level's own nodes, as first written in the fork, would let a level hold a node that comes later in `sched.order` than another level's node. The two schedules would then cross at different families. Once crossed, each remaining level runs only the nodes before the crossing node.
- The charge reuses `members::listing` (the walk `members` now filters): every index value bound yields one item, so an outer index over empty inner ones is counted step by step. The innermost index's bind is skipped in the charge (no count reads it).
- A part's evaluation is a run of its own under the inherited budget (its rows are carried across runs, so it cannot share the top's pot deterministically). A part that runs out has no row; the instancing run then runs out at admission at its instance node (`parts::Reached::exhausted`), so no budget-dependent `PartFault` reaches a node or the memo.
- Wired: viewer `Landing::Exhausted` / `Outstanding::Exhausted` / `Progress::Exhausted` (own toolbar line, no Re-evaluate); pncad-py `Evaluation.exhausted` and `evaluate(member_budget=)`; stackup `BindingViolation::Exhausted`; coincidence replay `Unreplayed::Exhausted` with `Recourse::MemberBudget`, per row (rows before the crossing still prove). `NodeStanding::NotEvaluated`'s sentence covers both stops.
- Not done here: the viewer's staged seam (`evalseam::run_staged`) answers nothing for an exhausted stage, as for a canceled one.
