---
id: ssi-transversality-death-says-lower-the-tolerance
kind: issue
title: ssi: the transversality-death refusal says 'lower the tolerance' unconditionally, unvalued
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

`crates/geom-brep/src/ssi.rs` (~:437), the transversality-death
refusal's `Display`: "...separate the operands, declare the tangency,
or lower the tolerance". It carries `sin_theta` and `arm`, so the
margin that would value a conditional tighten is in the payload. It is
unconditional and unvalued, and unlabelled (no `Recourse:` marker).

## Repair shape

Decide whether the death is a band-decided arm of the transversality
decision (`geom_brep::certify::CertCheck::Transversality` is the same
question on an edge). If it is, end it through that decision's
`SizedDecision`, with "declare the tangency" kept only where the door
takes a declaration. If it is not, name the lever alone.
