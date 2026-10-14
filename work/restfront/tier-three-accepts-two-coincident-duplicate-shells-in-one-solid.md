---
id: tier-three-accepts-two-coincident-duplicate-shells-in-one-solid
kind: issue
title: Tier 3 accepts a solid holding two coincident copies of one shell: validate_geometric passed (X∪Z)∪(X∪W) with both copies of X kept, volume 3.25
status: open
opened: 2026-10-02
priority: P3
cost: M
---


## Finding (FUSE review of PR 3897, 2026-10-02, sure)

The reviewer mutated `shell_witness::on_kept` to keep BOTH copies of an
aligned `On` shell. The boolean `(X ∪ Z) ∪ (X ∪ W)`, with W crossing Z,
then returned a body with two coincident copies of X's shell in one
solid, at volume 3.25 where the correct volume is lower by one copy of
X. `validate_geometric` accepted it.

So tier 3 does not refuse two coincident duplicate shells. It reads
winding per shell, and two same-sense coincident copies sit on each
other's boundary, so no witness lands off them. The defect predates
PR 3897. It matters there because every `On` row leans on the
at-rest validity assert to catch a wrong keep.

What is owed: a row that hands tier 3 such a body (two shells built from
one source, same sense, in one solid) and states the verdict. Either
add a refusal (shells whose faces are pairwise coincident), or record
why it cannot be told from the winding read. Filed on RESTFRONT's slate,
which owns `crates/topo/src/validate.rs`.
