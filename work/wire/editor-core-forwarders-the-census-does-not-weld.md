---
id: editor-core-forwarders-the-census-does-not-weld
kind: issue
title: Three NodeErrorKind arms forward an editor-core refusal the feature-tree census does not weld to its arms
status: closed
opened: 2026-10-19
priority: P4
cost: E
closed: 2026-10-20
pr: 5427
branch: wire/census-tail
---


Found by the STYLE review of PR 5407, which welds the feature-tree
census (`every_forwarded_editor_arm_has_a_row` in
`crates/editor-core/tests/refusal_concision_chains.rs`) to the
editor-core refusals `NodeErrorKind` forwards. `forward` classifies
every arm, and these three forward an editor-core refusal whose arms
the census does not weld. Each cites this row (`UNWELDED`):

- `MeasureClearanceRefused` forwards `clearance::ClearanceRefusal`, an
  enum of eleven arms with one row (`Unsupported`).
- `WitnessBifurcation` forwards `witness::WitnessBifurcation`, a
  struct whose `kind: BifurcationKind` (three arms) picks the sentence.
  One row (`FoldProximity`), plus the `Vanished(WitnessBifurcation)`
  resolve row.
- `Part` forwards `parts::PartFault` (nine arms). `document_arms`'
  `parts` list renders seven of them. `PartProduct` is in
  `part_products`, and `Unresolved` is in the viewer's
  `instance_authoring.rs`, as the comment above the list says. So the
  arms are covered, but across three lists that no roster ties.

## Repair shape

Give each enum an `f6_variants!` roster and a row per arm, and move
its `forward` arm from `Unwelded(UNWELDED)` to `welded(..)`. For
`PartFault`, weld over every list that renders it, or name in-file the
arms that render only in the viewer.
