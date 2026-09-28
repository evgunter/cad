---
id: a-tolerance-study-takes-ninety-minutes-at-eps-1e-12-unoptimized
kind: issue
title: editor-core's end-to-end tolerance study takes about 90 minutes at eps 1e-12 in the local gate, against about one second at the other eps rows
status: closed
closed: 2026-09-28
opened: 2026-09-28
priority: P4
cost: M
---


## What

`editor-core`'s `r2_m10_6_probes_interval::a_tolerance_study_end_to_end_through_the_public_doors`
(`crates/editor-core/tests/r2_m10_6_probes_interval.rs`) passed in **5360.7 s**
in the `test (eps = 1e-12)` row of a full local `ci-local.sh` run (2026-09-26,
4 cores, opt-level 0, line-tables-only debuginfo). It took about 1 s in the
default-eps and 1e-6 rows of the same run. On `origin/main` at `2a17d3f95`, the
same test under `CAD_TOLERANCE_EPS=1e-12` was still running after 29 minutes
when stopped, so the cost predates the PR that surfaced it (GERM, PR 3265).
Its geometry is two extruded boxes, all planar.

A ~5000× jump on one eps row is the row's own cost, not a slow machine. The
local gate builds tests at opt-0 and the hosted gate at opt-1, which may
account for part of it; the rest is whatever the drive or stackup does as the
band narrows by three orders.

## What would settle it

Time the test at eps 1e-12 at opt-0 and at opt-1, and profile where the time
goes (the drive's subdivision, the stackup's worst case, or the MC). Decide
whether the row is the right shape at 1e-12 (`memories/test-suite-cost.md`:
ask which shape a test is before paying for it).

## Home

TCOST: `crates/editor-core/tests/*` is its ground (and TINT's).

**2026-09-28:** `a_tolerance_study_end_to_end_through_the_public_doors` was deleted in the 2026-09-28 CI-latency cut.

Closed 2026-09-28: the test this item concerns is deleted (`work/ciw/latency-cut.md`).
