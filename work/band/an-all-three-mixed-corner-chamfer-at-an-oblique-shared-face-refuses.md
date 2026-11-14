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
