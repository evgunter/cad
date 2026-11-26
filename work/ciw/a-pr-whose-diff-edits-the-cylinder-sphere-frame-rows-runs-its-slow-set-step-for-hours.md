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

## 2026-11-26 — per-row times (PR 7801's lane, run 45774641309's slow-set step)

The step took 7641 s of wall time over 151 tests. The slowest rows, all in `crates/sweep/tests/cylinder_sphere_frame.rs`:

| row | time |
|---|---|
| `…_wider_than_the_wall` | 7641 s, the whole step |
| `…_in_two_loops_turned` | 2234 s |
| `…_in_two_loops` | 2190 s |
| `…_centre_on_the_wall` | 1921 s |
| `…_turned_chart` | 1043 s |
| `…_centre_outside` | 652 s |
| the cap row | 641 s |
| `…_centre_inside` | 570 s |

For comparison, SECT's nightly census on 2026-11-02 at 1e-12 took 250 s for each two-loop row and 168 s for `wider`. So the two-loop rows now cost about 9× that, and at the default ε.

Stack samples were taken on the cap row only. They put the time in the tier certificate's side reader: finish → validate → `plus_v_errors` → `side_reader::side_at_infinity` → `cast_aims` → sphere-region landings → the composite walk. These are aimed rays reading spline chords, which used to refuse at once. The other rows and the mesh-budget hour were not profiled.

Likely levers, unmeasured:
- the side reader casting fewer aims once one reads;
- a cheaper first pass in the composite walk;
- the rows trimmed to fewer poses.

## 2026-11-26 — the mesh-budget hour, a first lead

PR 7814's lane (its diff touched only sweep and editor tests) ran `mesh --features budget` locally. Several rows took more than 240 s each, among them `lift_probes` and `quadric_pair_sections`. On PRs 7813 and 7814 the hosted job was still running past an hour when their merges cancelled the run. None of this is profiled. It may predate this sitting's spline arms, so it needs a main baseline before blame.
