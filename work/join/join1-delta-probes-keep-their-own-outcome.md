---
id: join1-delta-probes-keep-their-own-outcome
kind: issue
title: join1_delta_probes judges SOUND by its own outcome, weaker than common::differential::outcome
status: open
opened: 2026-10-04
priority: P4
cost: E
---


Found by PR 4031's review (S7, NOTE-3).

## What

`crates/sweep/tests/join1_delta_probes.rs` keeps an `outcome` of its
own beside `crates/sweep/tests/common/differential.rs` `outcome`. Its
SOUND requires neither the at-rest certificate nor a legal operand, and
judges the volume to 1e-6 against a 4 096-chord oracle. PR 4031's
headline ("477 lines move to OK SOUND", `join1_delta_arc_battery`) was
judged by it; the review re-judged all 477 through the differential
checks, with Richardson-extrapolated volumes, and every one is SOUND
(`review_4031_probes.rs` `r4031_arc_battery_differential`, on the
review branch). So the drift has cost nothing yet, but a battery line
reading SOUND here is weaker than the same word elsewhere.

## The shape of a fix

Route the battery through `common::differential::outcome`. Its arc
oracle has to reach 1e-7, so take the review's extrapolated chord areas
or the closed forms (segments and lenses of the shapes' circles). Then
delete the local `outcome`. The battery's lines move in their wording,
and a main-vs-head diff has to be re-baselined across that change.

## Built

Branch `join/battery-hygiene`. `join1_delta_probes.rs` judges every
line by `common::differential::outcome` and its local `outcome` is
gone. The battery line appends the body's seam counts (`samekey`,
`coplanar`, `zero`). The arc oracle is `chord_area`: chord areas at
1 024 and 2 048 chords per arc, Richardson-extrapolated. It is within
1e-10 of the segments' and lenses' closed forms (half disc, quarter,
shallow segment, lens), and of the same extrapolation at 4 096 chords,
over all 245 shape pairs.

Measured in release, main `047d10d5` against the head. Lines are
classed by verdict (`OK SOUND`, `OK BAD`, `EMPTY ok`, the `ERR`
variant):
- the arc battery, 7 350 lines: 0 moves, and all wording only;
- the brick battery, 6 000 lines: 0 moves.

The arc battery's 342 `OK BAD` are BAD under both judges, on tier 3′
alone. Their volume, certificate and operand all hold.
- 336 fail on `CensusUndecidable`. The evidence is added to
  `census-cross-solid-curved-pairs-undecidable-on-shell-results`.
- 6 fail on `UndeclaredContact { EdgeFaceOverlap }`. The evidence is
  added to
  `a-corner-pair-with-an-edge-in-the-partners-face-plane-builds-with-undeclared-contacts`.

The sweep's siblings are filed as
`sibling-batteries-judge-sound-weaker-than-differential-outcome`.
