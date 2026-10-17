---
id: under-the-guided-lift-the-symbolic-rules-measured-effects-vanish-on-the-m10-evidence-set
kind: issue
title: Under the guided lift the M10 evidence set's symbolic splits fall and the rules' measured effects (rule G's trade, rule D's reach on carrier_matches_mapped_source, the read and canonical-root dials) vanish; the profile's validation predicates leave the lane's split
status: open
opened: 2026-10-17
priority: P2
cost: M
refs: [the-pinned-profile-lift-launders-at-sym]
---


Found by the pinned-lift unit (ruling A; branch `inlane/pinned-lift-derived`). Under ruling A the evaluator derives the profile lift, and an evaluation over a box that varies an axis is guided. The M10 evidence rows replay each document's nominal over such a box at `Sym<Interval>`. So they read the profile as a function of its variables, where under the pinned lift they read the `f64` form's literals. Every number below is identical at ε 1e-6, 1e-9 and 1e-12. The rows are re-pinned to them in the same PR and cite this item.

## What moved

**Symbolic receipts fall, with the kept-atom ladder and without it alike** (`sym_9_retry_interval.rs`). The ladder retried nothing before and retries nothing now, so this is the lift, not a retry:

| document | symbolic / numeric before | after |
|---|---|---|
| two_hole_plate | 1003 / 654 | 869 / 670 |
| r1_annulus | 539 / 399 | 437 / 447 |
| r1_segment_boss | 516 / 376 | 437 / 406 |
| r2_filleted_bracket | 1387 / 1004 | 1232 / 950 |
| r2_link | 827 / 663 | 725 / 682 |

**Rule G's trade is gone** (`decide_3_split_rows_interval.rs`). On r2_link, the G-shut and shipped totals are one, [725, 682]:
- `carrier_on_surface_*`: [96, 12] / [100, 8] → [108, 0] on both sides;
- `carrier_matches_mapped_source`: [152, 28] / [156, 24] → [112, 68] on both.

r1_annulus and r2_filleted_bracket also read the same on both sides.

**Rule D no longer reaches every sample of the plate's `carrier_matches_mapped_source`** (`m10_10_pins_interval.rs`). With the algebra on it was [252, 0], all theorems; now it is [188, 64]. With the algebra off it was [196, 56], now [188, 64]. So the algebra adds nothing on this predicate. `carrier_on_surface` (off) moves [288, 72] → [296, 64]. The other three of the plate's four still discharge in full with the algebra on.

**The profile's validation predicates leave the lane's split** (`m10_bulge_interval.rs`, `m10_10`'s `line_span`): `arc_apex_identity`, `arc_diameter_clearance`, `arc_span`, `carrier_circles_identity`, `carrier_line_circle`, `chord_side`, `circle_side`, `contact_at_shared_vertex`, `line_span`, `path_circle_radius`, `path_junction_turn`, `segment_straightness` and `vertex_separation`. Under the pinned lift these were decided in the lane over the lifted literals.

The carrier rows decide less symbolically:
- d_tab: `carrier_endpoint_end` 36/0 → 24/12, `carrier_endpoint_start` 36/0 → 32/4, `carrier_on_surface_*` 126/18 → 135/9, `pcurve_envelope` 8/4 → 10/2, `witness_on_surface_*` 14/2 → 15/1, and `carrier_matches_mapped_source` 170/10 (literal) and 166/14 (parameter) → 130/50 for both. The literal and the parameter spelling no longer part.
- segment_boss: `carrier_endpoint_end` 24/0 → 12/12, `carrier_matches_mapped_source` 126/0 → 78/48.

**The read and canonical-root dials no longer decide which residuals block** (`m10_sym_profile_interval.rs`). The plate's `Plain/Report` is 80 under every dial set; it was None / 8 / None / 8. The plate's walk ledger: the largest form 28 → 10 terms, `Plain/Decision` frozen 324 → 48, `Plain/Assertion` frozen 405 → 137, `Door/Decision` 7987 → 11465 forms. The slab's `Early/Decision` line leaves.

## Reading

These rows measured the symbolic tier on documents whose profile was a constant. Some of what the rules proved were identities over the `f64` form's literals, which are theorems about a function the document does not compute (`work/inlane/the-pinned-profile-lift-launders-at-sym.md`). So a fall is not a loss of a sound proof.

But a rule's whole measured effect vanishing (rule G's trade, the dials' blocking) means its evidence rows no longer measure it.

## Owed

- Re-establish evidence for each rule (G, D on `carrier_matches_mapped_source`, `decision_read`, `canonical_root`) on a document where it acts under the guided lift, or record that it acts only on constants.
- Read why the profile's validation predicates leave the lane's split: are they consumed from the structure record by the guided replay, and decided where?
- The 64 numeric `carrier_matches_mapped_source` samples on the plate: are they reachable by a rule over the guided profile's atoms?
