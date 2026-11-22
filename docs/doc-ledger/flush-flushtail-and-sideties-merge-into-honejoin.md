# FLUSH, FLUSHTAIL, SIDETIES merge into HONEJOIN — 2026-11-20

The track budget counts only the dispatchable P0, P1 and unbanded rows, because a sitting dispatches only P0 and P1 rows (`work/README.md`, Track size; Ev, in chat, 2026-11-02). A track holding no P0 or P1 row therefore has nothing a sitting would take up on its own, and Ev's standing instruction is to merge under-budget tracks that have a coherent partner (in chat, 2026-11-02). The 2026-11-20 tracker sweep merged these into HONEJOIN (HONEJOIN — the curved and chord join's P3 residue: arms its gates admit, invariants it only argues, refusals it misfiles). Unbanded rows were banded first, so the merge adds nothing to HONEJOIN's load.

- **FLUSH** — planar and declared-contact correctness at the join. Opened 2026-10-10. Its 10 live rows (2 P2, 8 P3) moved to `work/honejoin/` by `git mv`, ids and bodies unchanged, each with a dated Moved note; its 52 closed rows were deleted with it. Its charter, `paths` and `keep_out` are folded into `work/honejoin/program.md` and its plan into `work/honejoin/plan.md`, each as a `## From FLUSH` section. Its `ab_band` (none) closes with it.
- **FLUSHTAIL** — the join's tail: the side reader's bounds and ledgers, the glue door's quiet paths, and the curved-merge and conic-span residues. Opened 2026-10-20. Its 13 live rows (12 P3, 1 P4) moved to `work/honejoin/` by `git mv`, ids and bodies unchanged, each with a dated Moved note; its 2 closed rows were deleted with it. Its charter, `paths` and `keep_out` are folded into `work/honejoin/program.md` and its plan into `work/honejoin/plan.md`, each as a `## From FLUSHTAIL` section. Its `ab_band` (none) closes with it.
- **SIDETIES** — the side reader's remaining exact ties at a nominal, and the region walks still behind them. Opened 2026-10-31. Its 5 live rows (5 P2) moved to `work/honejoin/` by `git mv`, ids and bodies unchanged, each with a dated Moved note; its 0 closed rows were deleted with it. Its charter, `paths` and `keep_out` are folded into `work/honejoin/program.md` and its plan into `work/honejoin/plan.md`, each as a `## From SIDETIES` section. Its `ab_band` (none) closes with it.

Citations of a moved row's file, and of a merged track's `plan.md` or `program.md`, outside logs and the experiment ledgers now point into `work/honejoin/`. Historical prose (Moved notes, log entries) keeps the paths it was written with. None of the plans set `## Exit criteria`, so they close without an exit walk.

Recover the program, plan and log files, and the closed rows, at `131457c32b`:

    git show 131457c32b:work/flush/<FILE>
    git show 131457c32b:work/flushtail/<FILE>
    git show 131457c32b:work/sideties/<FILE>
