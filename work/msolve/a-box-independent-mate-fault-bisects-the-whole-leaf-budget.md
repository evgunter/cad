---
id: a-box-independent-mate-fault-bisects-the-whole-leaf-budget
kind: issue
title: A mate fault no box can move falls to the driver's catch-all Bisect, so a box run splits through its leaf budget and reports Budget
status: open
opened: 2026-10-01
priority: P2
cost: E
---

Found by a designer weighing plan item 19 (the analysis lanes). The
orchestrator checked it against the code; it has not been probed.

## What

The box driver reads a failed leaf in three steps
(`crates/editor-core/src/drive.rs`, `classify_replay`'s leaf read):
first the box-independent measure classes, then the escalation log,
then the error-enum arms. A mate fault is not in
`box_independent_measure_class`. In the enum match it falls to
`_ => return LeafVerdict::Bisect`.

Today the solve runs once, at the nominal. So every sub-box re-solves
the same problem and meets the same fault. Examples:
- a face-framed mate refusing `Unpinned` under a certifying clearance;
- a dangling head;
- an unresolved part.

The driver splits until the leaf budget is spent and then reports
`Budget`. That is the exact shape `RefusalReason::MeasureRefused`'s
doc records closing for measures (M10-6: "a measured >60 s on a
`NoAdmittedPair` fixture", "priced the mass as `Budget`, which named
the symptom").

## What would close it

Treat the mate faults that no box can change as terminal at read
(1), under their own class. That covers the walk, the class, a
dangling head, a count and an unresolved part. If plan item 19 makes
the solve run at the box's scalar, an undecided mate predicate becomes
an escalation on the mate's log (MSOLVE-11). That escalation splits
usefully and stays `Bisect`. The two land together or this first.

A red probe first: a box run over a face-framed mate under certifying
clearance, showing `Budget` where the fault is the mate's.

