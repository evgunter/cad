# WIREREST, SEAMRANK merge into BORDERS — 2026-11-20

The track budget counts only the dispatchable P0, P1 and unbanded rows, because a sitting dispatches only P0 and P1 rows (`work/README.md`, Track size; Ev, in chat, 2026-11-02). A track holding no P0 or P1 row therefore has nothing a sitting would take up on its own, and Ev's standing instruction is to merge under-budget tracks that have a coherent partner (in chat, 2026-11-02). The 2026-11-20 tracker sweep merged these into BORDERS (BORDERS — a piece's Borders and the lineage it reads: the joins the records cannot place, the pieces named bare, and the unions that refuse by member order). Unbanded rows were banded first, so the merge adds nothing to BORDERS's load.

- **WIREREST** — WIRE's remainder: the naming records of splits, unions and booleans (fold steps, coincidence rows, lineage, tie witnesses) and naming tests that do not hold at every K. Opened 2026-10-20. Its 23 live rows (22 P3, 1 P4) moved to `work/borders/` by `git mv`, ids and bodies unchanged, each with a dated Moved note; its 2 closed rows were deleted with it. Its charter, `paths` and `keep_out` are folded into `work/borders/program.md` and its plan into `work/borders/plan.md`, each as a `## From WIREREST` section. Its `ab_band` (14300-14399) closes with it.
- **SEAMRANK** — what a union's crossing vertex cites and how a run ranks: the conventional vertex, the seam sides that do not settle, and the carriers the flush rule cannot read. Opened 2026-10-19. Its 7 live rows (1 P2, 6 P3) moved to `work/borders/` by `git mv`, ids and bodies unchanged, each with a dated Moved note; its 6 closed rows were deleted with it. Its charter, `paths` and `keep_out` are folded into `work/borders/program.md` and its plan into `work/borders/plan.md`, each as a `## From SEAMRANK` section. Its `ab_band` (13800-13899) closes with it.

Citations of a moved row's file, and of a merged track's `plan.md` or `program.md`, outside logs and the experiment ledgers now point into `work/borders/`. Historical prose (Moved notes, log entries) keeps the paths it was written with. None of the plans set `## Exit criteria`, so they close without an exit walk.

Recover the program, plan and log files, and the closed rows, at `131457c32b`:

    git show 131457c32b:work/wirerest/<FILE>
    git show 131457c32b:work/seamrank/<FILE>
