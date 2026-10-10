---
id: select-defines-face-and-edge-variables
kind: issue
title: D10 stage 2 PR E: Select defines Face/Edge variables and owns the N5 ladder; fillet, chamfer, shell, face-frame and measure names become reads
status: closed
opened: 2026-10-07
priority: P0
cost: H
blocked_on: [measure-is-an-operation]
pr: 4523
branch: intent/s2-e-select
closed: 2026-10-10
---

INTENT stage 2, PR E. Spec: `docs/INTENT-STAGE2-SPEC.md` §6.

`Face`/`Edge` kinds and `Select { body, name }`. The eval-time N5 ladder runs in the
select and nowhere else, except for declared pairs (Q2, stage 4). Fillet/chamfer
selections, shell open faces, face-frame datums and measure refs read selects.
`Rebind` edits selects.

## FORK-3 pending (2026-10-07, #4222)

FORK-3 is with Ev on #4222 (`a-selection-is-a-definition-of-a-body-s-faces-or-edges`). The recommendation there: a selection is a definition, not a node; sets (`Faces`/`Edges`) or singletons; distinct by authoring; `Rebind { body, from, to }`. Spec §1 and §6 follow it, pending the ruling.

## FORK-3 ruled (2026-10-08, PR 4222)

Sets. A selection is a definition (`Select { body, names }`), not a node; a fillet or chamfer reads one `Edges` and a shell one `Faces`, stating the body once, and the three lose `target`; `FaceFrame` and a mate side read one `Face`; a `Measure` operand reads one selection of the kinds its primitive admits, or a `Body`; a selection authored twice is two variables (the GUI offers the existing one); `Rebind { body, from, to }`.

## Carried from unit D (PR 4355), first from unit B (PR 4342)

Fold `NodeErrorKind::UnresolvedSite` into `UnresolvedRead` here, once a measure reads a `Face`/`Edge` variable rather than naming a node: a dead site is then an unresolved read like any other (`eval/mod.rs`'s two refusals become one, with `eval/class.rs`, pncad-py `tags.rs` and the viewer's `tree.rs` arms).

## Closed

By INTENT stage 2 unit E on `intent/s2-e-select`. A selection is `VarDef::Select(Select { body, names })` of kind `Face`, `Edge`, `Vertex`, `Faces` or `Edges`; `Fillet`/`Chamfer` read one `Edges`, `Shell` one `Faces`, `Datum::FaceFrame` one `Face`, and a `Measure` reference a body or a one-name selection. The N5 ladder runs in `eval/wire.rs`'s `select` (refusals `SelectResolve`, `SelectKind`); `UnresolvedSite` folds into `UnresolvedRead`; `Rebind { body, from, to }` rewrites a body's selections in place. Rows: `crates/editor-core/tests/intent_s2_e_select.rs` (spec tests 17 and 18, and FORK-VTX's door table).
