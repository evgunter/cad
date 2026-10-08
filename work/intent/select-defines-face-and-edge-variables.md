---
id: select-defines-face-and-edge-variables
kind: issue
title: D10 stage 2 PR E: Select defines Face/Edge variables and owns the N5 ladder; fillet, chamfer, shell, face-frame and measure names become reads
status: parked
opened: 2026-10-07
priority: P0
cost: H
blocked_on: [measure-is-an-operation]
---

INTENT stage 2, PR E. Spec: `docs/INTENT-STAGE2-SPEC.md` §6.

`Face`/`Edge` kinds and `Select { body, name }`. The eval-time N5 ladder runs in the
select and nowhere else, except for declared pairs (Q2, stage 4). Fillet/chamfer
selections, shell open faces, face-frame datums and measure refs read selects.
`Rebind` edits selects.

## FORK-3 pending (2026-10-07, #4222)

FORK-3 is with Ev on #4222 (`a-selection-is-a-definition-of-a-body-s-faces-or-edges`). The recommendation there: a selection is a definition, not a node; sets (`Faces`/`Edges`) or singletons; distinct by authoring; `Rebind { body, from, to }`. Spec §1 and §6 follow it, pending the ruling.

## FORK-3 ruled (2026-10-08, PR 4222)

Sets. A selection is a definition (`Select { body, names }`), not a node; a fillet or chamfer reads one `Edges` and a shell one `Faces`, stating the body once, and the three lose `target`; `FaceFrame`, `Measure` and a mate side read one `Face`/`Edge`; a selection authored twice is two variables (the GUI offers the existing one); `Rebind { body, from, to }`.
