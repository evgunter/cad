---
id: two-refusals-the-widened-census-finds-state-no-recourse
kind: issue
title: UnresolvedRead and ParamBox/SymbolsOverABox, first given census rows by PR 5407, state no recourse
status: closed
opened: 2026-10-19
priority: P4
cost: E
closed: 2026-10-20
pr: 5427
branch: wire/census-tail
---


Found by `wire-p4` in PR 5407. Its new census of `NodeErrorKind`'s arms
(`NODE_ERROR_KIND` in `crates/editor-core/tests/refusal_concision_chains.rs`)
and its `ParamBox` weld gave two refusals their first feature-tree row.
`every_node_refusal_renders_within_the_budget` then found that neither
states a recourse:

- `NodeErrorKind::UnresolvedRead` (`eval/mod.rs`, the `Say` arm):
  "its selection reads #…, which no live operation defines: the
  operation it read was deleted". The author's move, pointing the
  selection at a live operation, is not stated.
- `ParamBoxError::SymbolsOverABox` (`analysis.rs`, `Display`): "the
  evaluation was asked to bind every variable a symbol and to widen
  some over a box, and a run has one environment". The caller's move,
  running the two apart, is not stated.

Both sit on `FILED_NO_RECOURSE` under a comment citing this row.

## Repair shape

End each in one labelled `Recourse:`, or in the kernel-defect ending if
no caller can reach it, and take its entry off `FILED_NO_RECOURSE`.
