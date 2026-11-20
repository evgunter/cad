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

## A started draft, handed back (2026-11-02, WIRE)

Dispatched and stopped under the sitting rule; nothing was pushed. The plan the draft took: rename the seven Unruled variants `Unsupported*`, keeping the Python tag words (the prefixed siblings do not spell "unsupported" there either). `PartnerUnread` is in the Limit category (a precision limit), so it is not this row's.

Moved from WIREREFUSE to DOCTAIL on 2026-11-20, by `git mv`, when WIREREFUSE merged into DOCTAIL: WIREREFUSE held no P0 or P1 row, so no sitting would dispatch it on its own (`work/README.md`, Track size), and its charter is one DOCTAIL carries.
