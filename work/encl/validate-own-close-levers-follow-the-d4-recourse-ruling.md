---
id: validate-own-close-levers-follow-the-d4-recourse-ruling
kind: issue
title: topo::validate's own-close escalations (SliverDihedral, the planar residuals) say 'lower the tolerance' where D4 now says 'tighten, if this size is intended', or nothing
status: open
opened: 2026-09-28
---


(ENCL implementer, from the fix pass on PR 3351.)

## What

The D4 ¶1 ruling (`[ev]` PR 3352) derives a refusal's ending from its
decision and verdict: *tighten the tolerance* only on a band-decided
arm of a decision that passes on a nonzero sign, phrased
conditionally with the value the margin gives ("if this size is
intended, tighten the tolerance below m/K"); never on a residual,
whose refused margin is a miss. PR 3351 moved certification onto that
rule and left `topo::validate`'s own-close endings, which still say
"lower the tolerance" unconditionally and with no value
(`crates/topo/src/validate.rs`):

- `EDGE_CLOSE` (~:2133, "move the geometry so the faces meet at a
  clearer angle, or lower the tolerance"), which
  `ValidationError::SliverDihedral`'s `Display` ends in through
  `own_close` (~:2787). The dihedral wedge passes on Positive, so the
  ruling's ending is the lever and the conditional tighten below
  `m/K`, the same one `geom_brep::certify::recourse` gives
  `CertCheck::Transversality`.
- `own_close(&cause.margin, "Recourse: lower the tolerance")` at
  `PlanarFaceEscalated` (~:2771) and `PlanarBoundaryEscalated`
  (~:2781), and in `classify_props`'s `P::Escalated` (~:2456),
  `classify_pcurve_mint`'s `M::Escalated` (~:2497) and its
  `C::FittedEscalated | C::Escalated` (~:2518). The planar ones are
  residuals (a corner or an edge on its face's plane): they pass on
  Zero, so no tolerance is the recourse, and the at-rest ending is the
  kernel-or-file defect or, where the kernel approximated, the last
  resort. The props and pcurve ones need their decision named first.

## Repair shape

Route each through its decision's closed type the way `certify`'s
`recourse` does (a `passes()` set and an exhaustive ending match),
or call `geom_brep::certify::recourse` where the decision is a
`CertCheck`; re-pin `validate`'s rendered-text rows.
