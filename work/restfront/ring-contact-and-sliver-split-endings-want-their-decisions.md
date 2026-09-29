---
id: ring-contact-and-sliver-split-endings-want-their-decisions
kind: issue
title: validate: RingContactEscalated carries no closed decision, and SliverDihedral's material-side split ends as a defect where the geometry may reach it
status: open
opened: 2026-09-29
---


(ENCL implementer, `work/encl/validate-own-close-levers-follow-the-d4-recourse-ruling.md`.)
The rule is D4 ¶1 (i)/(iv) in `docs/DESIGN.md`.

## What

- **`ValidationError::RingContactEscalated`** (`crates/topo/src/validate.rs`)
  is raised by check 9's gap decisions (`ring_outer_vertex_gap`,
  `ring_outer_locus_gap`) and its segment-window test, and carries only
  the `Indeterminate`. Its ending is `HOLE_INSIDE` alone, the lever
  `RingMeetsOuter` and `RingOutsideOuter` end in too, with no tolerance.
  The gaps pass on either nonzero sign, so a closed decision plus a
  two-sided `SizedPass` (PR 3390's `SizedPass::NonZero`) would let the
  in-band arm quote the tolerance below `|m|/K`.
- **`SliverDihedral { check: WedgeCheck::MaterialSide, .. }`** ends in
  the defect ending. For the cusp side that holds: its magnitude is
  the second-order margin one decision earlier classified positive. The
  pairing (`material_wedge_side`) has a knife-edge window: its margin
  `cosθ·arm` is bounded below only by `ε·√(K²−1)` (the arm gate passes
  `arm > Kε`, the smooth verdict leaves `sinθ·arm ≤ ε`), which is below
  the escalation threshold `Kε`, so sound geometry near both thresholds
  can land it in band and read a defect. For `MaterialArmOutcome::Split`
  it may not hold either: samples that
  disagree about which end of the wedge an edge makes (`material_cusp_side`)
  are what a tangent edge whose curvature difference changes sign
  between samples gives, and moving the geometry reaches that. Decide
  whether the split is a definite refusal with its own lever, and give
  it that ending.
- `ChartRegionError::TouchingBoundary` and `DegenerateLoop` carry no
  margin, so their endings (`classify_chart_region`) name the lever
  alone; `DegenerateLoop` also covers a loop with fewer than three chart
  vertices, which no tolerance decides.

## Repair shape

A closed check type on `RingContactEscalated` ending through a
`SizedDecision`; a ruling on the split's story; a margin on the two
chart-region arms if their valued tighten is wanted.
