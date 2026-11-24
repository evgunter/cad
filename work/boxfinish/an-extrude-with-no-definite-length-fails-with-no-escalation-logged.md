---
id: an-extrude-with-no-definite-length-fails-with-no-escalation-logged
kind: issue
title: An extrude whose length across the sketch plane is undecided over a box fails with no escalation on any log: 27,667 leaves the drive cannot tell from a definite failure
status: open
opened: 2026-11-24
priority: P1
cost: M
---


Found by M13 (the build-record measurement, 2026-11-24;
`probe/M13-REPORT.md` on `boxfinish/probe-build-record` at
`447298e0b`), and named as a source defect by all three designers
weighing
`a-parted-path-with-no-turn-is-reported-as-an-empty-flip-crossing`.

Over 38,119 leaf replays of the suite's drives, 27,781 leaves hold a
node that fails where the witness builds with no escalation on any
log. 27,667 of them are one sentence: an `Extrude` failing with "no
definite length across the sketch plane". 13,818 of them split today,
and 13 such boxes hold certified mass below them (2.02 summed), so
the failure is an undecided sign, not a definite one.

The sign is decided outside the decision funnel: nothing is logged, so
the drive cannot tell this leaf from one that fails definitely
(`Unsettled::Failed`, bisected to the budget), the K instrument never
sees the decision, and no refusal names a deciding predicate.

**The fix is at the source:** the extrude's length sign goes through
the funnel, so that it escalates where it straddles (and the drive
bisects on a named predicate) and reads definite where it does not.
No change to the leaf protocol is needed for it.

**Not measured:** which function forms the sentence and why it does not
call `decide`; whether other doors refuse on an undecided sign the same
way (the other 114 such leaves). First step: that sweep.
(BOXFINISH orchestrator)
