---
id: the-move-ledger-wants-its-own-row-type
kind: issue
title: step-import: the move ledger rides StructureNormalization, whose face may be a placement, a curve or a transform and whose census is a hand-built empty
status: open
opened: 2026-11-17
parent: adoption-rebuilds-caches-at-eps-from-eps-in-interpretation
priority: P4
cost: M
---

Found by the dual review of `importgate/interp-layer` (PR 7091).

## What

The move ledger (`crates/step-import/src/lib.rs`'s
`StepImport::normalizations`) rides `StructureNormalization`, a row made
for re-minted boundary graphs: a face id and a census pair. Its
interpretation kinds name no face: `PlacementReMinted` keys an
`AXIS2_PLACEMENT_3D` or an `ITEM_DEFINED_TRANSFORMATION`, `CurvePromoted`
a curve, `OneRecord` the first face on the joined record; each carries
`FaceCensus::EMPTY` twice. The `face` field's doc now has to enumerate
what it means per kind.

## Fix

A ledger row of its own: the file entity, its kind (face, placement,
transform, curve, surface record), what moved and the bound in metres,
with the census pair only on the kinds that re-mint a boundary graph;
`pncad-py`'s `StructureNormalization` binding and stubs follow.
