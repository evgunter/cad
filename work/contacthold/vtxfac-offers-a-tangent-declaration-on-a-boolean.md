---
id: vtxfac-offers-a-tangent-declaration-on-a-boolean
kind: issue
title: vtxfac's admitted offer list proposes a GlueClass::TANGENT declaration on a boolean, which D10 retires
status: review
opened: 2026-10-12
priority: P3
cost: E
branch: contacthold/claim-free-tangent-glue
pr: 4831
---


## What

The C8 refusal's offer machinery in vtxfac lists GlueClass::TANGENT as a declaration the user could make; D10's Booleans paragraph retires BooleanCoincidence declarations, so the offer names a recourse that does not exist.

Found by a designer of the overlapping-tangency fork (fork log row 118), read from the code at 63288134b8, not measured.

## Answered (PR 4831)

`vtxfac` offers no `Tangent` class: the glue door has already read every
pair the witness lane has a locus for, and leaves one unglued only where
it is no tangency.
