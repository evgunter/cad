---
id: the-older-missing-rule-naming-variants-lack-the-unsupported-prefix
kind: issue
title: The older missing-rule NamingError variants do not carry the Unsupported prefix D9's row-2 naming rule asks for
status: open
opened: 2026-10-21
priority: P3
cost: M
---


## What

D9's row-2 rule (`docs/DESIGN.md` `[failure.taxonomy]`): "`Unsupported*` means
'valid input, the kernel has not built this yet' and nothing else, which makes
the frontier inventory grep-able." PR 5518 gave its five row-2 `NamingError`
variants the prefix (`UnsupportedSeamVertex`, `UnsupportedCrossingSenses`,
`UnsupportedSeamSides`, `UnsupportedJoin`, `UnsupportedUnionName`). The older
variants in the same `Category::Unruled` arm of `NamingError::category`
(`crates/editor-core/src/names/emit.rs`) are the same class and do not carry
it, so a grep for the frontier misses them:

- `SeamVertexPartners`
- `SharedRim`
- `ChordOffRim`
- `MemberEdgeTied`
- `ConventionalVertex`
- `ClosedCarrierUnread`
- `CarrierUnplaced`
- `PartnerUnread`

## Next

Rename each (and its `pncad-py` tag word in `tags.rs` / `TAG_INVENTORY`, the
`display_contract` census idents, `refusal_concision_chains`' rows and keyed
list), or argue per variant why it is not row 2.
