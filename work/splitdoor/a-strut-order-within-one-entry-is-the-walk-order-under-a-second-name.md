---
id: a-strut-order-within-one-entry-is-the-walk-order-under-a-second-name
kind: issue
title: Within one sector entry, strut_order reads the walk order under a second predicate name and a different tie rule
status: open
opened: 2026-11-01
priority: P3
cost: E
---


Found by the FULL review of PR 6226 (`splitdoor/rims-seam-orientation`).

## What

`crates/topo/src/boolean/insert.rs` `walk_faces_first` orders two germs of one sector entry with `strut_order`, which it now anchors on the corner's arrival bound. Inside one entry that reading is the walk order: both germs lie less than a half-turn round from the bound, so `strut_order` reads the sign of `(g₀ × g₁)·n` at the entry's arm. `walks_after` reads exactly the same quantity, and `walks_before` uses it for the same two germs. So one order has two homes:
- `strut_order` decides `bool_strut_order` by `decide_nonzero`. A decided zero there is the caller's tie (`|| Ok(None)` from `walk_faces_first`), and it refuses as a `Coincide::Sectors` coincidence.
- `walks_after` decides `bool_shared_cut_order` by `decide`. A decided zero there answers `None`, which `walk_order` refuses as `ClassificationInvariant` and `cut_order` reads by the cuts' flankers.

`strut_order`'s doc and `the_strut_order_agrees_with_the_walk_within_an_entry` already state the agreement.

## Direction

Within one entry, read `walk_faces_first`'s order through `cut_order`/`walks_after` alone. That retires its arrival-bound anchor and `strut_order`'s one in-entry caller; `vtxfac::germ_order` keeps `strut_order`, whose anchor there is a germ. First decide which refusal two germs along one direction in one entry owe: the coincidence (in band) or the invariant (decided zero). The two homes answer them differently today, which is why the fold was not taken as a drive-by.
