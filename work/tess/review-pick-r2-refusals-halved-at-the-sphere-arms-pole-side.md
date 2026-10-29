---
id: review-pick-r2-refusals-halved-at-the-sphere-arms-pole-side
kind: issue
title: review_pick_r2's refused candidates halved at #5920 (the sphere arm keeps a pole as a polygon side): is that the mesh the arm should produce?
status: open
opened: 2026-10-29
priority: P3
cost: M
refs: [review-pick-r2-tally-moved-at-5225-azimuth-placement]
---


Found by GREEN's nightly-76 lane. The evidence is on PCERT's row,
`work/pcert/review-pick-r2-tally-moved-at-5225-azimuth-placement.md`
(the 2026-10-29 GREEN section).

## What

`viewer` `review_pick_r2::the_certified_determinant_refuses_no_genuine_crossing_over_the_corpus`
counts, over the corpus, the candidates the pick's certified
determinant refuses. At **#5920** (`tess/sphere-tilted-circle-lane`,
merge `148c1a24f`, against its first parent `890d10746`) the tally moved
from `(392112, 131343, 26058, 8292)` to `(392112, 131373, 12996, 6342)`:
- the ray count held;
- 30 more rays are answered at the aimed vertex;
- the refused candidates halve (26058 → 12996);
- 1950 fewer rays carry a refusal.

No genuine crossing is refused at either commit. Main has since
re-pinned the row over a new corpus (#6023), so main is green on it.

## The question

Is the halving what #5920's sphere mesh should produce? Fewer
near-degenerate candidates around a pole kept as a polygon side would
explain it, but nobody has read it.

Read the refused candidates per landing at `890d10746` against
`148c1a24f`. If the move is what the mesh should do, close this row
with that reading. If not, fix the mesh.

Reproduce at each commit: `cargo test -p viewer --test all --
--nocapture review_pick_r2::the_certified_determinant` (prints the tally).

