# QUADTAIL, SPANFOLD merge into QUAD — 2026-11-20

The track budget counts only the dispatchable P0, P1 and unbanded rows, because a sitting dispatches only P0 and P1 rows (`work/README.md`, Track size; Ev, in chat, 2026-11-02). A track holding no P0 or P1 row therefore has nothing a sitting would take up on its own, and Ev's standing instruction is to merge under-budget tracks that have a coherent partner (in chat, 2026-11-02). The 2026-11-20 tracker sweep merged these into QUAD (QUAD — the quadrature lane's engines, its budgets and its dials). Unbanded rows were banded first, so the merge adds nothing to QUAD's load.

- **QUADTAIL** — the quadrature lane's P2–P4 tail: enclosure looseness, missing volume rules and the tests over them. Opened 2026-10-21. Its 13 live rows (5 P2, 6 P3, 2 P4) moved to `work/quad/` by `git mv`, ids and bodies unchanged, each with a dated Moved note; its 2 closed rows were deleted with it. Its charter, `paths` and `keep_out` are folded into `work/quad/program.md` and its plan into `work/quad/plan.md`, each as a `## From QUADTAIL` section. Its `ab_band` (none) closes with it.
- **SPANFOLD** — the quadrature's span folds and loop-area arms: refusals dropped through f64::min, chord lengths rooted to nearest, and pieces no arm takes. Opened 2026-10-27. Its 8 live rows (1 P2, 7 P3) moved to `work/quad/` by `git mv`, ids and bodies unchanged, each with a dated Moved note; its 0 closed rows were deleted with it. Its charter, `paths` and `keep_out` are folded into `work/quad/program.md` and its plan into `work/quad/plan.md`, each as a `## From SPANFOLD` section. Its `ab_band` (15500-15599) closes with it.

Citations of a moved row's file, and of a merged track's `plan.md` or `program.md`, outside logs and the experiment ledgers now point into `work/quad/`. Historical prose (Moved notes, log entries) keeps the paths it was written with. None of the plans set `## Exit criteria`, so they close without an exit walk.

Recover the program, plan and log files, and the closed rows, at `131457c32b`:

    git show 131457c32b:work/quadtail/<FILE>
    git show 131457c32b:work/spanfold/<FILE>
