---
id: operations-define-output-variables
kind: issue
title: D10 stage 2 PR A: an operation defines variables — VarDef::Output { node, port }, the reference kinds, outputs minted at insert without moving a node id, the OutputSignature load walk
status: closed
opened: 2026-10-07
priority: P0
cost: M
refs: [d10-one-way-to-say-intent-is-unbuilt]
closed: 2026-10-10
pr: 4295
---

INTENT stage 2, PR A. Spec: `docs/INTENT-STAGE2-SPEC.md` §0, §2.

Every node defines its output variables (`VarDef::Output { node, port }`), of the
reference kinds FORK-1 settles. Outputs are minted at `InsertNode` from the insert's
own chain step (Q6), so no node id moves, and the `OutputSignature` load walk checks
them. Nothing reads them yet.


## FORK-1 pending (2026-10-07, #4222)

FORK-1 is with Ev on #4222 (`operations-state-their-outputs`). The recommendation there:
- the shapes `Body`, `Bodies` and `Profile`;
- a split defines two ports;
- an instance defines one `Body` per world placement of its part.

Spec §1 and §2 follow it, pending the ruling.

## FORK-1, FORK-1b and FORK-3 ruled (2026-10-08, PR 4222)

Ev approved FORK-1's shape and FORK-1b's pose order. For this unit: the kinds gain the shapes (`Body`, `Bodies`, `Profile`) and the selections (`Face`, `Edge`, `Faces`, `Edges`) but no `AxisInPlane`; both axis datums define an `Axis`; `Revolve` defines `body` and `axis` from the start (its axis port reads the axis it is given until the line moves onto the node); a pose kind names its symmetry as the mates' `Subgroup` (A11 (1)). Stage 1 has finished, so nothing else blocks this row.
