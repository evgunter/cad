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

## Measured (FLUSH `flush/shell-role-reader`, 2026-10-14)

The door's `Escalated { ShellRole }` now quotes an aimed ray's pair in
band (check 10's `ShellClassifyError::RayInBand` or check 7's
`OrientationUnread::Pair`), its margin the gap along the ray. The
decision still names its solid and shell, keys into the unreturned
result; the pair beside it (`AimedPair`: the aimed face and the two
faces met) is keys into the same body, so it locates nothing a caller
holds either. Still open.

## Measured (FLUSH PR 5119's fix pass, 2026-10-14)

Narrowed, not closed. The door's `Escalated { ShellRole }` now carries the
pair's two faces (`near`, `far`) and names them in its sentence, and no
longer the solid or shell, which located nothing either; the aimed point
rides in the at-rest refusal (`AimedPair::aim`, coordinates). A new
decision, `Escalated { SolidSide }`, refuses a result solid whose volume
lies in band where no aimed ray read its side, and names the solid and how
many rays were aimed and set aside. Both are still keys into a result the
door never returns, so a caller still cannot find the piece; the aim
point is the one coordinate the refusals now carry, and only at rest.
