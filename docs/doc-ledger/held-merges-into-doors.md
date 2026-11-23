# HELD merges into DOORS — 2026-11-20

The track budget counts only the dispatchable P0, P1 and unbanded rows, because a sitting dispatches only P0 and P1 rows (`work/README.md`, Track size; Ev, in chat, 2026-11-02). A track holding no P0 or P1 row therefore has nothing a sitting would take up on its own, and Ev's standing instruction is to merge under-budget tracks that have a coherent partner (in chat, 2026-11-02). The 2026-11-20 tracker sweep merged it into DOORS (DOORS — the viewer's authoring doors: the halves of AUTHOR's ruled node shapes, and what a takeover keeps of what reads the body it takes over). Unbanded rows were banded first, so the merge adds nothing to DOORS's load.

- **HELD** — what the viewer holds while a person authors: held picks and their marks, the picks it authors, and the frames its gestures read back. Opened 2026-11-01. Its 9 live rows (4 P2, 4 P3, 1 P4) moved to `work/doors/` by `git mv`, ids and bodies unchanged, each with a dated Moved note; its 0 closed rows were deleted with it. Its charter, `paths` and `keep_out` are folded into `work/doors/program.md` and its plan into `work/doors/plan.md`, each as a `## From HELD` section. Its `ab_band` (none) closes with it.

Citations of a moved row's file, and of a merged track's `plan.md` or `program.md`, outside logs and the experiment ledgers now point into `work/doors/`. Historical prose (Moved notes, log entries) keeps the paths it was written with. None of the plans set `## Exit criteria`, so they close without an exit walk.

Recover the program, plan and log files, and the closed rows, at `131457c32b`:

    git show 131457c32b:work/held/<FILE>
