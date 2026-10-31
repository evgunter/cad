---
id: check-9-drops-the-one-circle-escalation
kind: issue
title: topo: check 9's two-circle arm discards loop_circle's escalation, so ContainDecision::OneCircle's ending has no reader
status: open
opened: 2026-10-08
---


(CONTACT-10 fix pass, `contact/10-contain-endings`; the rule is D4 ¶1
(i) in `docs/DESIGN.md`.)

## What

`boolean::contain::loop_circle`'s one-circle question
(`bool_face_disc_carrier`) escalates as `ContainError::Escalated
{ decision: Some(ContainDecision::OneCircle), .. }`. CONTACT-10 gives
that decision an ending: sized, "gap between circles", `AnySign`. The
ending has no reader yet.

Its one caller, `validate::ring_outer_meeting`'s arm 4
(`crates/topo/src/validate.rs`, ~:7705, "Arm 4: two whole circles"),
matches `(Ok(Some(o)), Ok(Some(r)))` and lets an `Err` fall through to
arm 5, the edge-pair meeting. So an in-band one-circle reading is never
reported; arm 5 reads the pair edge by edge instead.

## Repair shape

Decide whether an escalated one-circle reading may fall through. If arm
5's edge-pair reading is sound on two arcs of nearly one circle, keep
the fall-through, and say so at the arm. That leaves `OneCircle`'s
ending for a future reader.

Otherwise surface it as `RingOuterVerdict::Escalated`. That reports
pairs which today read through arm 5, so it moves check 9's verdicts and
belongs to check 9's owner. It was not contained in CONTACT-10.
