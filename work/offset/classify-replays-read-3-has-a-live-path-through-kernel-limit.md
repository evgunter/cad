---
id: classify-replays-read-3-has-a-live-path-through-kernel-limit
kind: issue
title: classify_replay's read (3) has a live path: limb 2's KernelLimit reaches its `_ => Bisect` arm
status: closed
opened: 2026-10-26
closed: 2026-10-27
---


`classify_replay`'s comment (`crates/editor-core/src/drive.rs:1726`)
says its read (3), the error-enum arms, has no live path today: every
escalation is minted inside `eval_node`'s bracket, so read (2) answers
first. PR 5728's `AnalyticRung3Refusal::KernelLimit`
(`crates/geom-brep/src/edge_nurbs.rs`) is a path that read (2) does not
answer. A stalled limb-2 bound is decided definite past the band, so
nothing lands on the node's escalation log, and the node's failure
falls to read (3)'s catch-all `_ => return LeafVerdict::Bisect`
(`crates/editor-core/src/drive.rs:1806`). At a parameter-family scalar
this is the bisection that fork row 204 rules (a stalled bound over a
box is undecided at that leaf). It is reached by the catch-all,
though, not by a typed read.

The arms' deletion question
(`work/verdict/should-classify-replays-error-enum-arms-be-deleted.md`)
now has a live case to measure against: deleting the catch-all would
change what these leaves do. When fork 203's mechanism lands, the stall
should reach the drive as an undecided margin, read (2)'s path, and the
comment should be corrected either way.

**Closed by fork 203's mechanism (PR 5728).** At an enclosure scalar,
`Interval` and the drive's `Sym<Interval>`, limb 2 no longer refuses as
`KernelLimit`: it decides the composite's enclosure `[floor, upper]` as
one margin (`edge_nurbs::limb2_enclosure`), and a stall is that
decision's `Indeterminate`, minted by the funnel, so it lands on the
node's escalation log and read (2) answers it. That read bisects
unless the classifier's terminal bit is set, and for a bare upper bound
the bit is false. `KernelLimit` is left at point scalars only (`f64`,
`Probe`), which the drive does not replay at. So read (3)'s comment at
`crates/editor-core/src/drive.rs:1726` ("no live path to them today")
is true again and stays as written. The deletion question
(`work/verdict/should-classify-replays-error-enum-arms-be-deleted.md`)
loses this live case.
