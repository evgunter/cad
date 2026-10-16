---
id: verify-declared-contacts-bridges-an-exact-zero-for-auto-glued-pairs
kind: issue
title: verify_declared_contacts runs the tangent table with declared=true for auto-glued pairs, bridging an exact zero with no declaration behind it
status: closed
opened: 2026-10-12
priority: P3
cost: E
branch: contacthold/claim-free-tangent-glue
pr: 4831
closed: 2026-10-12
---


## What

The verify path passes declared = true for pairs the boolean glued itself, so an exact-zero kappa_rel is bridged as if declared. Possibly intended under D10, but the comment says otherwise; make the code and the comment agree.

Found by a designer of the overlapping-tangency fork (fork log row 118), read from the code at 63288134b8, not measured.

## Answered (PR 4831)

The door's tangency check (`verify_tangency` → `contact_verify::tangency_reading`)
reads with `declared = false`: a glued tangency is jet-determinate, and no
exact zero is bridged at the door. The re-check in
`verify_declared_contacts` is the same call. An exact zero met later, at
the lump, is read by the pair's sense bit (`sectors::tangent_lump`), not
by a declaration.
