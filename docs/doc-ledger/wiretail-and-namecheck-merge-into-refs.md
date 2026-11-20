# WIRETAIL, NAMECHECK merge into REFS — 2026-11-20

The track budget counts only the dispatchable P0, P1 and unbanded rows, because a sitting dispatches only P0 and P1 rows (`work/README.md`, Track size; Ev, in chat, 2026-11-02). A track holding no P0 or P1 row therefore has nothing a sitting would take up on its own, and Ev's standing instruction is to merge under-budget tracks that have a coherent partner (in chat, 2026-11-02). The 2026-11-20 tracker sweep merged these into REFS (REFS — references and reads: a name scoped by its read, indexed reads, and the façade's slot words). Unbanded rows were banded first, so the merge adds nothing to REFS's load.

- **WIRETAIL** — WIRE's P3 and P4 tail: the selection vocabulary's missing atoms, label and kind-mirror hygiene, and at-rest gating repeated per consumer. Opened 2026-10-17. Its 12 live rows (7 P3, 5 P4) moved to `work/refs/` by `git mv`, ids and bodies unchanged, each with a dated Moved note; its 1 closed rows were deleted with it. Its charter, `paths` and `keep_out` are folded into `work/refs/program.md` and its plan into `work/refs/plan.md`, each as a `## From WIRETAIL` section. Its `ab_band` (13300-13399) closes with it.
- **NAMECHECK** — N0's checks: the denotation verifier, the coverage fuzzer, the RoleSeg form map, the storage-order perturbation, and the clauses that point at N0. Opened 2026-10-20. Its 5 live rows (4 P2, 1 P3) moved to `work/refs/` by `git mv`, ids and bodies unchanged, each with a dated Moved note; its 0 closed rows were deleted with it. Its charter, `paths` and `keep_out` are folded into `work/refs/program.md` and its plan into `work/refs/plan.md`, each as a `## From NAMECHECK` section. Its `ab_band` (14200-14299) closes with it.

Citations of a moved row's file, and of a merged track's `plan.md` or `program.md`, outside logs and the experiment ledgers now point into `work/refs/`. Historical prose (Moved notes, log entries) keeps the paths it was written with. None of the plans set `## Exit criteria`, so they close without an exit walk.

Recover the program, plan and log files, and the closed rows, at `131457c32b`:

    git show 131457c32b:work/wiretail/<FILE>
    git show 131457c32b:work/namecheck/<FILE>
