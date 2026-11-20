# PROFILEREST, PROFILETAIL, ROUND merge into PROFILE — 2026-11-20

The track budget counts only the dispatchable P0, P1 and unbanded rows, because a sitting dispatches only P0 and P1 rows (`work/README.md`, Track size; Ev, in chat, 2026-11-02). A track holding no P0 or P1 row therefore has nothing a sitting would take up on its own, and Ev's standing instruction is to merge under-budget tracks that have a coherent partner (in chat, 2026-11-02). The 2026-11-20 tracker sweep merged these into PROFILE (PROFILE — the profile lattice after PATHS, its goal: the lowering epic's tail, the planar-primitive dedupe and the validation pair pass). Unbanded rows were banded first, so the merge adds nothing to PROFILE's load.

- **PROFILEREST** — the profile lattice's P2 remainder and validation's P3 reads, behind PROFILE. Opened 2026-10-21. Its 14 live rows (8 P2, 6 P3) moved to `work/profile/` by `git mv`, ids and bodies unchanged, each with a dated Moved note; its 0 closed rows were deleted with it. Its charter, `paths` and `keep_out` are folded into `work/profile/program.md` and its plan into `work/profile/plan.md`, each as a `## From PROFILEREST` section. Its `ab_band` (none) closes with it.
- **PROFILETAIL** — the profile lattice's P3 and P4 tail: the path door's gaps, the lift, the coverage and the refusal text, behind PROFILEREST. Opened 2026-10-21. Its 23 live rows (21 P3, 2 P4) moved to `work/profile/` by `git mv`, ids and bodies unchanged, each with a dated Moved note; its 2 closed rows were deleted with it. Its charter, `paths` and `keep_out` are folded into `work/profile/program.md` and its plan into `work/profile/plan.md`, each as a `## From PROFILETAIL` section. Its `ab_band` (none) closes with it.
- **ROUND** — the profile fillet family: its doors, its recourse sentences, and where a quarter of it lives. Opened 2026-09-20. Its 4 live rows (1 P2, 3 P3) moved to `work/profile/` by `git mv`, ids and bodies unchanged, each with a dated Moved note; its 18 closed rows were deleted with it. Its charter, `paths` and `keep_out` are folded into `work/profile/program.md` and its plan into `work/profile/plan.md`, each as a `## From ROUND` section. Its `ab_band` (8200-8299) closes with it.

Citations of a moved row's file, and of a merged track's `plan.md` or `program.md`, outside logs and the experiment ledgers now point into `work/profile/`. Historical prose (Moved notes, log entries) keeps the paths it was written with. None of the plans set `## Exit criteria`, so they close without an exit walk.

Recover the program, plan and log files, and the closed rows, at `131457c32b`:

    git show 131457c32b:work/profilerest/<FILE>
    git show 131457c32b:work/profiletail/<FILE>
    git show 131457c32b:work/round/<FILE>
