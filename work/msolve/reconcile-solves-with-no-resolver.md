---
id: reconcile-solves-with-no-resolver
kind: issue
title: The edit door's cluster-record maintenance solves with no resolver, so its re-minted frames read every relative pose as the identity
status: open
opened: 2026-09-07
---


Found by MSOLVE-6 (`docs/MSOLVE-6-SPEC.md`), which is its stop clause
(iii): a caller of `solve_document` with no resolver in hand that
cannot be given one without a design change.

`crates/editor-core/src/mate/solve.rs` `reconcile` — the D-3
cluster-record maintenance the edit door runs after any edit that can
move the mate graph (`crates/editor-core/src/edit.rs` `apply`, the
`maintenance` branch) — solves the PRIOR document to read each new
gauge's relative pose under the old mate graph, then re-mints the
cluster frame from it (`before_poses.relative(gauge).unwrap_or_default()`).
After MSOLVE-6 the solve's lever is the mated parts' own extent, asked
through `MateReach`; `apply(doc, edit, tol)` and `Doc::replay(id, edits,
tol)` are pure over the document and carry no resolver, so `reconcile`
solves through `mate::reach::NoResolver`: every mate on a part faults
`Unleverable(PartUnresolved(NoResolver))`, every relative pose reads as
the identity, and a split or gauge rewrite keeps the prior cluster
frame instead of composing the solved pose.

MEASURED: `asm_r2a_mate_solve::row4b_a_mate_delete_splits_and_re_mints_from_the_solved_pose`
and `row4c_deleting_the_gauge_rewrites_the_key_and_holds_world_poses`
(the re-minted frame lands at z = 4 where the solved pose put it at
z = 5). Every other mate row is green.

The choices are design changes, so this is Ev's call: (a) `apply` and
`replay` take a reach (the edit door and D7's replay then depend on
the store the parts resolve through — deterministic under A4's pins,
but a replay from the log alone no longer reproduces the placement
registry); (b) the maintenance leaves the edit door and runs where a
resolver is in hand (the session, the Python door), which moves a D-3
obligation; (c) the maintenance's solve is declared verdict-free — it
needs poses, not verdicts, and a lever-free fold that never refuses a
parallelism question would answer it, but that is a second solve shape
beside the one A11 names. None is taken on the MSOLVE-6 branch; the
interim there is (the honest) `NoResolver`, with the two rows red.
