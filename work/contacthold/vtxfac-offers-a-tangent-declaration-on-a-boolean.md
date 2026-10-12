---
id: vtxfac-offers-a-tangent-declaration-on-a-boolean
kind: issue
title: vtxfac's admitted offer list proposes a GlueClass::TANGENT declaration on a boolean, which D10 retires
status: open
opened: 2026-10-12
priority: P3
cost: E
---


## What

The C8 refusal's offer machinery in vtxfac lists GlueClass::TANGENT as a declaration the user could make; D10's Booleans paragraph retires BooleanCoincidence declarations, so the offer names a recourse that does not exist.

Found by a designer of the overlapping-tangency fork (fork log row 118), read from the code at 63288134b8, not measured.
