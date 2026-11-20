# CENSUS merges into BIND — 2026-11-20

The track budget counts only the dispatchable P0, P1 and unbanded rows, because a sitting dispatches only P0 and P1 rows (`work/README.md`, Track size; Ev, in chat, 2026-11-02). A track holding no P0 or P1 row therefore has nothing a sitting would take up on its own, and Ev's standing instruction is to merge under-budget tracks that have a coherent partner (in chat, 2026-11-02). The 2026-11-20 tracker sweep merged it into BIND (BIND — the bindings' own housekeeping: the stubs, the tag pins, the censuses and the prose beside them). Unbanded rows were banded first, so the merge adds nothing to BIND's load.

- **CENSUS** — one vocabulary, spelled by hand in several places. Opened 2026-09-11. Its 32 live rows (21 P3, 11 P4) moved to `work/bind/` by `git mv`, ids and bodies unchanged, each with a dated Moved note; its 10 closed rows were deleted with it. Its charter, `paths` and `keep_out` are folded into `work/bind/program.md` and its plan into `work/bind/plan.md`, each as a `## From CENSUS` section. Its `ab_band` (3900-3999) closes with it.

Citations of a moved row's file, and of a merged track's `plan.md` or `program.md`, outside logs and the experiment ledgers now point into `work/bind/`. Historical prose (Moved notes, log entries) keeps the paths it was written with. None of the plans set `## Exit criteria`, so they close without an exit walk.

Recover the program, plan and log files, and the closed rows, at `131457c32b`:

    git show 131457c32b:work/census/<FILE>
