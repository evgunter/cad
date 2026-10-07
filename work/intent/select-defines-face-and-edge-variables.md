---
id: select-defines-face-and-edge-variables
kind: issue
title: D10 stage 2 PR E: Select defines Face/Edge variables and owns the N5 ladder; fillet, chamfer, shell, face-frame and measure names become reads
status: parked
opened: 2026-10-07
priority: P0
cost: H
design: true
blocked_on: [measure-is-an-operation]
---

INTENT stage 2, PR E. Spec: `docs/INTENT-STAGE2-SPEC.md` §6.

`Face`/`Edge` kinds and `Select { body, name }`. The eval-time N5 ladder runs in the
select and nowhere else, except for declared pairs (Q2, stage 4). Fillet/chamfer
selections, shell open faces, face-frame datums and measure refs read selects.
`Rebind` edits selects.

`design: true`: FORK-3 (what a selection is: node or definition, one entity or a set,
shared or distinct; SELECT-DESIGN §4) blocks this row.

## FORK-3 pending (2026-10-07, #4222)

FORK-3 is with Ev on #4222 (`a-selection-is-a-definition-of-a-body-s-faces-or-edges`). The recommendation there: a selection is a definition, not a node; sets (`Faces`/`Edges`) or singletons; distinct by authoring; `Rebind { body, from, to }`. Spec §1 and §6 follow it, pending the ruling.
