---
id: operations-define-output-variables
kind: issue
title: D10 stage 2 PR A: an operation defines variables — VarDef::Output { node, port }, the reference kinds, outputs minted at insert without moving a node id, the OutputSignature load walk
status: parked
opened: 2026-10-07
priority: P0
cost: M
design: true
blocked_on: [no-dimensioned-literal-in-a-slot]
refs: [d10-one-way-to-say-intent-is-unbuilt]
---

INTENT stage 2, PR A. Spec: `docs/INTENT-STAGE2-SPEC.md` §0, §2.

Every node defines its output variables (`VarDef::Output { node, port }`), of the
reference kinds FORK-1 settles. Outputs are minted at `InsertNode` from the insert's
own chain step (Q6), so no node id moves, and the `OutputSignature` load walk checks
them. Nothing reads them yet.

`design: true`: FORK-1 (the output signature of an operation) is open and blocks this row.
Parked on stage 1's last unit.
