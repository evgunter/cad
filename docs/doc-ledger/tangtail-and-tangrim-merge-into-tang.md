# TANGTAIL, TANGRIM merge into TANG — 2026-11-20

The track budget counts only the dispatchable P0, P1 and unbanded rows, because a sitting dispatches only P0 and P1 rows (`work/README.md`, Track size; Ev, in chat, 2026-11-02). A track holding no P0 or P1 row therefore has nothing a sitting would take up on its own, and Ev's standing instruction is to merge under-budget tracks that have a coherent partner (in chat, 2026-11-02). The 2026-11-20 tracker sweep merged these into TANG (TANG — declared tangency (kiss and cusp), the germ and pierce remainder, and the pinch machinery). Unbanded rows were banded first, so the merge adds nothing to TANG's load.

- **TANGTAIL** — TANG's P3 tail: contact-line and ring corners, the banked torus rest lane, Approx and offset readings, fixtures. Opened 2026-10-17. Its 8 live rows (8 P3) moved to `work/tang/` by `git mv`, ids and bodies unchanged, each with a dated Moved note; its 2 closed rows were deleted with it. Its charter, `paths` and `keep_out` are folded into `work/tang/program.md` and its plan into `work/tang/plan.md`, each as a `## From TANGTAIL` section. Its `ab_band` (13700-13799) closes with it.
- **TANGRIM** — tangent curved rims and crescents: unions and differences of curved faces that touch tangent at a point or along a line. Opened 2026-11-01. Its 20 live rows (14 P2, 6 P3) moved to `work/tang/` by `git mv`, ids and bodies unchanged, each with a dated Moved note; its 11 closed rows were deleted with it. Its charter, `paths` and `keep_out` are folded into `work/tang/program.md` and its plan into `work/tang/plan.md`, each as a `## From TANGRIM` section. Its `ab_band` (none) closes with it.

Citations of a moved row's file, and of a merged track's `plan.md` or `program.md`, outside logs and the experiment ledgers now point into `work/tang/`. Historical prose (Moved notes, log entries) keeps the paths it was written with. None of the plans set `## Exit criteria`, so they close without an exit walk.

Recover the program, plan and log files, and the closed rows, at `131457c32b`:

    git show 131457c32b:work/tangtail/<FILE>
    git show 131457c32b:work/tangrim/<FILE>
