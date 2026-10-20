---
id: a-blend-of-no-edges-refuses-at-evaluation
kind: issue
title: A blend's empty selection refuses at evaluation though its names are authored data the doors hold
status: closed
opened: 2026-10-17
priority: P3
cost: M
design: true
closed: 2026-10-20
pr: 5436
branch: wire/empty-blend-at-the-door
---


## Finding

Found by the sweep of WIRE's `wire/selection-kind-at-the-door` unit,
whose rows moved a selection's per-name kind from evaluation to the
edit and load doors on the ground that it is authored data. The sweep
looked for other facts evaluation re-asks that the doors could hold.

`crates/editor-core/src/eval/wire.rs`, `wire_blend`, refuses
`NodeErrorKind::BlendSelectionEmpty` when `selection.ents` is empty.
How many names a selection holds is as much authored data as their
kind: `Select::fault` (`crates/editor-core/src/var.rs`) already holds a
singleton to one name at both doors, and could hold an `Edges` set read
by a blend to at least one. The seat knows it reads a blend's edges
(`OperandSlot::Selection`).

Not decided here, because there is a reason the other way:
`NodeErrorKind::BlendSelectionEmpty`'s doc calls an empty blend *"an
unfinished recipe"*, and a document may be meant to hold one while it
is being authored (the viewer's `BlendError::NoEdges` pre-check, noted
on `work/vdoc/sweep-blind-spots-the-precheck-sweep-could-not-see.md`,
is the other half of that trade). A shell's empty set is the sealed
hollow, not a refusal, so the rule would be the blend seat's alone.

## What is owed

A ruling: either the doors refuse an empty blend selection (the
`Select::fault` arm takes the seat, or the edit door's seat check does)
and `BlendSelectionEmpty` goes, or the evaluation refusal stays and its
doc says why a door does not hold it.
