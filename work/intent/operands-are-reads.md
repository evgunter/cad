---
id: operands-are-reads
kind: issue
title: D10 stage 2 PR B: every operand field holds a VarId read of an output; Node::inputs becomes Doc::reads/upstream; a delete leaves its readers unresolved, typed
status: parked
opened: 2026-10-07
priority: P0
cost: H
design: true
blocked_on: [operations-define-output-variables]
refs: [d10-one-way-to-say-intent-is-unbuilt]
---

INTENT stage 2, PR B. Spec: `docs/INTENT-STAGE2-SPEC.md` §3.

Every operand field (`profile`, `target`, `a`/`b`, `members`, …) holds a `VarId`
read of an output, kind-checked at the door. `Node::inputs()` becomes
`Doc::reads`/`Doc::upstream` at all 37 call sites. Deleting an operation leaves its
readers unresolved and typed, reported as `Maintenance::Strand` (D10; Q1), and
`DeleteWouldDangle` retires. The roots are byte-equal before and after, which is the PR's check.
Gauges and mate sides are not converted here (Q3; PR F).

`design: true`: FORK-4 (re-pointing an operand against REFERENCES DM6). B keeps DM6
until it is ruled.
