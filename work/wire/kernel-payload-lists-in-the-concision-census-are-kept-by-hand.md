---
id: kernel-payload-lists-in-the-concision-census-are-kept-by-hand
kind: issue
title: The feature-tree census's kernel-payload lists are kept in step with their enums by hand, and four skip arms
status: closed
opened: 2026-10-19
priority: P4
cost: E
closed: 2026-10-20
pr: 5427
branch: wire/census-tail
---


Found by `wire-p4`'s sweep for the class of
`editor-payload-lists-other-than-naming-are-kept-by-hand`, whose PR
welds the editor-core payload lists (`every_forwarded_editor_arm_has_a_row`
in `crates/editor-core/tests/refusal_concision_chains.rs`).

The kernel-payload lists in the same file are still kept in step with
their enums by hand, and a scripted comparison of each list's
constructors against its enum's arms finds arms with no row:

- `split`'s `reduce` (`topo::SplitReduceError`): `LineageRow`,
  `UnjoinedOperand`;
- `split`'s `finish` (`topo::SplitFinishError`): `Merge`,
  `ResultEscalated`, `ResultNotAtRest`;
- `split`'s join arms (`topo::SplitJoinError`): `LineageRow`;
- `shell`'s `separation_reasons` (`topo::PointInSolidError`):
  `InsideOut`;
- `editor_payloads`' `resolve` samples three of `Diagnosis`'s seven
  arms (`Cascade`, `WitnessBifurcation`, `PredicateFlip`), so
  `BorderDelta`, `GroupResized`, `StructuralParam` and `RecipeEdit`
  never render inside a `Vanished` row.

`document_arms`' `parts` skips `PartFault::PartProduct` and
`PartFault::Unresolved` on purpose (each is rendered elsewhere, as the
comment above it says), so it is not a hit.

Some of these may be unreachable through a node, or covered by
another list. Each wants a reason in-file or a row.

`every_forwarded_editor_arm_has_a_row` cites this row for every
`NodeErrorKind` arm that forwards a kernel refusal (`forward`'s
`UNWELDED_KERNEL`), and asserts the file still exists.

## Repair shape

Weld each list with an `f6_variants!` roster as
`every_forwarded_editor_arm_has_a_row` does, with each row named by
its payload's arm, and add a row for each arm the roster turns up, or
name in-file why it has none.
