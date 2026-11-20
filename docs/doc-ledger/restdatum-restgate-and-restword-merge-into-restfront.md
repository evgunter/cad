# RESTDATUM, RESTGATE, RESTWORD merge into RESTFRONT — 2026-11-20

The track budget counts only the dispatchable P0, P1 and unbanded rows, because a sitting dispatches only P0 and P1 rows (`work/README.md`, Track size; Ev, in chat, 2026-11-02). A track holding no P0 or P1 row therefore has nothing a sitting would take up on its own, and Ev's standing instruction is to merge under-budget tracks that have a coherent partner (in chat, 2026-11-02). The 2026-11-20 tracker sweep merged these into RESTFRONT (RESTFRONT — what tier 3 passes at rest that it should refuse: a face's region on the plane and off it, and a body's shells and solids). Unbanded rows were banded first, so the merge adds nothing to RESTFRONT's load.

- **RESTDATUM** — checks 1 and 4: a carrier read against its datum and against its faces. Opened 2026-10-29. Its 6 live rows (1 P2, 4 P3, 1 P4) moved to `work/restfront/` by `git mv`, ids and bodies unchanged, each with a dated Moved note; its 0 closed rows were deleted with it. Its charter, `paths` and `keep_out` are folded into `work/restfront/program.md` and its plan into `work/restfront/plan.md`, each as a `## From RESTDATUM` section. Its `ab_band` (none) closes with it.
- **RESTGATE** — what the at-rest gate admits a finished body and an operand on. Opened 2026-10-29. Its 5 live rows (1 P2, 3 P3, 1 P4) moved to `work/restfront/` by `git mv`, ids and bodies unchanged, each with a dated Moved note; its 0 closed rows were deleted with it. Its charter, `paths` and `keep_out` are folded into `work/restfront/program.md` and its plan into `work/restfront/plan.md`, each as a `## From RESTGATE` section. Its `ab_band` (none) closes with it.
- **RESTWORD** — how validate decides and words its refusals: one funnel, one ending per reading. Opened 2026-10-29. Its 11 live rows (5 P3, 6 P4) moved to `work/restfront/` by `git mv`, ids and bodies unchanged, each with a dated Moved note; its 1 closed rows were deleted with it. Its charter, `paths` and `keep_out` are folded into `work/restfront/program.md` and its plan into `work/restfront/plan.md`, each as a `## From RESTWORD` section. Its `ab_band` (none) closes with it.

Citations of a moved row's file, and of a merged track's `plan.md` or `program.md`, outside logs and the experiment ledgers now point into `work/restfront/`. Historical prose (Moved notes, log entries) keeps the paths it was written with. None of the plans set `## Exit criteria`, so they close without an exit walk.

Recover the program, plan and log files, and the closed rows, at `131457c32b`:

    git show 131457c32b:work/restdatum/<FILE>
    git show 131457c32b:work/restgate/<FILE>
    git show 131457c32b:work/restword/<FILE>
