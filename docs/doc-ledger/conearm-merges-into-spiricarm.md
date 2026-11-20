# CONEARM merges into SPIRICARM — 2026-11-20

The track budget counts only the dispatchable P0, P1 and unbanded rows, because a sitting dispatches only P0 and P1 rows (`work/README.md`, Track size; Ev, in chat, 2026-11-02). A track holding no P0 or P1 row therefore has nothing a sitting would take up on its own, and Ev's standing instruction is to merge under-budget tracks that have a coherent partner (in chat, 2026-11-02). The 2026-11-20 tracker sweep merged it into SPIRICARM (SPIRICARM — the spiric and torus lanes: bands and rings with no lane, mirror tori, near-tangent ends, and the widths that grow with distance). Unbanded rows were banded first, so the merge adds nothing to SPIRICARM's load.

- **CONEARM** — the cone arm: what a cone face's rings and holes bound, and the section-ellipse and slant readings under its closed form. Opened 2026-10-27. Its 11 live rows (4 P2, 5 P3, 2 P4) moved to `work/spiricarm/` by `git mv`, ids and bodies unchanged, each with a dated Moved note; its 2 closed rows were deleted with it. Its charter, `paths` and `keep_out` are folded into `work/spiricarm/program.md` and its plan into `work/spiricarm/plan.md`, each as a `## From CONEARM` section. Its `ab_band` (15200-15299) closes with it.

Citations of a moved row's file, and of a merged track's `plan.md` or `program.md`, outside logs and the experiment ledgers now point into `work/spiricarm/`. Historical prose (Moved notes, log entries) keeps the paths it was written with. None of the plans set `## Exit criteria`, so they close without an exit walk.

Recover the program, plan and log files, and the closed rows, at `131457c32b`:

    git show 131457c32b:work/conearm/<FILE>
