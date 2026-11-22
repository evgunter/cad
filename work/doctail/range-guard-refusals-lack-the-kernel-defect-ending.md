---
id: range-guard-refusals-lack-the-kernel-defect-ending
kind: issue
title: range.rs: three guard refusals that fire only on a kernel defect do not end in KERNEL_DEFECT_ENDING
status: open
opened: 2026-11-22
priority: P4
cost: E
refs: [the-certified-range-refines-away-from-the-current-value]
---

Found by OFFER's designers (2026-11-22, while weighing `work/offer/certify-affordance-on-the-bounds-panel`).

In `crates/editor-core/src/range.rs`, three `RangeRefusal` arms guard an invariant that is the kernel's own, not the caller's: `SeedIsNotTheAnalyzedAxis`, `MoreThanOneAxisVaries` and `LeavesAreNotAPartition`. Their `Display` does not end in `KERNEL_DEFECT_ENDING`. Of these guard arms, only `Derivation` carries it.

While here, fix a stale citation as a drive-by. `range.rs` and `crates/viewer/src/bounds.rs` (OFFER's) both cite `work/props/coincidence-zone-priced-budget-at-the-floor`, which now lives at `work/verdict/`.

If OFFER's certify unit replaces the query first, this row shrinks to the remaining two arms: its design fork proposes a stepwise search with no `LeavesAreNotAPartition`. (OFFER orchestrator)
