---
id: boolean-and-census-spell-the-normal-at-refusal-and-the-cut-order-by-hand
kind: issue
title: NormalAtError is mapped to a refusal by hand at six sites, and straight-span cuts are insertion-sorted by hand outside order_cuts
status: open
opened: 2026-10-30
priority: P3
cost: E
---


Recorded by the second review of PR 6061 (keel on a bore's ruling); out
of that PR's scope by the orchestrator's ruling.

- **`NormalAtError` mapped by hand.** The review counted six spellings of
  `crate::face_normal::face_outward_normal_at`'s refusal mapped to a
  caller's error (`Escalated` → an escalation, the rest → a typed
  no-normal), `boolean/slit.rs::walls_inside`'s `normal_at` among them.
  One door per error family would keep them from drifting.
- **Cut orders by hand.** `census.rs`'s `order_cuts` serves pass 4's
  straight cuts and the curved arm; `along` (`pm_census_arc_cut_order`,
  a metered step along an arc) and any other insertion sort read through
  a trilean comparator stay hand-rolled. Fold where the question is the
  same.
