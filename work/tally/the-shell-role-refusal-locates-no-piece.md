---
id: the-shell-role-refusal-locates-no-piece
kind: issue
title: The door's in-band shell refusal names its shell by keys into a body it never returns, so the move-the-parts lever cannot say where the thin piece is
status: open
opened: 2026-10-09
priority: P3
cost: M
---


## What

Found by PR 4415's second review (MINOR-1 of its table, and its Q5).

`BooleanError::Escalated { decision: BooleanDecision::ShellRole {
solid, shell, others }, .. }` names the shell whose certified `V/A`
binds the offer. The keys point into the refused result, and the door
never returns that body (`BooleanError::ResultInvalid`'s doc: "no body
below it is ever returned"). So they are diagnostic only, and the field
docs say so.

The offer needs no location: a tolerance below the binding shell's
|V/A|/K decides every in-band shell. The other recourse is "move the
parts so the pieces and cavities they leave are clearly thick", and it
cannot say which piece. With several slivers, it cannot say which ones.

## The shape to give

Carry something a caller can use. The binding shell's bounding box is
cheap at the gate (`boolean::boxes`), or there could be a point on it.
Do it once for all in-band shells, if the refusal's shape allows.
`BooleanDecision` is `Copy`, which rules out a list in the decision.
