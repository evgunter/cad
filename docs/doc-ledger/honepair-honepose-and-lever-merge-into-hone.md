# HONEPAIR, HONEPOSE, LEVER merge into HONE — 2026-11-20

The track budget counts only the dispatchable P0, P1 and unbanded rows, because a sitting dispatches only P0 and P1 rows (`work/README.md`, Track size; Ev, in chat, 2026-11-02). A track holding no P0 or P1 row therefore has nothing a sitting would take up on its own, and Ev's standing instruction is to merge under-budget tracks that have a coherent partner (in chat, 2026-11-02). The 2026-11-20 tracker sweep merged these into HONE (HONE — the split and boolean lane's P2 residue: margins and error propagation on split and boolean ground). Unbanded rows were banded first, so the merge adds nothing to HONE's load.

- **HONEPAIR** — one verdict on one margin at its own reach: split and boolean readings that decide two margins one at a time or lever a reading at the wrong extent. Opened 2026-10-26. Its 10 live rows (10 P2) moved to `work/hone/` by `git mv`, ids and bodies unchanged, each with a dated Moved note; its 3 closed rows were deleted with it. Its charter, `paths` and `keep_out` are folded into `work/hone/program.md` and its plan into `work/hone/plan.md`, each as a `## From HONEPAIR` section. Its `ab_band` (none) closes with it.
- **HONEPOSE** — split and boolean poses that refuse in band where nothing is near a decision. Opened 2026-10-26. Its 12 live rows (12 P2) moved to `work/hone/` by `git mv`, ids and bodies unchanged, each with a dated Moved note; its 0 closed rows were deleted with it. Its charter, `paths` and `keep_out` are folded into `work/hone/program.md` and its plan into `work/hone/plan.md`, each as a `## From HONEPOSE` section. Its `ab_band` (none) closes with it.
- **LEVER** — a lever read at the extent its decision consumes, not a whole face, a pivot or a 3-D distance. Opened 2026-10-17. Its 16 live rows (5 P2, 11 P3) moved to `work/hone/` by `git mv`, ids and bodies unchanged, each with a dated Moved note; its 3 closed rows were deleted with it. Its charter, `paths` and `keep_out` are folded into `work/hone/program.md` and its plan into `work/hone/plan.md`, each as a `## From LEVER` section. Its `ab_band` (13500-13599) closes with it.

Citations of a moved row's file, and of a merged track's `plan.md` or `program.md`, outside logs and the experiment ledgers now point into `work/hone/`. Historical prose (Moved notes, log entries) keeps the paths it was written with. None of the plans set `## Exit criteria`, so they close without an exit walk.

Recover the program, plan and log files, and the closed rows, at `131457c32b`:

    git show 131457c32b:work/honepair/<FILE>
    git show 131457c32b:work/honepose/<FILE>
    git show 131457c32b:work/lever/<FILE>
