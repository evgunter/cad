---
id: checks-escalated-evidence-says-lower-the-tolerance
kind: issue
title: editor-core: the checks window's Escalated evidence says 'lower the tolerance' with no decision behind it
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

`crates/editor-core/src/checks.rs`, the finding recourse for
`CheckEvidence::Escalated` (~:647): "Recourse: thicken or remove the
degenerate geometry, or lower the tolerance". It names no decision,
and its tolerance arm is unconditional and unvalued.

## Repair shape

Route the escalated evidence to the decision that escalated. The
evidence carries its source refusal, whose own ending (certify's,
the meters', validate's) is derived from the decision. Reuse that
ending rather than a window-wide sentence. Re-pin the checks window's
text rows.
