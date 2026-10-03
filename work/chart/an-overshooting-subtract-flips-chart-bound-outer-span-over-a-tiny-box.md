---
id: an-overshooting-subtract-flips-chart-bound-outer-span-over-a-tiny-box
kind: issue
title: A subtract whose tools overshoot the target refuses a 1e-9 whole box FlipCrossing, chart_bound_outer_span diverging 0 → 2 on both boolean nodes
status: open
opened: 2026-10-03
priority: P2
cost: M
---



Measured by the review of PR 3922 (SHOW `the-plate-document-never-cuts-its-holes`),
not yet pinned in the tree.

The tour's two-hole plate (`demos/tour/src/plate.rs`; an 8 × 4 × 1 mm
blank, two r = 1.25 mm holes whose spacing and radii are the study's
parameters) with each hole extrude moved to z = −0.5 mm and made 2 mm
deep, so the tools overshoot both caps and there is no flush pair to
declare, subtracted by two `Boolean(Subtract)`s. Over the whole box at
`1e-9` of the study (one leaf) the drive refuses `FlipCrossing`:
`chart_bound_outer_span` diverges 0 → 2 from the witness's verdict
vector on both boolean nodes. A box that narrow cannot cross a real
sign change of the outer span, so the flip is the lane's, not the
part's.

Independent of the subtract's volume-bound tie a full-depth cut meets
(`work/reach/a-hole-wholly-inside-its-target-ties-the-subtract-volume-bound.md`):
that tie does not arise here, and this refuses anyway. Possibly the
same mechanism as `chart-bound-outer-span-decides-a-poisoned-margin.md`
(an `Invalid` outcome on one side of the vector); unmeasured.
