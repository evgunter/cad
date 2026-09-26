---
id: product-per-part-gate-counts-solids-but-gates-sources
kind: issue
title: the product's per-part gate counts SOLIDS but gates SOURCES, so a lone multi-solid source is gated twice on one geometry
status: open
opened: 2026-09-26
priority: P3
cost: D
---


Found by the style review of PR 3280, which gave the F8/D7 per-part
policy its home in `topo::per_part_gate_owed` (`crates/topo/src/instance.rs`).

**The two callers mean different things by a part.** The policy counts
the aggregate's SOLIDS and skips the per-part gate at one, because one
solid is one subject.

- `step_import::import_step` gates each placed instance, and an
  instance is exactly one solid (`topo::graft_disjoint` refuses
  anything else). Its part count and its solid count are the same
  number.
- `editor_core::product_recorded` (`crates/editor-core/src/product.rs`,
  pass 2) sums `solids().count()` over its sources and passes that to
  the policy, but the part it gates is a SOURCE body, gated whole. A
  source can carry several solids (an instantiated sub-assembly, a
  `PlacedUnion`).

So a product whose only root is one multi-solid source is gated twice
on the same geometry: once as the part (tier 3, refusing
`ProductError::SolidInvalid`) and once as the aggregate (tier 3,
refusing `ProductError::ProductInvalid`). The first always answers
before the second.

**The decision** is which count the product passes.

- **(a) Solids, as today.** The call reads as the policy's own words,
  and the lone multi-solid source pays one extra tier-3 pass. Its
  refusal is `SolidInvalid` naming the root.
- **(b) Sources, the per-part SUBJECT count.** A lone source of any
  solid count is one subject, so the per-part gate is skipped as an
  identity, which is the policy's own reason applied to what this
  caller actually gates. The refusal a lone multi-solid source raises
  changes from `SolidInvalid` to `ProductInvalid`, which consumers
  (`ProductErrorKind`, the viewer's badge, `pncad-py`'s tag) can see.
  The policy's doc would then say its argument is the per-part subject
  count, not the solid count, and step-import's call would be unchanged
  (its subjects are its solids).

Because (b) changes a refusal a caller can observe, it is a decision
and not a fix.

**What moves if (b) is taken:**
`crates/editor-core/tests/per_part_gate_policy.rs`,
`a_lone_multi_solid_source_is_gated_as_a_part_today`, pins (a)'s
answer and is written to be the row that flips. The pass-2 comment in
`product.rs` and the policy's doc both cite this row.
