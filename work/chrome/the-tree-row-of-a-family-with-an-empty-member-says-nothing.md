---
id: the-tree-row-of-a-family-with-an-empty-member-says-nothing
kind: issue
title: The tree row of a family of bodies with an empty member says nothing of the empty member
status: open
opened: 2026-11-06
priority: P3
cost: E
---


Found by the REFS lane that unified a family of bodies onto one payload (`work/refs/a-family-of-bodies-has-two-payloads.md`, 2026-11-06), from reading. It is a question of what the row shows, so it is left for a ruling rather than picked.

**The state.** A family of bodies is `ValuePayload::Instances(Vec<Option<Framed>>)`, `None` at a member that is a boolean's empty result (`F[k] − F[1]` over a row: member 1 is empty). The tree's readout (`crates/viewer/src/tree.rs` `readout_of`) answers `None` for every `Instances`, so the row of such a family says nothing. Every seat that reads the family whole refuses it `EmptyMember`, naming the member.

The neighbouring rows do say so. A boolean's empty result reads `Readout::Empty(Emptiness::Whole)` ("empty"), and a split with an empty side reads `Emptiness::Half` ("above half empty"). `Readout::Empty`'s doc gives the reason: an empty value is "the one a consumer refuses as its input, so the row it came from says so". That reason covers an empty member, but the tree does not settle how such a row would spell it.

**The options.**

1. **No readout**, as today. The refusal at the reading seat names the member.
2. **`Emptiness::Member`, naming the empty members by their index values**, spelled as `EmptyMember`'s refusal spells them (`k = 1`), e.g. "k = 1 empty". If every member is empty, the readout is `Emptiness::Whole`, as for a split with neither side. This needs `Display` to reach the document's spoken names for the indices, which `Emptiness`'s `Display` does not do today.
3. **A count**, e.g. "1 member empty". It needs nothing from the document, but it does not name which member.

A pin for any of them can reuse the empty-member document in `crates/editor-core/tests/g_family_of_booleans.rs`.
