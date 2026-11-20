# TESSWALK, TESSWORD merge into TESSLIE — 2026-11-20

The track budget counts only the dispatchable P0, P1 and unbanded rows, because a sitting dispatches only P0 and P1 rows (`work/README.md`, Track size; Ev, in chat, 2026-11-02). A track holding no P0 or P1 row therefore has nothing a sitting would take up on its own, and Ev's standing instruction is to merge under-budget tracks that have a coherent partner (in chat, 2026-11-02). The 2026-11-20 tracker sweep merged these into TESSLIE (TESSLIE — the mesh answers Ok, or panics, where it should refuse or mesh right: non-manifold meshes, short meshes, debug asserts that fire and empty patches). Unbanded rows were banded first, so the merge adds nothing to TESSLIE's load.

- **TESSWALK** — what the walk and the lanes decide: structural premises without a body-level witness, and readings taken off undecided numbers. Opened 2026-10-28. Its 11 live rows (3 P2, 6 P3, 2 P4) moved to `work/tesslie/` by `git mv`, ids and bodies unchanged, each with a dated Moved note; its 2 closed rows were deleted with it. Its charter, `paths` and `keep_out` are folded into `work/tesslie/program.md` and its plan into `work/tesslie/plan.md`, each as a `## From TESSWALK` section. Its `ab_band` (16000-16099) closes with it.
- **TESSWORD** — tessellation refusal words, mesh docs and contract hygiene. Opened 2026-10-28. Its 10 live rows (7 P3, 3 P4) moved to `work/tesslie/` by `git mv`, ids and bodies unchanged, each with a dated Moved note; its 0 closed rows were deleted with it. Its charter, `paths` and `keep_out` are folded into `work/tesslie/program.md` and its plan into `work/tesslie/plan.md`, each as a `## From TESSWORD` section. Its `ab_band` (16300-16399) closes with it.

Citations of a moved row's file, and of a merged track's `plan.md` or `program.md`, outside logs and the experiment ledgers now point into `work/tesslie/`. Historical prose (Moved notes, log entries) keeps the paths it was written with. None of the plans set `## Exit criteria`, so they close without an exit walk.

Recover the program, plan and log files, and the closed rows, at `131457c32b`:

    git show 131457c32b:work/tesswalk/<FILE>
    git show 131457c32b:work/tessword/<FILE>
