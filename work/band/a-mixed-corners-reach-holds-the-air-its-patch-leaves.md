---
id: a-mixed-corners-reach-holds-the-air-its-patch-leaves
kind: issue
title: A mixed corner's reach holds the air its patch leaves as it was, so a feature there refuses FaceClearance
status: open
opened: 2026-11-17
priority: P2
cost: M
refs: [the-reach-meters-a-mixed-corner-as-a-uniform-one-and-refuses-short-edges, 7078]
---


Found by both reviews of PR 7078 (the mixed corner's own reach).

**What.** `reach::mixed_corner_reach`'s region holds the material the patch changes, and also material the result leaves as it was. The odd band's fill and the patch cancel there, at a flush corner:
- in the fillet's case, the air quadrant's points below the torus (`ρ ∈ [r, ρ_t(z)]` about the odd axis);
- in the chamfer's case, the air triangle under the strip's plane.

At an inset corner the same holds for the boss's own material. The odd band's prism, whose window runs down to `E`, over-claims the same points. A feature standing there refuses `FaceClearance { bounded: true }` though the carve would leave it untouched.

**Witnesses** (`band_mixed_corner_reach::a_feature_in_what_the_patch_leaves_as_it_was_refuses_as_a_bound`, size 0.1):
- the flush L 2 tall with an island `[0.002, 0.006]² × [0.003, 0.01]`, filleted;
- the same L with an island `[0.01, 0.03]² × [0.003, 0.01]`, chamfered;
- the inset with the mirrored voids in its boss.

Per the reviews, islands about 3% clear of the real change also refuse.

**What the fix needs.** A region of the change alone. That is the torus section less the cut-back fill, which is not an intersection of 1-Lipschitz bounds. It could be read as two stretches, one each side of the walls' half-spaces. The odd band's prism also has to stop below the centre's plane, or be met with the torus's outside, without leaving the fill above the torus uncovered. The containment argument has to be re-made for both.

P2: refusing is sound, and the features are unusual (an island in the corner's air or a void in its boss within `2r`).
