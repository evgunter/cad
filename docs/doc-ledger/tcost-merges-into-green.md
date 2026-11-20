# TCOST merges into GREEN — 2026-11-20

The track budget counts only the dispatchable P0, P1 and unbanded rows, because a sitting dispatches only P0 and P1 rows (`work/README.md`, Track size; Ev, in chat, 2026-11-02). A track holding no P0 or P1 row therefore has nothing a sitting would take up on its own, and Ev's standing instruction is to merge under-budget tracks that have a coherent partner (in chat, 2026-11-02). The 2026-11-20 tracker sweep merged it into GREEN (GREEN — a green main and a fast gate: the infra rows that matter now). Unbanded rows were banded first, so the merge adds nothing to GREEN's load.

- **TCOST** — test-suite cost. Opened 2026-09-02. Its 17 live rows (1 P2, 6 P3, 10 P4) moved to `work/green/` by `git mv`, ids and bodies unchanged, each with a dated Moved note; its 33 closed rows were deleted with it. Its charter, `paths` and `keep_out` are folded into `work/green/program.md` and its plan into `work/green/plan.md`, each as a `## From TCOST` section. Its `ab_band` (1400-1499) closes with it.

Citations of a moved row's file, and of a merged track's `plan.md` or `program.md`, outside logs and the experiment ledgers now point into `work/green/`. Historical prose (Moved notes, log entries) keeps the paths it was written with. None of the plans set `## Exit criteria`, so they close without an exit walk.

Recover the program, plan and log files, and the closed rows, at `131457c32b`:

    git show 131457c32b:work/tcost/<FILE>
