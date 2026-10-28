---
id: twopeg-triangles-grew-past-the-tess-gate-on-main
kind: issue
title: twopeg's three bodies mesh 1.4-1.5x the baseline's triangles on main, so the nightly tess-lint gate fails there
status: closed
opened: 2026-10-02
priority: P3
cost: E
closed: 2026-10-28
---

Filed by the SHOW orchestrator on 2026-10-02.

## Finding

I ran a fresh `scripts/tess_budget_sweep.sh` on a tree that is main (`a281655cd`) merged into SHOW's snowman branch. Nothing in that branch touches `twopeg`. Against the committed baseline (cut at `24f02a376460`), `tess-lint` reports three gate findings:

- `twopeg/twopeg_mated`: triangles 116 → 164 (1.41×)
- `twopeg/twopeg_apart_p`: triangles 204 → 300 (1.47×)
- `twopeg/twopeg_apart_q`: triangles 212 → 308 (1.45×)

So the nightly tess-lint gate should fail on main for these rows, independently of any SHOW PR.

## Cause

Not traced. The candidates are recent main changes to the union and joint path that `twopeg` goes through, for example TANG's `slit_zip` / `fuse_by_joint` work. That is a lead, not a measurement.

Per `tess-lint`'s own recourse:
1. Decide whether this is a GEOMETRY change (re-baseline the rows in the PR that caused it) or a SIZING regression (fix it in the lane).
2. Do not coarsen delta.

## Moved to TESSSIZE (the curved and trimmed lanes' sizing: which density each lattice emits, and the theorem behind it) (2026-10-28)

Cut out of TESS by its semantic-seam split (`work/README.md`, Track size) when TESS measured 146 budget points against 30. Moved by `git mv`; the id and body are unchanged.

## Closed (2026-10-28)

**The cause is a geometry change, not sizing.** #3657 (REACH,
`b43aa794e5`, merged 2026-10-02 02:32, 1.5 h after the baseline's
`24f02a376460` cut) rounded the plate's corners (`CORNER_R` = 0.5 in
`demos/tour/src/twopeg.rs`). That replaced four planar corners with
four quarter-cylinder faces on every body: `twopeg_mated` 12 → 10
faces (12 planes → 6 planes + 4 cylinders), and `apart_p`/`apart_q`
14 → 18 and 12 → 16. The triangle growth is those faces. It is
intended, so it was re-baselined, not fixed. The re-cut was #5717's
(`c3ff4a85a526`), which also re-sized the rows to 76/220/228. The
per-scene reading is
`work/green/tess-lint-gate-red-on-main-23-findings.md` ("Closed"). The
nightly gate is green on main since 2026-10-27.
