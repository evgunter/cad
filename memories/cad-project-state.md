---
name: cad-project-state
description: Greenfield Rust CAD kernel — DESIGN.md is the authoritative contract; M0–M6 COMPLETE (M6 close statement in docs/M6-LOG.md; #89 CLOSED / K=10 permanent), M7 (STEP adoption ONLY) in flight with units 1/2/4 merged; M8 = error propagation next; LIVE STATUS = the highest-numbered docs/M*-LOG.md tail, never this memory; merge gate = hosted Actions; name pending (Q9)
metadata:
  node_type: memory
  type: project
  originSessionId: 11974b46-1641-48d9-9802-fdf44dcb6927
---

**Rule this memory exists to state: live milestone status is the
highest-numbered `docs/M*-LOG.md` tail, NOT this file** — this file
only pins the completed-milestone floor and the standing facts that
do not churn.

As of 2026-08-05: **M0–M6 COMPLETE.** M5 closed at 35 PRs (#169 =
the exit walk; docs/M5-EXIT-WALK.md is the done-state of record).
M6 closed with its four executed units merged (#171 surgery, #176
SSI lift, #192 loft/sweep assembly, #178 CONTACT-DESIGN ratified)
and three items explicitly re-banked — see M6-LOG's close
statement. **M7 (STEP adoption ONLY) is in flight**: units 1/2/4
merged (#183 own-corpus round-trip, #189 FreeCAD foreign corpus,
#193 wild corpus); import is LIVE. M8 = error propagation
(ERROR-DESIGN's body says "M6" = historical pre-renumbering). #89
CLOSED: K=10 permanent ratified default (docs/K-REPORT.md incl. the
M7 landing-retraction addendum). Merge gate = hosted Actions
(nextest build-once/sharded matrix since #167), ci-local.sh mirror.
References live in the MAIN checkout. Name still pending (Q9).
