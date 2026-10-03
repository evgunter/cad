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


## Measured: a box run over a document whose mate escalates (MSOLVE-11, PR 3680)

MSOLVE-11 puts the solve's escalations on the deciding mate's own log.
So `classify_replay` would now meet an escalating mate at read (2), the
log, where before it fell to read (3)'s `_ => Bisect`. That move is
latent: no box run reaches it today.

Measured on a pair with two planar rests, one tilted so that its
levered sine lands in the band, and one parameter boxed by
`range::derive`:

- `drive` returns `DriveRefusal::WitnessDoesNotBuild`. Its cause is the
  mate's `Unleverable`: "no part resolver was given".
- The driver's evaluations (`drive::lane_opts`) carry no resolver. At
  the nominal witness every mated part is out of hand, and the fold
  refuses the lever before it decides anything. No leaf is ever
  classified.
- The same document under a resolver escalates. The added mate refuses
  `MateFault::Indeterminate`, with the escalation on its own log.

Pinned by
`msolve11_mate_log::a_box_run_over_an_escalating_mate_refuses_at_its_witness`.

What this means here: before this item's `Budget` shape can show, a box
run over any assembly needs the driver to carry a resolver. Once it
does, an escalating mate reaches read (2), not the catch-all. It
bisects, and it reads `SliverTerminal` when the margin sits wholly in
the band. The box-independent faults this item names would still fall
to read (3).


## Re-measured (MSOLVE-14, PR 3986)

The solve now runs at the evaluation's scalar over its lane
environment, so an undecided mate predicate in a box run escalates on
the deciding mate's log, and a checked offset's on its placing mate's.
The driver still carries no resolver (`drive::lane_opts`), so this
row's `Budget` shape is still unreachable:
`msolve11_mate_log::a_box_run_over_an_escalating_mate_refuses_at_its_witness`
holds unchanged (`WitnessDoesNotBuild`, the mate `Unleverable` for want
of a resolver). The two analysis doors that evaluate an assembly
directly now take one (`stackup::sensitivities_resolved`,
`ClearanceQuery::resolver`); the driver is the remaining door, and
closing this row waits on it.
