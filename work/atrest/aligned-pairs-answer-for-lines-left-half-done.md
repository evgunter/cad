---
id: aligned-pairs-answer-for-lines-left-half-done
kind: issue
title: PR 5779's aligned-pair closure is spelled twice in Declared, its prose cites the old premise, and pass 4's face-through-edge wedge may re-report an aligned pair
status: open
opened: 2026-10-26
priority: P3
cost: M
refs: [two-copies-filling-one-volume-certify-at-rest]
---


Found by the post-merge review of PR 5779 (minor 2 and the style findings).

- **Unscheduled blind spot (PR 5779's body, "Blind spot").** Pass 4's `edge_in_own_face` / `FaceThroughEdgeWedge` may re-report an aligned pair's overlap, the way pass 5 did before #5779. The reviewer's half-overlap bricks probe (`review_probe_bricks_overlapping_by_half`) did not duplicate. Find a shape that does, or show that none can, and add the row.
- **Two spellings of "a pair holds q".** `Declared::overlap_holds` is a near-copy of `Declared::holds`: the same loop, `contains` and `in_overlap_closure`, the errors discarded, and the `too_many_arguments` allow copied. Give them one home.
- **Three hand-filled nested parallel sets.** `Declared` keeps `confirmed ⊇ area ⊇ aligned`, each in both orientations, filled by hand in `confirm`. One map from pair to `PairContact` would make the nesting structural.
- **Stale prose citing the old premise:**
  - `census.rs` near 2152: tier 3′ reads "every overlap pass 5 certifies";
  - near 5235: `ee_overlap_sides` "refuses whatever backs the pair";
  - near 4144: "never for its sides", one line above its new exception;
  - the over-long spliced doc line near 13109.
- **`two_solids`** guards same-solid only for the wedge arms of `census_interference_solids` / `census_uncleared_solids`. The `EdgeEdgeCross` arms beside them don't have the guard. Add it, or say why crossings are cross-solid by construction.
- **Pin arm 2's decided cross-solid `Said::Wedges` → MixedTouch with more than the pinwheel row.** #5529 pinned it with three rows, and now only one gates it. This rides with `a-touch-the-analysis-decides-crossing-is-read-as-undecidable`, which changes that mapping.
- **One connected overlap reported twice at tier 3′.** `coincident_line_wedges`' leaning-prism row pins its z-axis wedge overlap and the cap sliver as two findings, where D10 `[assertions]` has one finding per connected overlap. The door folds them, so this is harmless there. Decide whether tier 3′ should fold them too.
