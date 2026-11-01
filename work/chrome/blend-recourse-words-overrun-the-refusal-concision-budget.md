---
id: blend-recourse-words-overrun-the-refusal-concision-budget
kind: issue
title: PR 6200's blend recourse words put blend refusals over the 75-word concision budget, so refusal_concision_chains is red on main
status: open
opened: 2026-11-01
priority: P1
cost: E
refs: [6200]
---


**Main is red.** Three rows of `crates/editor-core/tests/refusal_concision_chains.rs`
fail on `origin/main` at `fbfebd92` (reproduced on main's own tree by the
SPACES orchestrator, 2026-11-01):

- `every_carried_refusal_draws_within_the_budget_at_every_line`
- `every_node_refusal_renders_within_the_budget`
- `every_blend_detail_renders_within_the_budget`

Blend refusals now run 88–95 words against the 75-word budget. The added
words are the recourse "end the blend at a corner where three flat faces
meet…", from `6f138f2049` (PR 6200, `chrome/blend-recourse-words`). Main's
own CI did not select these rows (its diffs did not touch editor-core), so
the first PR that did, SPACES' PR 6208, went red on them.

The lane that caused it owes the fix (`memories/agent-lane-operations.md`,
merging rules): shorten the recourse, or argue the budget, but not both by
default. Until it lands, every PR touching `editor-core` inherits the red.
(SPACES orchestrator)
