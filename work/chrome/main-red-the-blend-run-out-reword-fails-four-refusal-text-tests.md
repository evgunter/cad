---
id: main-red-the-blend-run-out-reword-fails-four-refusal-text-tests
kind: issue
title: main red: PR 6200's run-out refusal reword fails die_body's 'not built' assertion and three editor-core refusal-concision budgets
status: open
opened: 2026-11-01
priority: P0
cost: E
refs: [6200]
---


Filed by the TOUCHREC orchestrator from PR 6210's CI (run 41276137296). The same four rows fail on the unrelated `clear/refines` PR (run 41275103002), with the same messages. Main's push CI only primes the cache, so main reads green.

## What

PR 6200 (`chrome/blend-recourse-words`, merged 2026-11-01 04:46, `c43600e23`) rewrote the run-out refusal in `crates/sweep/src/blend/mod.rs`. Its commit is `6f138f204`, "sweep: say the run-out refusal and the corner recourse in a user's words". Since that merge, every PR's gate fails these rows:

- `sweep::all m5_pr12_die_body::a_subset_of_the_edges_refuses_at_the_assembly_front_door`. Its assertion that the refusal contains `"not built"` (`crates/sweep/tests/m5_pr12_die_body.rs`) fails on the new text: "fillet: a straight blend would have to stop against a curved face, which is not supported yet; it can stop against a flat face. Recourse: end the blend at a corner where three flat face…"
- `editor-core refusal_concision_chains::{every_carried_refusal_draws_within_the_budget_at_every_line, every_blend_detail_renders_within_the_budget, every_node_refusal_renders_within_the_budget}`. These are attributed to 6200 because it is the only change to blend refusal prose in that window, and the same CI runs fail them. Read the assertion messages to confirm.

## Owed

PR 6200's lane owes the fix (`memories/agent-lane-operations.md`, inherited reds). Two things to settle:

- whether the die-body row should assert the new wording, or the refusal should keep naming what is not built;
- how to bring the reworded refusal back within the concision budgets.

Until it is fixed, PRs that merge past this red cite this row.
