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

**What is not known is where the escalation goes** (read 2026-11-24,
after filing). `sweep::extrude`'s direction classification does read
its sign through the funnel: `extrusion_normal_component` by
`swept::decide_reported` (a distance) and `decide` (a vector). Two
failing arms print the one sentence: `ExtrudeError::DegenerateExtrusion`
(a decided `Zero`) and `ExtrudeError::ExtrusionEscalated` (the funnel's
`Indeterminate`). So one of two things is true, and the measurement
lane is asked which (M13b):

- the leaves are `ExtrusionEscalated` and the escalation is lost
  between the funnel and the node's log (a node that returns `Err`
  dropping its frame's log): then the defect is the drive's reading of
  a failed node, and every door's undecided failure has it;
- the leaves are `DegenerateExtrusion`, a decided `Zero` over a box
  that holds certified mass below it: then `Zero` is decided on an
  enclosure that is not zero across the box.

Either way the drive cannot tell these leaves from ones that fail
definitely (`Unsettled::Failed`, bisected to the budget), and no
refusal names a deciding predicate. The row as first filed said the
sign is decided outside the funnel; that was an inference from the
missing escalation and the code does not bear it out.

**Not measured:** the other 114 such leaves (`Profile` "not simple"
95, `ProfileLaneReplay` 10, `Skin` 5, `Extrude`'s rung-3 certificate
4), which may be the same loss.
(BOXFINISH orchestrator)
