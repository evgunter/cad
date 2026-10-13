---
id: vtxfac-offers-a-tangent-declaration-on-a-boolean
kind: issue
title: vtxfac's admitted offer list proposes a GlueClass::TANGENT declaration on a boolean, which D10 retires
status: closed
opened: 2026-10-12
priority: P3
cost: E
branch: contacthold/claim-free-tangent-glue
pr: 4831
closed: 2026-10-12
---


## What

The C8 refusal's offer machinery in vtxfac lists GlueClass::TANGENT as a declaration the user could make; D10's Booleans paragraph retires BooleanCoincidence declarations, so the offer names a recourse that does not exist.

Found by a designer of the overlapping-tangency fork (fork log row 118), read from the code at 63288134b8, not measured.

## Answered (PR 4831)

`vtxfac` offers no `Tangent` class: no declaration reaches the glue door
under D10. The door reads every pair the witness lane has a locus for,
and leaves one unglued where the lane refutes the tangency, and also
where its reading refuses or escalates: it swallows those, so an in-band
tangency reaches `vtxfac` unglued and refuses `CurvedBooleanUnsupported`
at the C8 guard (filed:
`the-glue-doors-tangency-swallows-its-escalations`).
