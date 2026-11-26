---
id: a-pr-whose-diff-edits-the-cylinder-sphere-frame-rows-runs-its-slow-set-step-for-hours
kind: issue
title: A PR that edits cylinder_sphere_frame or the ring composite runs its kernel slow-set step for nearly three hours and its mesh-budget job for nearly two, far past the CI bar
status: open
opened: 2026-11-26
priority: P2
cost: M
refs: [7801]
---

Filed by INSIDE's orchestrator (2026-11-26) from PR 7801's CI (`inside/face-door-spline-arm`, run 45744347800 on `71b25e3021`).

## Measured

- `test (kernel)`'s step "the slow-set tests this diff adds or edits" ran from 03:00:24Z to 05:47:22Z, which is **2 h 47 min**.
- `mesh budget meter + certificate falsifier`'s `mesh --features budget` ran from 02:45:57Z to 04:30:07Z, which is **1 h 44 min**.
- The bars are 15 min, or 25 min for a seed diff (`memories/orchestration-model.md`, "Friction is a finding").

The likely cause is the lane's reading, not yet broken down per row. Once `curve_face::cross` answers the spline-composite arm at `T: Decide`, the tier certificate's side rays and the region walks read the cylinder × sphere results' spline chords where they used to refuse at once. So the rows that build those results and certify them got much slower:
- `crates/sweep/tests/cylinder_sphere_frame.rs`. Its cap-brick row alone took 1868 s unoptimised even after it was trimmed to one capped result per pose.
- The other rows in the files the PR edits.

That is the price of doors that now answer. The question is whether the work belongs in the per-PR slow-set step at all, and whether the walk can be made cheaper.

## What would close it

- Take the per-row times from the run's junit, or a local `--profile ci` run.
- Decide per row: trim it to the fewest runs that carry its assertion; move it to the nightly only; or file the walk's cost on the reading's owner (INSIDE's ring composite, the tier certificate's side reader).
- Say where the mesh-budget hour went. That job is not the slow set, and its rise is unexplained here.
