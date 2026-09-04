---
id: VERBS-CONE
kind: issue
title: cone and torus operand lanes
status: open
opened: 2026-08-21
refs: [1604, VERBS-C5ARMS]
---

Wave 2 row 10: cone (and torus) operand lanes for the boolean, sequenced on
what rows 6–9 learn. Gates Klein wall 3 (measured at #1001). Known traps named
in the plan: #226 residual 1 (conic-trimmed cylinder walls slip both sense
gates) and #685 (cone-wedge grid sizing drops `nv` at `nu == 1`, mesh side).
No spec; queued behind CYLSPH and the C5ARMS remainder.

**VERBS closed** (exit walk ratified, PR #1793); re-homed to
`work/issues/` awaiting an owner.

**Remaining scope at re-home:** the cone and torus OPERAND lanes —
never cut as a spec; the C5-section half was executed by TORAX +
C5ARMS PR-1. A candidate seed for a successor program.
