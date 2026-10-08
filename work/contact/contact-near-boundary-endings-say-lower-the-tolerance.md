---
id: contact-near-boundary-endings-say-lower-the-tolerance
kind: issue
title: topo: the census's WitnessTooClose and contfp's grazed-parity refusal say 'lower the tolerance' where D4 ¶1 now says 'if this size is intended, tighten below m/K'
status: dispatched
opened: 2026-09-28
priority: P3
cost: E
parent: CONTACT-10
---

(ENCL implementer, from the §5 sweep of PR 3382.) The rule is D4 ¶1 (i)
in `docs/DESIGN.md`; `geom_brep::recourse::SizedDecision` is the shared
ending table for a sized decision.

## What

Two contact-ground refusals still end in the pre-ruling phrasing:

- `crates/topo/src/census.rs`, the `WitnessTooClose` arm (~:2983):
  "Recourse: move the parts until their bounding boxes no longer
  overlap, or lower the tolerance". Unconditional and unvalued.
- `crates/topo/src/boolean/contain.rs` (~:107), the contfp
  grazed-parity refusal: "move the point off the boundary or lower the
  tolerance". Unconditional, unvalued, and unlabelled (no
  `Recourse:` marker).

## Repair shape

Name each site's decision and its pass set. Where it passes on a
nonzero sign and the margin is carried, end the band-decided arm in
the lever plus the conditional, valued tighten; otherwise the lever
alone. `SizedDecision::recourse` can do this directly if the margin
and band reach the variant.
