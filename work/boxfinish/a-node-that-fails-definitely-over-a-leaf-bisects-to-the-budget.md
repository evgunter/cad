---
id: a-node-that-fails-definitely-over-a-leaf-bisects-to-the-budget
kind: issue
title: A node that fails definitely over a leaf is bisected until the budget: the drive has no class for a leaf that is definitely invalid
status: parked
opened: 2026-11-24
priority: P2
cost: M
blocked_on: [a-parted-path-with-no-turn-is-reported-as-an-empty-flip-crossing]
---


Found by the designers weighing
`a-parted-path-with-no-turn-is-reported-as-an-empty-flip-crossing`
(2026-11-24), by reading.

`classify_replay`'s first step sends every typed node failure to
bisect (`Unsettled::Failed`), including a check that refuses
definitely over the whole leaf. No narrower box changes a definite
refusal, so such a leaf costs depth-many replays and ends as `Budget`
mass that means "not proven" where the drive had a proof. E6 has no
class for "definitely invalid over the leaf" (`Infeasible` is the
solver's). Under the parted-path weighing's final state it is the
last place a decided leaf is treated as undecided: a node that failed
where the witness built is another build, proven. What the class is,
and whether it is `FlipCrossing` or its own, is a design question
weighed with that unit. (BOXFINISH orchestrator)

**Measured by M13 (2026-11-24):** over 38,119 leaf replays, 27,781
leaves hold a node that fails where the witness builds with no
escalation on any log; 27,667 of them are one sentence, an `Extrude`
with "no definite length across the sketch plane". 13,818 of them
split today, and 13 such boxes hold certified mass below them (2.02
summed over the suite's drives): so the failure is not proven definite
over the leaf, and no log today carries one that is. Whatever class a
definitely failing leaf gets needs that proof first. (BOXFINISH
orchestrator)

**Corrected by M13b (2026-11-24, `probe/M13b-REPORT.md` section 7 on
`boxfinish/probe-build-record` at `efceb821b`): the paragraph above
drew the wrong conclusion.** All 27,667 `Extrude` leaves are
`ExtrudeError::DegenerateExtrusion`: `extrusion_normal_component`
decided `Zero` in the funnel, on a depth wholly inside the zero band
(a representative: depth in [-8.6e-10, -6.6e-10] against a band of
1e-9; every such leaf is at most 7.8e-10 wide). **None has certified
mass below it.** That is a failure proven definite over the leaf: the
log does carry the proof (a decided `Zero` and no escalation), and the
drive bisects it to the budget all the same, 13,908 leaves ending
`Budget` and 13,759 splitting. So this row is real and measured, on
one drive family (a slab whose nominal depth, 3e-8, sits near the
band). The 13 boxes with certified mass below them are other
failures, the ones a narrower box does remove: 9 a profile lane's
replay with no structure (`m10_3`'s undecided-axis row), 4 an
extrude's rung-3 carrier enclosure (the `notch` drives).
`an-extrude-with-no-definite-length-fails-with-no-escalation-logged`,
filed on the first reading, is closed into this row. (BOXFINISH
orchestrator)

**Weighed with the parted-path fork (2026-11-24): a node that fails on
a decided answer where the witness builds is a `FlipCrossing` naming
that decision, refused at once.** A turned definite sign holds on
every sub-box, so no split helps. `classify_replay`'s failure arm has
to read the failed node's log to see it. That is one of the clauses
put to Ev on PR 7700 and is built with that fork's unit, on which this
row now waits. A node that fails definitely with NO decision turned
against the witness has no instance yet. (BOXFINISH orchestrator)

