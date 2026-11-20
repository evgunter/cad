# WIREREFUSE, WIRECOST, WIREHOLD merge into DOCTAIL — 2026-11-20

The track budget counts only the dispatchable P0, P1 and unbanded rows, because a sitting dispatches only P0 and P1 rows (`work/README.md`, Track size; Ev, in chat, 2026-11-02). A track holding no P0 or P1 row therefore has nothing a sitting would take up on its own, and Ev's standing instruction is to merge under-budget tracks that have a coherent partner (in chat, 2026-11-02). The 2026-11-20 tracker sweep merged these into DOCTAIL (DOCTAIL — the document model's tail: the edit door's reports and value checks, the save format, the flip lane's reading, refusal prose, name hygiene and the certified pick). Unbanded rows were banded first, so the merge adds nothing to DOCTAIL's load.

- **WIREREFUSE** — WIRE's refusal tail: endings typed by the wrong D9 row, recourse labels spelled by hand or missing, and the eval doors' record and index plumbing. Opened 2026-10-28. Its 7 live rows (3 P3, 4 P4) moved to `work/doctail/` by `git mv`, ids and bodies unchanged, each with a dated Moved note; its 4 closed rows were deleted with it. Its charter, `paths` and `keep_out` are folded into `work/doctail/program.md` and its plan into `work/doctail/plan.md`, each as a `## From WIREREFUSE` section. Its `ab_band` (18200-18299) closes with it.
- **WIRECOST** — what a family costs to evaluate and hold: the cheaper per-member payload, the union fold over a family, and the stop condition the kernel's own evaluations take from their caller. Opened 2026-11-02. Its 4 live rows (4 P2) moved to `work/doctail/` by `git mv`, ids and bodies unchanged, each with a dated Moved note; its 1 closed rows were deleted with it. Its charter, `paths` and `keep_out` are folded into `work/doctail/program.md` and its plan into `work/doctail/plan.md`, each as a `## From WIRECOST` section. Its `ab_band` (18500-18599) closes with it.
- **WIREHOLD** — WIRE's post-D10 residue: member families and their cancelation, the placement and pose readers, forwarders the census does not weld, and the profile's structure record. Opened 2026-10-17. Its 9 live rows (1 P2, 4 P3, 4 P4) moved to `work/doctail/` by `git mv`, ids and bodies unchanged, each with a dated Moved note; its 12 closed rows were deleted with it. Its charter, `paths` and `keep_out` are folded into `work/doctail/program.md` and its plan into `work/doctail/plan.md`, each as a `## From WIREHOLD` section. Its `ab_band` (13200-13299) closes with it.

Citations of a moved row's file, and of a merged track's `plan.md` or `program.md`, outside logs and the experiment ledgers now point into `work/doctail/`. Historical prose (Moved notes, log entries) keeps the paths it was written with. None of the plans set `## Exit criteria`, so they close without an exit walk.

Recover the program, plan and log files, and the closed rows, at `131457c32b`:

    git show 131457c32b:work/wirerefuse/<FILE>
    git show 131457c32b:work/wirecost/<FILE>
    git show 131457c32b:work/wirehold/<FILE>
