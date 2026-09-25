---
id: ruled-cut-off-builds-a-bore-wholly-inside-the-removed-sliver
kind: issue
title: blend: a cap ring the ruled cut-off's sliver wholly contains refuses RingClearance; the bore vanishing with the sliver is buildable, and the meter is an enclosing annulus, not the sliver
status: open
opened: 2026-09-25
priority: P2
cost: H
---


## Finding

A convex ruled crease's cut-off removes a sliver from each transverse
cap. Every other cycle of the cap is metered before the carve
(`ring_clearance_pass` arm (c), `crates/sweep/src/blend/surgery.rs`;
`CapSliver` and `rim_reach` in `crates/sweep/src/blend/open/ruled.rs`)
against the closed ANNULUS `r ≤ ‖p − c‖ ≤ reach` about the section
centre `c`, which encloses the sliver, and by each edge's WHOLE carrier.
Anything not definitely clear refuses `RingClearance` at the cap.

Two things that meter leaves on the table:

1. **The bore wholly inside the sliver is buildable.** Its walls lie
   entirely in the material the band removes, so the right answer is
   the band with the bore gone: the bore's ring, its wall faces and
   their far-cap ring all die, and the sliver folds into the band as
   today. Rows that refuse it now:
   `band_ruled_cap_ring::a_bore_inside_the_d_rods_removed_sliver_refuses_ring_clearance`
   and
   `review_band_ruled_ring_probes::a_bore_in_a_keyhole_creases_removed_sliver_refuses_ring_clearance`.
   Building it needs (a) an EXACT classification of "wholly inside the
   sliver" (not the annulus — see 2), and (b) surgery that kills a
   through-feature — its two rings and the walls between — which no
   blend carve does today; `kfmrh`/`kef` over the bore's walls is the
   likely shape. It was not attempted in the unit that added the meter
   because (a) has no closed form in the tree yet.
2. **The annulus over-reaches the sliver.** It includes the part of
   the annulus on the far side of `c` from the old vertex — kept
   material under the ball's section — so a bore within
   `[r, reach]` of `c` but away from the corner refuses where it is
   clear. Tightening to the wedge between the rays from `c` through the
   two feet is sound only once "the sliver lies inside that wedge" is
   argued for circle rims (the rim piece stays on one side of the line
   through `c` and the foot unless it passes the circle's far point);
   a line-carrier edge is likewise metered by its infinite line.

## What the taker owes

An exact sliver-membership classification for a circle ring (a
Q1 trilean with its own margin), the refusal split into "straddles the
arc" (a frontier: the band would have to be trimmed by the bore) and
"wholly inside" (built: the feature dies with the sliver), and the two
rows above flipped to carves at the closed form `ΔV = −2·A·L + V_bore`.
