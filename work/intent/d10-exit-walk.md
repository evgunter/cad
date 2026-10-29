---
id: d10-exit-walk
kind: issue
title: D10's exit walk: no code path implements a clause D10 supersedes, and the companion pages state D10's design
status: closed
opened: 2026-10-29
priority: P0
cost: M
closed: 2026-10-29
pr: 5997
---


`work/intent/plan.md`'s exit criteria: D10's last paragraph lists the clauses it supersedes, and the program is done when no code path implements one of them and the companion pages state D10's design.

The walk goes through that list clause by clause, against main. For each clause it either names the code that still implements it, filed as a row on the owning program, or cites what retired it. It does the same for the companion pages DESIGN.md's table lists. It also re-reads the rows parked on `intent-stage3-is-built` (closed 2026-10-29): `work/scalar/the-box-driver-carries-no-part-resolver.md` is closed if its code is gone and opened if its scene still stands.

Done when the walk's report is committed beside this item and every clause still implemented has a row.
