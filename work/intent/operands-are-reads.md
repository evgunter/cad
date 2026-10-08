---
id: operands-are-reads
kind: issue
title: D10 stage 2 PR B: every operand field holds a VarId read of an output; Node::inputs becomes Doc::reads/upstream; a delete leaves its readers unresolved, typed
status: review
opened: 2026-10-07
priority: P0
cost: H
branch: intent/s2-b-reads
pr: 4342
refs: [d10-one-way-to-say-intent-is-unbuilt, an-operand-slot-is-re-pointed-by-the-slot-door, set-members-admits-a-forward-member-the-save-validator-refuses]
---

INTENT stage 2, PR B. Spec: `docs/INTENT-STAGE2-SPEC.md` §3.

Every operand field (`profile`, `target`, `a`/`b`, `members`, …) holds a `VarId`
read of an output, kind-checked at the door. `Node::inputs()` becomes
`Doc::reads`/`Doc::upstream` at all 37 call sites. Deleting an operation leaves its
readers unresolved and typed, reported as `Maintenance::Strand` (D10; Q1), and
`DeleteWouldDangle` retires. The roots are byte-equal before and after, which is the PR's check.
Gauges and mate sides are not converted here (Q3; PR F).

## FORK-4 ruled (2026-10-07, #4221)

DM6 now reads "no edit infers a re-point". B builds the one slot door for operand slots, under kind, liveness, acyclicity and DM5. `SetMembers` becomes that door on a list slot, and strands are reported, never refused. The design flag is cleared.
