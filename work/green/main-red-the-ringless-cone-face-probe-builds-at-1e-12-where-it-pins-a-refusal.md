---
id: main-red-the-ringless-cone-face-probe-builds-at-1e-12-where-it-pins-a-refusal
kind: issue
title: Main red: mesh reach_probes::a_ringless_cone_face_near_its_apex_meshes_sound builds T1's box at s = 0.03 at eps 1e-12, where it pins CurvedPierceUnsupported
status: open
opened: 2026-11-20
priority: P1
cost: E
---


Found by ONEROUND (2026-11-20) when PR 7143's last CI run (44829682803, `test (app)`, the ε = 1e-12 step) went red on a row its diff does not reach.

## What

`mesh::all reach_probes::a_ringless_cone_face_near_its_apex_meshes_sound` at `CAD_TOLERANCE_EPS=1e-12`:

    panicked at crates/mesh/tests/reach_probes.rs: T1's box at s = 0.03, A ∩ B: builds at ε 1e-12, where it refused CurvedPierceUnsupported

The row pins `refused_at(&[(1e-12, "CurvedPierceUnsupported")])`. The boolean now builds the pose at that ε.

## It is main's

Reproduced by PR 7143's lane on a clean worktree of main `6821ff4a` with its own target directory, the same message:

    CAD_TOLERANCE_EPS=1e-12 cargo nextest run -p mesh --all-features -E 'test(a_ringless_cone_face_near_its_apex_meshes_sound)'

The row was green on a PR run whose main was `c8c00a2a` (run 44813757075), and the test file did not change in `c8c00a2a..6821ff4a`. The first-parent merges in that range that touch kernel code: PR 7184 (`germ/line-cone-one-rule-set`), PR 7187 (`sect/rimside-retires`), PR 7146 (`oneround/section-area`), PR 7159 (`scalar/told-quantity-type`), PR 7195 and PR 7196 (tesslie). Not bisected when filed; ONEROUND has a lane bisecting it, since PR 7146 is one of the six.

On PR 7143's branch the whole of `mesh` and `editor-core` at ε = 1e-12 (`ci` profile, `--no-fail-fast`) is 3525 of 3526: this row only.

## Owed

Whether the body the boolean now builds is right. If it is, the row's refusal pin goes and the build is checked as its other ε rows are (`Got::Meshed`); if not, the merge that moved it has a defect. The lane that caused the red owes the fix. `work/tesslie/a-cone-face-near-its-apex-meshes-an-edge-of-four-triangles.md` (P0, in flight on `tesslie/four-use-edge`) is about the same pose at the other ε rows.
