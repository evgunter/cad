---
id: main-red-the-viewers-pick-tally-reads-six-grazes-short-of-its-pin
kind: issue
title: main red (slow set): viewer review_pick_r2::the_certified_determinant_refuses_no_genuine_crossing_over_the_corpus tallies 147,850 rays answered at the aimed vertex against a pin of 147,856
status: open
opened: 2026-11-23
priority: P2
cost: E
---


Found by TESSLANE's unit 4c (PR 7096) in its pre-push slow-set run, 2026-11-22. Filed here because the red is main's and its cause is not known.

## What

`cargo nextest run -p viewer --ignore-default-filter -E 'test(the_certified_determinant_refuses_no_genuine_crossing_over_the_corpus)'` on main at `54e6a5c5b8` (2026-11-22 19:04Z): the tally is `(451512, 147850, 16596, 8142)` against `PINNED = (451_512, 147_856, 16_596, 8_142)` in `crates/viewer/tests/review_pick_r2.rs`. Six fewer rays are answered at the aimed vertex; the ray count, the refused candidates and the rays carrying a refusal are as pinned. No genuine crossing is refused.

The same tally on PR 7096's head, so that pull request did not move it. The test file's last change before that commit is `6d6cf1916f1` (2026-11-22 04:47Z). PR CI does not run the slow set, so whichever merge moved six grazes landed green.

## What closing it takes

Find the merge between the pin's last re-baseline and `54e6a5c5b8` that moved the six rays (the row's doc comment names the bodies each earlier re-pin moved), say whether the new answers are right, and re-pin with that sentence or fix the cause. The lane that moved it owes this; until it is found the debt is recorded here.

Neighbouring rows: `work/pcert/review-pick-r2-tally-moved-at-5225-azimuth-placement.md`, `work/tesslane/review-pick-r2-refusals-halved-at-the-sphere-arms-pole-side.md`.
