---
id: an-all-three-mixed-corner-chamfer-at-an-oblique-shared-face-refuses
kind: issue
title: An all-three mixed corner chamfered where the shared face is oblique to the odd edge refuses: the odd strip's feet separate
status: open
opened: 2026-11-12
priority: P2
cost: M
---


Found by the bracket step-3 lane (band-bracket3, phase 1, 2026-11-12).

The ratified mixed-corner rule (`work/band/a-boss-on-a-plate-meets-it-at-a-mixed-convexity-corner-no-run-out-rounds.md`, "All three") refuses only the **fillet** where the shared face `E` is oblique to the odd edge (a canal-surface link, `SpineUnsupported`). By implication the chamfer builds there. On the step-3 cut, though, the oblique chamfer's two feet on the odd strip separate. That is the turn-overrun case at both ends of the resting strip, which the perpendicular cut does not exercise.

Step 3's first PR builds the perpendicular case only and refuses the oblique chamfer typed. This row is the oblique build. It needs a fixture with `E` tilted against the odd edge, and the mitred strip's volume in closed form.

P2: no consumer yet (the bracket's `E` is perpendicular), but the rule promises it.

- 2026-11-17 — widened to the odd edge with one pair edge (bracket step 3, cut 2, PR 7038). Since PR 7038 that chamfer is admitted by the same reading of `E` (`battery::mixed_corner_square`). So it also refuses `UnsupportedCorner { MixedConvexity }` where `E` is oblique to the odd edge. Witness: `crates/sweep/tests/band_mixed_corner_odd_plus_one.rs::the_odd_edge_with_one_other_refuses_the_fillet_and_an_oblique_shared_face`, on the `band_mixed_corner::leaning` fixture, with either pair edge. Square, it is the turn's overrun, with the odd band the far one. Oblique, the odd strip's plane meets `E` along a chord that is not square to the odd edge. The overrun machinery reads the end's section through `cap_transverse`, so that case needs its own closed form and pins. This row builds both: the all-three corner and the odd-plus-one chamfer.
