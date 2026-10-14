---
id: the-mate-tool-across-two-gauges-commits-a-declaring-mate
kind: issue
title: the viewer's mate tool has no copy-gauge-then-mate: across two gauges it commits a plain insert that declares
status: open
opened: 2026-10-03
priority: P3
cost: M
---


Found by RECIPE's `a-partly-applied-regauge-then-mate-list-declares-silently`
sweep (the `regauge_then_mate` door), 2026-10-03.

## What

The kernel has two ways to insert a mate: the plain insert
(`DocEdit::InsertNode`, `crates/editor-core/src/edit.rs`), which
DECLARES when the two instances stand on different gauges, and the
compound door `regauge_then_mate` (same file), which puts the first
operand's group on the second's gauge and then inserts, so the mate
PLACES. Python has both (`Doc.insert`, `Doc.regauge_then_mate`).

The viewer has only the first. `MateProposal::op`
(`crates/viewer/src/matetool.rs`) answers `SessionOp::AddMate`, and
the session commits that as a bare `DocEdit::InsertNode`
(`crates/viewer/src/session.rs`, the `SessionOp::AddMate` arm of
`perform`). Neither the tool's proposal nor the commit reads the two
instances' gauges. So in a document that has gauges (a loaded one;
the viewer authors no `Node::Gauge` itself), a mate picked between
two instances on different gauges lands as a declaring mate, with no
refusal, no offer of the compound action, and no maintenance row. The
person sees the difference only later, if the declared contact fails
at the at-rest gate.

## Shape of a fix

The tool reads both members' gauges at proposal time. Where they
differ, it either offers the compound action (a `SessionOp` that
commits through `regauge_then_mate`, which also refuses
`WouldStartPlacing` typed) or says that the mate will declare, before
the person commits. Which one to do is a design call for whoever
owns the viewer's assembly authoring.

The door answers a `RegaugeThenMateOutcome` (the document, the edits it
applied in order, the maintenance, the mate's id), so such an op
commits `edits` as one history group, as `record_run` does a staged run.
