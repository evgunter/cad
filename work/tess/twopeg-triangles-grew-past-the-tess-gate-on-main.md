---
id: twopeg-triangles-grew-past-the-tess-gate-on-main
kind: issue
title: twopeg's three bodies mesh 1.4-1.5x the baseline's triangles on main, so the nightly tess-lint gate fails there
status: open
opened: 2026-10-02
priority: P3
cost: E
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
