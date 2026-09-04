# MSOLVE log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/msolve/plan.md`.

## Opened (2026-09-04)

Opened by the FIX orchestrator on Ev's steer (in-chat, PR 1731 thread:
DOCM "feels somewhat different — it's ok to open a successor unit to
S-MATE if that's what makes sense").

The five items below were measured or ruled during FIX's run and have
no owner: S-MATE closed while they were in flight, and DOCM inherited
the FILES rather than this class of question. Re-homed here by header
edit and `git mv`, ids unchanged.

Two things this program starts with that most do not:

- **A live defect with characterization rows already on main.** PR 1773
  pins the transform-blind solve as a known-wrong answer, with a header
  saying the fix DELETES the rows rather than updating them. They go red
  when item (1) is fixed; that is the signal, not a regression.
- **A ruling already made.** Ev ruled item (2) in on PR 1731, and the
  sequencing — the gate first — with it.

The territory overlap with DOCM is announced on their orchestrator PR,
not assumed. Nothing has been taken from them.
