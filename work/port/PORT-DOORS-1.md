---
id: PORT-DOORS-1
kind: unit
title: One order and one list at the assembly doors: widen the mint refusals, ask kind before tie
status: closed
opened: 2026-09-15
refs: [2090, 1854]
branch: port/doors-1-refusal-order
pr: 2635
closed: 2026-09-15
---


The unit that carries PORT's two assembly-door findings. Both are
`crates/editor-core/src/assembly.rs`, both change what a refusal says,
and answering one without the other leaves the two doors disagreeing in
a new way — so they are one unit, not two in sequence
(`work/port/plan.md`, Order).

## The two findings

- `assembly-door-raises-only-the-head-of-each-refusal-list` —
  `assemble_gathered` raises the head of `carried_unminted` and of
  `unminted` and drops the rest, in a function whose third arm carries
  every finding it has.
- `product-table-answers-a-tie-before-kind-the-operand-the-reverse` —
  `resolve_face` asks tie-before-kind for the product's own rows;
  `operand_answer` asks kind-before-everything one node below.

Each row carries the decision taken for it (PORT orchestrator,
2026-09-15) and the evidence behind it. Neither decision is reopened by
this unit; the spec is how they land.

## Spec

`docs/PORT-DOORS-1-SPEC.md`, deleted at merge per `docs/DOC-LEDGER.md`.

## Territory

`crates/editor-core/src/assembly.rs` is **EDIT's** and the pncad-py
façade half is **LIB's**. PORT claims no paths; both are announced in
the PR and either program may take the unit instead.
