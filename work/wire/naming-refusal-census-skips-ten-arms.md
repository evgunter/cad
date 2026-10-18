---
id: naming-refusal-census-skips-ten-arms
kind: issue
title: wire: the feature-tree refusal census skips ten of NamingError's arms
status: open
opened: 2026-10-18
priority: P3
cost: E
---


## What

`crates/editor-core/tests/refusal_concision_chains.rs` (`editor_payloads`,
the `naming` list) renders `Duplicate`, `Unnamed`, `MissingUpstream`,
`Emission`, `SplitLineage`, `SeamVertexParentage`, `SharedRim`, `Band` and
`Escalated` of `NamingError` (`crates/editor-core/src/names/emit.rs`),
and the module header claims every arm of a forwarded refusal. Ten
arms have no row: `Settle`, `SeamVertexPartners`, `MergedChord`,
`MergedChordOffRim`, `MergedChordConstituents`, `ConventionalVertex`,
`ClosedCarrierUnread`, `CarrierUnplaced`, `PartnerUnread`,
`MemberEdgeTied`.
Each now ends in `NamingError::ending`'s sentence, but nothing holds
their length, key dumps or recourse to the standard.

## Repair shape

Add a row per arm. The missing-rule arms print `{key:?}` the way
`SeamVertexParentage` and `SharedRim` do, so each wants a
`KERNEL_KEYED` entry, or the keys said in words. Then the guard's
must-fire check keeps the list honest.
