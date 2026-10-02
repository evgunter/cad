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
cap. Every other edge of the cap is metered before the carve
(`ring_clearance_pass` arm (c), `crates/sweep/src/blend/surgery.rs`;
`CapSliver` in `crates/sweep/src/blend/open/ruled.rs`), each over its
own window, against a region ENCLOSING the sliver: the annulus
`r ≤ ‖p − c‖ ≤ reach` about the section centre `c`, cut down to the
half-plane `(p − c)·(V − c)/‖V − c‖ ≥ floor` the sliver lies in.
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
2. **The region over-reaches the sliver.** It is exact on the D-rod's
   line-and-wall corner up to the half-plane's tilt, but it is still
   an enclosure: an edge is cleared only if it lies wholly inside the
   ball's section, wholly beyond `reach`, or wholly short of `floor`,
   so an edge that leaves the region by different faces at different
   points — e.g. one running from inside the ball's section out across
   the annulus on the far side of `c` — refuses where it is clear.
   And the half-plane's direction is `V − c`, not the chord normal of
   the two feet, which leaves a thin wedge of kept material near each
   foot inside the region on corners where the two differ.

   **Witness, the cut cycle's own edge** (the tour's `rocker`,
   `demos/tour/src/rocker.rs`, `crease_narration`, wall 2): a keyhole
   through a plate (disc `R = 1/2`, slot half-width `w = 1/5`, slot
   end `0.8` from the disc's centre). Its two convex creases carve at
   the closed form below `r* = 0.3097` and refuse `RingClearance` on a
   cap from `r*` on, with margin exactly `‖E − c‖ − ‖V − c‖`, `E =
   (0.8, w)` the slot end edge's corner nearest the crease: `−1.5e-4`
   at `r = 0.31`, `−0.00884` at `r = 0.33`, `−0.03263` at `r = 0.4`,
   `−0.05264` at `r = 0.49`. `r*` is derived from that margin, not
   read off a sweep: `E` and `V` share the wall line and `c` sits `r`
   above it, so the margin changes sign where `cx = (x0 + 0.8)/2`, and
   `cx² = x0² + 2r(R − w)` gives `r* = (cx² − x0²)/(2(R − w))`
   (`ring_clearance_onset` in the scene, which pins it from both sides
   at ±1 %).
   The slot end edge is clear of the sliver at every one of those
   radii — the sliver ends at the wall foot
   `x = √((R + r)² − (w + r)²)` (`0.639` at `r = 0.33`, `0.671` at
   `r = 0.4`), short of the slot's end — and it is the case above: at
   `r = 0.33` its upper end is short of the floor but inside `reach`,
   its lower part past the floor but beyond `reach`, so no one face
   clears it whole. The enclosure refuses an edge the band does not
   touch, and the rendered sentence says that edge "lies in the part of
   a face the blend cuts away", which it does not.

## What the taker owes

An exact sliver-membership classification for a circle ring (a
Q1 trilean with its own margin), the refusal split into "straddles the
arc" (a frontier: the band would have to be trimmed by the bore) and
"wholly inside" (built: the feature dies with the sliver), and the two
rows above flipped to carves at the closed form `ΔV = −2·A·L + V_bore`.
