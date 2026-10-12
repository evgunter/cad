---
id: verify-declared-contacts-bridges-an-exact-zero-for-auto-glued-pairs
kind: issue
title: verify_declared_contacts runs the tangent table with declared=true for auto-glued pairs, bridging an exact zero with no declaration behind it
status: open
opened: 2026-10-12
priority: P3
cost: E
---


## What

The verify path passes declared = true for pairs the boolean glued itself, so an exact-zero kappa_rel is bridged as if declared. Possibly intended under D10, but the comment says otherwise; make the code and the comment agree.

Found by a designer of the overlapping-tangency fork (fork log row 118), read from the code at 63288134b8, not measured.
