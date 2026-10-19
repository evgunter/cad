---
id: naming-refusal-census-skips-ten-arms
kind: issue
title: wire: the feature-tree refusal census skips nine of NamingError's arms
status: closed
opened: 2026-10-18
priority: P3
cost: E
closed: 2026-10-19
---


## What

`crates/editor-core/tests/refusal_concision_chains.rs` (`editor_payloads`,
the `naming` list) renders `Duplicate`, `Unnamed`, `MissingUpstream`,
`Emission`, `SplitLineage`, `SeamVertexParentage`, `SharedRim`, `Band` and
`Escalated` of `NamingError` (`crates/editor-core/src/names/emit.rs`),
and the module header claims every arm of a forwarded refusal. Nine
arms have no row: `Settle`, `SeamVertexPartners`, `MergedChord`,
`MergedChordOffRim`, `ConventionalVertex`,
`ClosedCarrierUnread`, `CarrierUnplaced`, `PartnerUnread`,
`MemberEdgeTied`.
Each now ends in `NamingError::ending`'s sentence, but nothing holds
their length, key dumps or recourse to the standard.

## Repair shape

Add a row per arm. The missing-rule arms print `{key:?}` the way
`SeamVertexParentage` and `SharedRim` do, so each wants a
`KERNEL_KEYED` entry, or the keys said in words. Then the guard's
must-fire check keeps the list honest.

## Resolution (2026-10-19)

Every `NamingError` arm now has a `Naming/` row in `editor_payloads`
(`crates/editor-core/tests/refusal_concision_chains.rs`), the nine named
above and `ShortTie`, added beside this fix. The list is welded to the
enum: `every_naming_arm_has_a_row` takes its roster from
`display_contract`'s `NAMING_ERROR` census (a wildcard-free `match`
rustc holds exhaustive) and names each row by its payload's variant, so
an arm added or a row removed reds. Measured: with the `MergedChord`
row taken out, `every_naming_arm_has_a_row` and
`every_admission_admits_a_row_it_is_needed_for` (its `KERNEL_KEYED`
entry unused) both red.

The rows held to the budget found five arms over 75 words
(`SeamVertexPartners`, `MergedChordOffRim`, `ConventionalVertex`,
`MemberEdgeTied`, `PartnerUnread`). The words came out of the shared
`UNRULED_FRAMING` that opens all ten missing-rule sentences ("no naming
rule covers this construction; the recipe and its result are sound"),
not out of what each sentence reports.
The seven missing-rule arms that print an arena key
print it as `SeamVertexParentage` does and have `KERNEL_KEYED` entries.
The sweep's two siblings, `PlacementRuleFault::NonRigidFrame` and
`ResolveError::ReadGone`, have rows too; the first states no recourse
and is filed as `work/wirehold/placement-rule-refusals-state-no-recourse.md`,
which also holds the other held placement, instance and face-frame
entries. Welding the other four lists is
`work/wire/editor-payload-lists-other-than-naming-are-kept-by-hand.md`.
