---
id: namecheck
kind: program
title: NAMECHECK — N0's checks: the denotation verifier, the coverage fuzzer, the RoleSeg form map, the storage-order perturbation, and the clauses that point at N0
status: ready
opened: 2026-10-20
group: api/names
prefix: namecheck/
tag: (NAMECHECK orchestrator)
ab_band: 14200-14299
paths: [crates/editor-core/tests/name_invariance.rs]
keep_out: [opened 2026-10-20 by WIRE's cut (WIRE measured 69 budget points against 30 - work/README.md Track size) - the rows moved by git mv with ids and bodies unchanged and WIRE kept its P2 naming-defect rows, crates/editor-core/src/names/* is shared ground with WIRE WIREREST and WIRETAIL, names/role.rs is DOCM's by its table and the form map lands beside it by announced seam, the arena-permuting helper lands in topo behind test-support (TOPO's ground - announce it), run scripts/work.py territory on your branch and announce the seam in the PR where the changes are coupled]
---

**The checks N0 names** (PR 5455, `crates/editor-core/src/names/README.md`
N0). Each N0 principle is stated with the check that enforces it. Today
only NS1's perturbation harness exists (`tests/name_invariance.rs`, PR
5460). The rest are here:
- NS2's denotation verifier and its exhaustive `RoleSeg`-to-form map;
- NS4's coverage fuzzer;
- the eighth perturbation (storage order), which needs a topo helper;
- the pointer rewrite of the clauses N0 now states once.

These rows sit apart from WIRE's because they are checks, not naming
defects: none of them changes a name.

Slate, order and posture: `work/namecheck/plan.md`; narrative in
`work/namecheck/log.md`.
