# RESTREAD, SECTOR merge into CONTACTHOLD — 2026-11-20

The track budget counts only the dispatchable P0, P1 and unbanded rows, because a sitting dispatches only P0 and P1 rows (`work/README.md`, Track size; Ev, in chat, 2026-11-02). A track holding no P0 or P1 row therefore has nothing a sitting would take up on its own, and Ev's standing instruction is to merge under-budget tracks that have a coherent partner (in chat, 2026-11-02). The 2026-11-20 tracker sweep merged these into CONTACTHOLD (CONTACTHOLD — CONTACT's remaining rows: the coincidence door and the declared-pair retirement). Unbanded rows were banded first, so the merge adds nothing to CONTACTHOLD's load.

- **RESTREAD** — the census's near-band reads in its crossing and edge-on-face lanes, the cross-solid curved pairs, and the P1 point and seam readings. Opened 2026-10-09. Its 39 live rows (8 P2, 26 P3, 5 P4) moved to `work/contacthold/` by `git mv`, ids and bodies unchanged, each with a dated Moved note; its 43 closed rows were deleted with it. Its charter, `paths` and `keep_out` are folded into `work/contacthold/program.md` and its plan into `work/contacthold/plan.md`, each as a `## From RESTREAD` section. Its `ab_band` (11800-11899) closes with it.
- **SECTOR** — CONTACT's P3 tail: sector-arm levers, self-touches and wedge joins, torn records, and the gate readers that drop a verdict. Opened 2026-10-09. Its 11 live rows (11 P3) moved to `work/contacthold/` by `git mv`, ids and bodies unchanged, each with a dated Moved note; its 4 closed rows were deleted with it. Its charter, `paths` and `keep_out` are folded into `work/contacthold/program.md` and its plan into `work/contacthold/plan.md`, each as a `## From SECTOR` section. Its `ab_band` (12000-12099) closes with it.

Citations of a moved row's file, and of a merged track's `plan.md` or `program.md`, outside logs and the experiment ledgers now point into `work/contacthold/`. Historical prose (Moved notes, log entries) keeps the paths it was written with. None of the plans set `## Exit criteria`, so they close without an exit walk.

Recover the program, plan and log files, and the closed rows, at `131457c32b`:

    git show 131457c32b:work/restread/<FILE>
    git show 131457c32b:work/sector/<FILE>
