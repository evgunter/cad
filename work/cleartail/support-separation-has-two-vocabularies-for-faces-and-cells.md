---
id: support-separation-has-two-vocabularies-for-faces-and-cells
kind: issue
title: Face-level and cell-level support separation are spelled twice: boolean/separating.rs's support_along and PAIR_GAP, and clearance.rs's support_separation and CLEARANCE_SUPPORT_GAP
status: open
opened: 2026-11-01
priority: P2
cost: M
---

Found by DIRSUPPORT's review (2026-11-01). Two places separate a pair
by the gap between its supports along a direction, and they share no
code or vocabulary.

- **Face level:** `crates/topo/src/boolean/separating.rs`.
  `apart_along` reads `support_along` (each face's reach in a frame
  aimed along a unit direction) and decides the padded gap at
  `PAIR_GAP` (`bool_pair_gap`).
- **Cell level:** `crates/topo/src/clearance.rs`.
  `separated_by_support` reads `support_separation`, which takes the
  hull of `⟨P, n⟩` over a cell's restricted net (`Hull::support`)
  along an unnormalized direction and divides by `|n|`, and decides at
  `CLEARANCE_SUPPORT_GAP` (audit row F29).

The cell-level door was named `apart_along` when it landed and was
renamed so the crate has one `apart_along`. The ask is one vocabulary:
one notion of a support along a direction, and one gap decision with
its pad, used by both questions. Each keeps its own funnel site where
the two readings of the gap differ.
