---
id: the-dual-review-streams-fourth-readout-is-owed
kind: ruling
title: The dual review stream's full readout is owed: twenty M-tier units at DR-144
status: closed
opened: 2026-10-10
closed: 2026-10-10
pr: 4555
---

`docs/DUAL-REVIEW-PROTOCOL.md` rule 9 owes a **full readout** when twenty M-tier units have been recorded under rule 1's arms. DR-144 is the twentieth: NURBS, PR #4518, sequential arm. It was recorded on that PR's branch at `3122531ba2`, where it was renumbered from DR-143 after GERM's DR-143 landed first.

- A separate lane writes the readout blind, on `analysis/dual-review/readout-4`. The orchestrator does not read it; the readouts are off-file (rule 10).
- The orchestrator opens the `[ev]` PR from that branch. The PR is not for main.
- The arms continue unchanged until Ev rules.

## Closed

Ev ruled on PR 4555, 2026-10-10: "the current pattern can continue, iirc 20 more majors or 2 more unilateral majors where the other side didn't catch the issue at all". The ruling is recorded in `docs/DUAL-REVIEW-LOG.md`'s rulings list, and the count toward the next full readout restarts after DR-144. PR 4555 is closed unmerged; readouts are off-file.
