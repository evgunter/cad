# ONEREAD, TWICE merge into HONEONE — 2026-11-20

The track budget counts only the dispatchable P0, P1 and unbanded rows, because a sitting dispatches only P0 and P1 rows (`work/README.md`, Track size; Ev, in chat, 2026-11-02). A track holding no P0 or P1 row therefore has nothing a sitting would take up on its own, and Ev's standing instruction is to merge under-budget tracks that have a coherent partner (in chat, 2026-11-02). The 2026-11-20 tracker sweep merged these into HONEONE (HONEONE — one reading per question: the D3 pair-table audit, the duplicate readers on boolean ground, and one numeric fact decided in several places). Unbanded rows were banded first, so the merge adds nothing to HONEONE's load.

- **ONEREAD** — one question, two readers: the boolean's material, bend and inward-direction readings kept in step by hand. Opened 2026-11-01. Its 14 live rows (2 P2, 10 P3, 2 P4) moved to `work/honeone/` by `git mv`, ids and bodies unchanged, each with a dated Moved note; its 15 closed rows were deleted with it. Its charter, `paths` and `keep_out` are folded into `work/honeone/program.md` and its plan into `work/honeone/plan.md`, each as a `## From ONEREAD` section. Its `ab_band` (none) closes with it.
- **TWICE** — one question read twice or spelled twice, and reach measures read past their span. Opened 2026-10-17. Its 11 live rows (2 P2, 5 P3, 4 P4) moved to `work/honeone/` by `git mv`, ids and bodies unchanged, each with a dated Moved note; its 2 closed rows were deleted with it. Its charter, `paths` and `keep_out` are folded into `work/honeone/program.md` and its plan into `work/honeone/plan.md`, each as a `## From TWICE` section. Its `ab_band` (13600-13699) closes with it.

Citations of a moved row's file, and of a merged track's `plan.md` or `program.md`, outside logs and the experiment ledgers now point into `work/honeone/`. Historical prose (Moved notes, log entries) keeps the paths it was written with. ONEREAD's plan set `## Exit criteria`. Nothing in it is finished, so no walk is due now: the criteria travel with the rows, under the `## From` section of `work/honeone/plan.md`, and are walked when those rows close. The other plans set none.

Recover the program, plan and log files, and the closed rows, at `131457c32b`:

    git show 131457c32b:work/oneread/<FILE>
    git show 131457c32b:work/twice/<FILE>
