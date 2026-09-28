---
id: coincidence-recourse-says-lower-where-d4-says-tighten
kind: issue
title: geom-core: COINCIDENCE_RECOURSE's third arm says 'lower the tolerance' where D4 ¶1 names it 'tighten the tolerance', conditional and valued
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

`geom_core::COINCIDENCE_RECOURSE` (`crates/geom-core/src/predicate.rs`
~:1002) is "declare the coincidence, move the geometry, or lower the
tolerance". D4 ¶1 (i) names the three-arm sentence "declare the
coincidence / move the geometry / tighten the tolerance". It offers
tightening only on a band-decided arm of a decision that passes on a
nonzero sign, conditionally and valued. The constant is composed by
`Indeterminate`'s own `Display` and by every coincidence site, so its
third arm is unconditional and unvalued wherever it renders.

Related: `work/chrome/escalations-forwarded-whole-are-untriaged-for-a-declarations-object.md`
(which forwarding sites should render it at all).

## Repair shape

This touches every coincidence site's text, so it needs its own
decision on scope: whether the constant keeps a third arm at all, or
whether each coincidence decision composes its own valued conditional
from its margin (`Indeterminate` carries the margin and band).
