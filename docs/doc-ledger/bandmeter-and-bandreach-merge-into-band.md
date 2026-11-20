# BANDMETER, BANDREACH merge into BAND — 2026-11-20

The track budget counts only the dispatchable P0, P1 and unbanded rows, because a sitting dispatches only P0 and P1 rows (`work/README.md`, Track size; Ev, in chat, 2026-11-02). A track holding no P0 or P1 row therefore has nothing a sitting would take up on its own, and Ev's standing instruction is to merge under-budget tracks that have a coherent partner (in chat, 2026-11-02). The 2026-11-20 tracker sweep merged these into BAND (BAND — the blend and rim bands: what a roll builds over a rim, what the band-feature rule takes that it still refuses, and what it may call the result). Unbanded rows were banded first, so the merge adds nothing to BAND's load.

- **BANDMETER** — the blend surgery's ring, cap and strip meters reading wider than they need. Opened 2026-10-26. Its 12 live rows (12 P3) moved to `work/band/` by `git mv`, ids and bodies unchanged, each with a dated Moved note; its 0 closed rows were deleted with it. Its charter, `paths` and `keep_out` are folded into `work/band/program.md` and its plan into `work/band/plan.md`, each as a `## From BANDMETER` section. Its `ab_band` (18000-18099) closes with it.
- **BANDREACH** — predicate 2 reading wider than it needs: the blend's reach meter and its support screen. Opened 2026-10-26. Its 10 live rows (1 P2, 9 P3) moved to `work/band/` by `git mv`, ids and bodies unchanged, each with a dated Moved note; its 2 closed rows were deleted with it. Its charter, `paths` and `keep_out` are folded into `work/band/program.md` and its plan into `work/band/plan.md`, each as a `## From BANDREACH` section. Its `ab_band` (17900-17999) closes with it.

Citations of a moved row's file, and of a merged track's `plan.md` or `program.md`, outside logs and the experiment ledgers now point into `work/band/`. Historical prose (Moved notes, log entries) keeps the paths it was written with. None of the plans set `## Exit criteria`, so they close without an exit walk.

Recover the program, plan and log files, and the closed rows, at `131457c32b`:

    git show 131457c32b:work/bandmeter/<FILE>
    git show 131457c32b:work/bandreach/<FILE>
