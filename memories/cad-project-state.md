---
name: cad-project-state
description: Greenfield Rust CAD kernel — DESIGN.md is the authoritative contract; M0–M8 COMPLETE (#89 CLOSED / K=10 permanent); M9 = C7 join lane (OPEN next), M10 = error propagation; the LIB and ASM programs run concurrently; LIVE STATUS = the M*/LIB/ASM-LOG tails, never this memory; merge gate = hosted Actions; name pending (Q9)
metadata:
  node_type: memory
  type: project
  originSessionId: 11974b46-1641-48d9-9802-fdf44dcb6927
---

**Rule this memory exists to state: live status is the relevant
`docs/*-LOG.md` tail, NOT this file** — this file only pins the
completed floor and the standing facts that do not churn.

As of 2026-08-15: **M0–M8 COMPLETE.** M5 closed at 35 PRs (#169);
M6 closed 2026-08-08 (#243); M7 closed 2026-08-09 (#300); M8 closed
2026-08-15 (#508). Each has an exit walk as its done-state of
record (`docs/M*-EXIT-WALK.md`).

**Three concurrent programs**, each with its own plan and log:

- **Kernel milestones — next is M9 = C7**, the declared-contact
  join lane plus ASSEMBLY-DESIGN A5's at-rest census door (+ the
  lily FULL rebuild); **M10 = error propagation** (ERROR-DESIGN).
  This numbering was fixed by Evan's 👍 on PR #300 (2026-08-09).
  M8's carried items at the seam: the ratified A′ PlacedUnion
  design (docs/GROUP-BOOLEAN-DESIGN.md, unit unscheduled), D1 (the
  revolve name emitter), and the banked follow-up issues listed in
  the M8 walk's row 12.
- **LIB — usable-as-a-library** (`docs/LIB-LOG.md`), ratified as
  LIBRARY-DESIGN #229. The `pncad` façade, the PATHS algebra, the
  profiles-as-programs switch and the Python bindings all shipped
  here; `docs/GUIDE.md` is the user-facing surface.
- **ASM — assemblies** (`docs/ASM-PLAN.md` / `ASM-LOG.md`), ratified
  as ASSEMBLY-DESIGN #333 (A1–A11). R1 is nearly closed.

#89 CLOSED: K=10 permanent ratified default (docs/K-REPORT.md incl.
the M7 landing-retraction addendum). Merge gate = hosted Actions
(nextest build-once/sharded matrix since #167), ci-local.sh mirror.
References live in the MAIN checkout. Name still pending (Q9);
`pncad` is the greppable placeholder.
