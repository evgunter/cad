# TOUCHREC merges into HELDRECORDS — 2026-11-20

The track budget counts only the dispatchable P0, P1 and unbanded rows, because a sitting dispatches only P0 and P1 rows (`work/README.md`, Track size; Ev, in chat, 2026-11-02). A track holding no P0 or P1 row therefore has nothing a sitting would take up on its own, and Ev's standing instruction is to merge under-budget tracks that have a coherent partner (in chat, 2026-11-02). The 2026-11-20 tracker sweep merged it into HELDRECORDS (HELDRECORDS — a finished body holds its contact records, and every door gates what it returns at tier 3′). Unbanded rows were banded first, so the merge adds nothing to HELDRECORDS's load.

- **TOUCHREC** — a boolean result holds the records of its own touches and keeps the copy the glue decided. Opened 2026-11-01. Its 8 live rows (5 P2, 1 P3, 2 P4) moved to `work/heldrecords/` by `git mv`, ids and bodies unchanged, each with a dated Moved note; its 3 closed rows were deleted with it. Its charter, `paths` and `keep_out` are folded into `work/heldrecords/program.md` and its plan into `work/heldrecords/plan.md`, each as a `## From TOUCHREC` section. Its `ab_band` (none) closes with it.

Citations of a moved row's file, and of a merged track's `plan.md` or `program.md`, outside logs and the experiment ledgers now point into `work/heldrecords/`. Historical prose (Moved notes, log entries) keeps the paths it was written with. TOUCHREC's plan set `## Exit criteria`. Nothing in it is finished, so no walk is due now: the criteria travel with the rows, under the `## From` section of `work/heldrecords/plan.md`, and are walked when those rows close. The other plans set none.

Recover the program, plan and log files, and the closed rows, at `131457c32b`:

    git show 131457c32b:work/touchrec/<FILE>
