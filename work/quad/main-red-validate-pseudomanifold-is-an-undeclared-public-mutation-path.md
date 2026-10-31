---
id: main-red-validate-pseudomanifold-is-an-undeclared-public-mutation-path
kind: issue
title: Main red (slow set): topo's tier-1 door census names props.rs::validate_pseudomanifold since PR 5741
status: closed
opened: 2026-10-27
priority: P1
cost: E
closed: 2026-10-30
pr: 5824
---


## What

`topo`'s slow-set row
`review_m1_pr5_internal::every_public_mutation_path_preserves_tier1`
fails on `origin/main` (reproduced at `2f9e6f9e`). Slow sets do not run
in hosted CI. Found by SPLICE's slow-set triage lane (branch
`splice/slow-set-reds`).

**Bisect:** the census reads source at run time, so one test binary was
replayed over each tree.
- first bad: `00a3d070`, the merge of PR 5741 (`quad/measure-c-no-default`);
- last good: `ab2b6108`, its first parent.

**Failure** (`crates/topo/src/review_m1_pr5_internal.rs:563`),
verbatim:

    public mutation path(s) that neither declare the tier-1 debug postcondition nor open a surgery scope nor appear on this test's allowlist: ["/home/user/cad/crates/topo/src/props.rs::validate_pseudomanifold"]. Either call `assert_euler_postcondition` at the end of the door, open a surgery scope and close it with the sweep, or add it above WITH the reason tier 1 survives it — and if the reason is that it does not, that is a finding, not an entry.

PR 5741 added `Certifier::validate_pseudomanifold(&mut self, …)`
(`crates/topo/src/props.rs`). The census reads a `pub fn` taking
`&mut self` as a mutation path. The certifier holds its body as
`&'a Body<f64>`, so the door mutates only the certificate it keeps,
never a body.

## What is owed

Most likely an `ALLOWED` entry, with that reason, in
`review_m1_pr5_internal.rs`. Then check whether
`the_two_door_tables_cover_the_same_surface` wants the door in its
table too. An earlier, different cause of the same row's red
(`join_edges`) is recorded in
`work/green/main-reds-editor-core-slow-set-and-the-k-probe-sweep.md`.

## Closed

A duplicate of `main-red-the-pseudomanifold-gate-door-is-off-the-tier1-mutation-roster`, which PR 5824 closed: `validate_pseudomanifold` is on the roster's `ALLOWED` list, with the reason that the certifier holds its body by shared reference. Both `every_public_mutation_path_preserves_tier1` and `the_two_door_tables_cover_the_same_surface` pass on main `4762c685` (re-run 2026-10-30, `--run-ignored all`).
