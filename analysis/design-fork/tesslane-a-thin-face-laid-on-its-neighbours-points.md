# Design fork: a thin face laid on its neighbour's points (TESSLANE, PR 7573)

- Protocol: `docs/DESIGN-FORK-PROTOCOL.md` at `ddd201f411f`.
- Dispatched 2026-11-23 (time by `date -u` in the commit).
- Bytes (`od -An -N2 -tu1 /dev/urandom`): **82, 225**.
  - Arm byte 82 (< 86): THREE designers.
  - Label byte 225: 225 mod 3 = 0, so the Fable designer is **A**; the two Opus designers are **B** and **C**.
- Designers: local background subagents of the orchestrator's session, read-only, on a checkout of PR 7573's frozen head `24ee9a28fb`; one identical problem statement (`tesslane-a-thin-face-laid-on-its-neighbours-points-problem.md`), no candidate solutions.
- Models as the harness reports them: to be filled in from each subagent's result.

## First reports

(as delivered; filled in before any reconciliation)
