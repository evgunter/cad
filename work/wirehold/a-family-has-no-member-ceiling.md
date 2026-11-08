---
id: a-family-has-no-member-ceiling
kind: issue
title: A member family has no size ceiling, so a headless run with a huge count runs out of memory instead of refusing
status: closed
priority: P2
cost: H
opened: 2026-11-01
closed: 2026-11-02
pr: 6305
branch: wirehold/a-run-holds-a-member-budget-built
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

- `EvalOptions::member_budget`, default `DEFAULT_MEMBER_BUDGET = 65_536` (`eval/mod.rs`), the reason beside it: 17.5 KB and 1.1 ms × 65,536 ≈ 1.1 GB and about a minute in debug.
- Admission (`eval/mod.rs`, `Admission::admit`) charges every node in `sched.order` before any node runs (review of PR 6305: one pot, parts included). The walk then runs the nodes before the crossing one: the serial walk in order, each level filtered to them. Charging level by level, as first written in the fork, would let a level hold a node later in `sched.order` than a node of a later level, and the two schedules would cross at different families (witness `a_family_a_level_down_and_later_in_order_crosses_after`). Families are listed over the nominal environment, so the charge is the document's. The trace reader's nominal run (`nominal_ev`) is a second pot under the same budget, so a trace-reading document at a non-nominal lane lists up to twice the budget; it runs out at the same node.
- The charge reuses `members::listing` (the walk `members` now filters): every index value bound yields one item, so an outer index over empty inner ones is counted step by step. The innermost index's bind is skipped in the charge (no count reads it).
- One pot, parts included, charged by listing and not by building, at every build the build makes (rulings on the reviews of PR 6305). An instance at the top is charged the cost of building its part, unless the top has charged that part already (its instances share one build). A part's cost is the descent `PartStore::resolve_and_evaluate` runs, walked with nothing built (`PartStore::charge`, the same `Entered` stack and refusal rules): its own families' listing steps, its parts charged wherever the descent builds them, shared within one document and not across two (deliberate: commit `2871d662e4`, a row is decided against its document's own chain), so a lattice of documents that list members is charged exponentially and refuses. The cost is a function of the pinned content, kept per key across runs (`Carried::costs`). Only once the whole charge fits is any part built, and each build runs under exactly what it was charged: running out there is an argued `unreachable!`. A part whose build refuses is charged what it lists.
- Remaining gap: a lattice of member-free parts is charged 0 and builds up to 2^depth times (bounded only by `MAX_DEPTH` = 1024). It was already unbounded on main and is not widened here; filed as `wirecost/a-lattice-of-member-free-parts-builds-uncharged`.
- Wired: viewer `Landing::Exhausted` / `Outstanding::Exhausted` / `Progress::Exhausted` (own toolbar line, no Re-evaluate); pncad-py `Evaluation.exhausted` and `evaluate(member_budget=)`; stackup `BindingViolation::Exhausted`; coincidence replay `Unreplayed::Exhausted` with `Recourse::MemberBudget`, per row (rows before the crossing still prove). `NodeStanding::NotEvaluated`'s sentence covers both stops.
- The viewer's staged seam answers an exhausted stage (`Staged::Exhausted`), so a draft over the budget does not read as checking forever.
