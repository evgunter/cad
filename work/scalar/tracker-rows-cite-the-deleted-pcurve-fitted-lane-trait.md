---
id: tracker-rows-cite-the-deleted-pcurve-fitted-lane-trait
kind: issue
title: Open tracker rows on other slates still cite PcurveFittedLane, which LANE-4 deleted
status: review
branch: scalar/hygiene
pr: 3449
opened: 2026-09-25
priority: P4
cost: E
---


## What

LANE-4 deleted `geom_brep::PcurveFittedLane`; its door is now the value
`geom_brep::FittedLane<T>`, answered by `topo::AtRestPolicy::fitted_lane`,
and every bound that named the trait is `AtRestPolicy` or `Decide`.
Open rows on other programs' slates still cite the trait in their
evidence prose, and read as current:

- TOPO `work/topo/half-edge-minting-euler-ops-leave-a-minted-curved-face-incomplete.md`
  — its options are argued as a `Decide → PcurveFittedLane` ripple on
  the minting operators; the ripple is now `Decide → AtRestPolicy`.
- TOPO `work/topo/split-edge-children-lack-pcurve-rows-on-curved-charts.md`
  — "the lane bound `mint_pcurves_of` … `PcurveFittedLane` requires".
- PCERT `work/pcert/pcurve-fit-refusal-drops-the-domain-doors-reason.md`
  — cites `crate::PcurveFittedLane::general_image` (now
  `FittedLane::general_image`).
- REACH `work/reach/graft-recertifies-through-the-narrow-lane.md` —
  cites `verbs::Verb`'s `impl<T: Decide + Bounds + PcurveFittedLane>`
  (the term is gone; `verbs` no longer depends on geom-brep).
- CLEAR `work/clear/symbolic-tier-and-clearance-engine.md` — lists the
  trait among the lane traits.
- BAND `work/band/S90-impl.md` — `fillet_edges` "with `+
  PcurveFittedLane`".
- SHELL `work/shell/plain-transform-rigid-still-refuses-the-m7-8-class.md`
  — the same `verbs::Verb` bound.
- ENCL `work/encl/approx-surface-tolerance-is-now-always-the-runs-eps.md`
  and PROPS `work/props/real-rs-seat9-paragraph-describes-a-second-impl-block-that-is-gone.md`
  — historical mentions (`remap_certificate`, a quoted old `impl`
  header).

LANE-4 left them alone (its deviation 4), because each is its owner's
evidence text and TOPO's row had a live PR on it. Raised by LANE-4's R1
(S8). Found by `git grep PcurveFittedLane -- work/`; the TQUERY row
`split-edge-cannot-carry-a-fitted-or-general-pcurve-row` was updated in
LANE-4 itself.

## Proposed

Each owner, next time it touches its row, re-states the bound as
`AtRestPolicy` (or drops it where the term is simply gone) and says
so. A historical quotation may stay as quotation.

## Cost

E: prose in nine files, owned by eight programs.
