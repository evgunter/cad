---
id: range-guard-refusals-lack-the-kernel-defect-ending
kind: issue
title: range.rs: three guard refusals that fire only on a kernel defect do not end in KERNEL_DEFECT_ENDING
status: closed
opened: 2026-11-22
closed: 2026-11-22
priority: P4
cost: E
refs: [the-certified-range-refines-away-from-the-current-value]
---

Found by OFFER's designers (2026-11-22, while weighing `work/offer/certify-affordance-on-the-bounds-panel`).

In `crates/editor-core/src/range.rs`, three `RangeRefusal` arms guard an invariant that is the kernel's own, not the caller's: `SeedIsNotTheAnalyzedAxis`, `MoreThanOneAxisVaries` and `LeavesAreNotAPartition`. Their `Display` does not end in `KERNEL_DEFECT_ENDING`. Of these guard arms, only `Derivation` carries it.

While here, fix a stale citation as a drive-by. `range.rs` and `crates/viewer/src/bounds.rs` (OFFER's) both cite `work/props/coincidence-zone-priced-budget-at-the-floor`, which now lives at `work/verdict/`.

If OFFER's certify unit replaces the query first, this row shrinks to the remaining two arms: its design fork proposes a stepwise search with no `LeavesAreNotAPartition`. (OFFER orchestrator)

## Closed

By OFFER's kernel certify unit (`work/offer/the-certified-range-refines-away-from-the-current-value`). `LeavesAreNotAPartition` is gone with the walk it guarded: the search builds its own partition. `SeedIsNotTheAnalyzedAxis` and `MoreThanOneAxisVaries` end in `KERNEL_DEFECT_ENDING`, pinned by `docm9_range::the_guard_refusals_render_what_they_carry`. The stale `work/props/` citations in `range.rs` and `crates/viewer/src/bounds.rs` name where the rows live now (`work/verdict/`, `work/nurbs/`, `work/encl/`); the same rot elsewhere in the tree is `work/ciw/tracker-paths-in-crate-doc-comments-rot-on-a-program-re-cut`.
