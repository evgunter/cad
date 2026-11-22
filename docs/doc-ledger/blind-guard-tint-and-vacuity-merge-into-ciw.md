# BLIND, GUARD, TINT, VACUITY merge into CIW — 2026-11-20

The track budget counts only the dispatchable P0, P1 and unbanded rows, because a sitting dispatches only P0 and P1 rows (`work/README.md`, Track size; Ev, in chat, 2026-11-02). A track holding no P0 or P1 row therefore has nothing a sitting would take up on its own, and Ev's standing instruction is to merge under-budget tracks that have a coherent partner (in chat, 2026-11-02). The 2026-11-20 tracker sweep merged these into CIW (CIW — hosted CI, workflows and scripts, and what no gate reads). Unbanded rows were banded first, so the merge adds nothing to CIW's load.

- **BLIND** — the kernel-discipline gates' matchers, and the spellings they miss. Opened 2026-10-26. Its 13 live rows (3 P2, 8 P3, 2 P4) moved to `work/ciw/` by `git mv`, ids and bodies unchanged, each with a dated Moved note; its 0 closed rows were deleted with it. Its charter, `paths` and `keep_out` are folded into `work/ciw/program.md` and its plan into `work/ciw/plan.md`, each as a `## From BLIND` section. Its `ab_band` (15600-15699) closes with it.
- **GUARD** — the gate harness: the shared readers and the selftests. Opened 2026-09-11. Its 16 live rows (2 P2, 12 P3, 2 P4) moved to `work/ciw/` by `git mv`, ids and bodies unchanged, each with a dated Moved note; its 12 closed rows were deleted with it. Its charter, `paths` and `keep_out` are folded into `work/ciw/program.md` and its plan into `work/ciw/plan.md`, each as a `## From GUARD` section. Its `ab_band` (3600-3699) closes with it.
- **TINT** — test-suite integrity. Opened 2026-09-11. Its 75 live rows (1 P2, 54 P3, 20 P4) moved to `work/ciw/` by `git mv`, ids and bodies unchanged, each with a dated Moved note; its 22 closed rows were deleted with it. Its charter, `paths` and `keep_out` are folded into `work/ciw/program.md` and its plan into `work/ciw/plan.md`, each as a `## From TINT` section. Its `ab_band` (3500-3599) closes with it.
- **VACUITY** — rows that cannot fail: assertions on their own codomain, probes that print, floors that count what they discard. Opened 2026-09-20. Its 20 live rows (17 P3, 3 P4) moved to `work/ciw/` by `git mv`, ids and bodies unchanged, each with a dated Moved note; its 0 closed rows were deleted with it. Its charter, `paths` and `keep_out` are folded into `work/ciw/program.md` and its plan into `work/ciw/plan.md`, each as a `## From VACUITY` section. Its `ab_band` (8800-8899) closes with it.

Citations of a moved row's file, and of a merged track's `plan.md` or `program.md`, outside logs and the experiment ledgers now point into `work/ciw/`. Historical prose (Moved notes, log entries) keeps the paths it was written with. None of the plans set `## Exit criteria`, so they close without an exit walk.

Recover the program, plan and log files, and the closed rows, at `131457c32b`:

    git show 131457c32b:work/blind/<FILE>
    git show 131457c32b:work/guard/<FILE>
    git show 131457c32b:work/tint/<FILE>
    git show 131457c32b:work/vacuity/<FILE>
