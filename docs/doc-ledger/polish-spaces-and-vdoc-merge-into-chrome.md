# POLISH, SPACES, VDOC merge into CHROME — 2026-11-20

The track budget counts only the dispatchable P0, P1 and unbanded rows, because a sitting dispatches only P0 and P1 rows (`work/README.md`, Track size; Ev, in chat, 2026-11-02). A track holding no P0 or P1 row therefore has nothing a sitting would take up on its own, and Ev's standing instruction is to merge under-budget tracks that have a coherent partner (in chat, 2026-11-02). The 2026-11-20 tracker sweep merged these into CHROME (CHROME — viewer chrome and coverage). Unbanded rows were banded first, so the merge adds nothing to CHROME's load.

- **POLISH** — viewer chrome that shows the right information in a rougher shape than it could. Opened 2026-10-01. Its 5 live rows (5 P4) moved to `work/chrome/` by `git mv`, ids and bodies unchanged, each with a dated Moved note; its 1 closed rows were deleted with it. Its charter, `paths` and `keep_out` are folded into `work/chrome/program.md` and its plan into `work/chrome/plan.md`, each as a `## From POLISH` section. Its `ab_band` (10200-10299) closes with it.
- **SPACES** — where a new body lands, which spaces the picture shows, and what reaches the product. Opened 2026-11-01. Its 7 live rows (5 P2, 2 P3) moved to `work/chrome/` by `git mv`, ids and bodies unchanged, each with a dated Moved note; its 16 closed rows were deleted with it. Its charter, `paths` and `keep_out` are folded into `work/chrome/program.md` and its plan into `work/chrome/plan.md`, each as a `## From SPACES` section. Its `ab_band` (none) closes with it.
- **VDOC** — the viewer's prose, citations and censuses. Opened 2026-09-17. Its 21 live rows (1 P2, 4 P3, 16 P4) moved to `work/chrome/` by `git mv`, ids and bodies unchanged, each with a dated Moved note; its 33 closed rows were deleted with it. Its charter, `paths` and `keep_out` are folded into `work/chrome/program.md` and its plan into `work/chrome/plan.md`, each as a `## From VDOC` section. Its `ab_band` (5500-5599) closes with it.

Citations of a moved row's file, and of a merged track's `plan.md` or `program.md`, outside logs and the experiment ledgers now point into `work/chrome/`. Historical prose (Moved notes, log entries) keeps the paths it was written with. None of the plans set `## Exit criteria`, so they close without an exit walk.

Recover the program, plan and log files, and the closed rows, at `131457c32b`:

    git show 131457c32b:work/polish/<FILE>
    git show 131457c32b:work/spaces/<FILE>
    git show 131457c32b:work/vdoc/<FILE>
