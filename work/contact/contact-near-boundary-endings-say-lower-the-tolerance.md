---
id: contact-near-boundary-endings-say-lower-the-tolerance
kind: issue
title: topo: the census's WitnessTooClose and contfp's grazed-parity refusal say 'lower the tolerance' where D4 ¶1 now says 'if this size is intended, tighten below m/K'
status: open
opened: 2026-09-28
---

(ENCL implementer, from the §5 sweep of the offset-meters D4 ¶1 reshape,
`work/encl/offset-meters-follow-the-d4-recourse-ruling.md`.)

The D4 ¶1 ruling (Ev, `[ev]` PR 3352) derives a refusal's ending from
its decision and verdict. *Tighten the tolerance* is offered only on a
band-decided arm (in band, or Zero where Zero does not pass) of a
decision that passes on a nonzero sign, phrased conditionally and
valued: "if this size is intended, tighten the tolerance below m/K".
It is never offered on a sign-certain arm, nor on a residual. The
decision is a closed type at its site, so its recourse is an
exhaustive match, never a lookup by predicate name.
`geom_brep::recourse::SizedDecision` is that table for a sized decision
(lever, size noun, pass set, stored-definite reading); certification
(`geom_brep::certify::recourse`) and the offset meters
(`geom_brep::offset_meters::Meter`) both end through it.

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
