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

## 2026-11-21: bisected to PR 7184 (GERM), and the new build is right

Found by ONEROUND's lane on `oneround/cone-probe-red`, one target directory, the row's command above. Still red on main `bf3ed771`; neither `crates/mesh/tests/reach_probes.rs` nor `crates/geom-brep/src/curve_face/line_quadric.rs` changed between `6821ff4a` and `49501f13`.

**The merge.** `4d2d9942`, PR 7184 (`germ/line-cone-one-rule-set`). Its first parent `706f6438` is green (1 passed), the merge is red (the message above); the PR 7195 merge `43b37389` below it is green too. Not PR 7146.

**The hunk.** `line_cone_roots` in `crates/geom-brep/src/curve_face/line_quadric.rs`, the `line_quadric_cone_root_slack` rung. Before the PR the door built its quadratic about the edge's start (`About::Start`) and charged each root `over_root_slope(speed · rounding_charge((|w0| + speed·|t|)²), disc)`, `w0` the start's offset from the apex. The PR builds it about the point of the run nearest the apex (`line.param_within(apex, run)`) and charges `Centred::slack_at(t − shift, disc)`, whose terms are the root's distance from that point. A slack past the band is `Unsettled::Meter`, which `reduce.rs`' curved-pierce arm refuses at its frontier, `CurvedPierceUnsupported`.

The pose's deciding read is the box's edge `EdgeKey(5v1)` (a line, run `(0, 6)`) against the cone face `FaceKey(1v1)`: the refusal on the parent was `CurvedPierceUnsupported { operand: B, face: FaceKey(1v1), edge: EdgeKey(5v1), band: Band { zero: 1e-12, escalate: 1e-11 } }`. Printed from inside the door, the same roots on each side of the merge:

    parent 706f6438: t = 3.0212132034355763, slack = 2.02e-12, |w0| = 3.0004, disc = 2.53e-4   (past the 1e-12 band: Meter)
    merge  4d2d9942: t = 2.9787867965644037 and 3.0212132034355963, shift = 3.0, slack = 2.84e-14 each   (placed)

The roots sit 0.021 either side of the apex-nearest point and 3 from the edge's start, so the old charge was that of a root 3 away and the new one that of a root 0.021 away. The roots themselves moved by 2·10⁻¹⁴.

**The build is right, by the row's own check.** With the 1e-12 pin taken off locally (`refused_at(&[])`), the row passes at ε = 1e-12: A ∩ B and B ∩ A are `Got::Meshed` at δ = 5·10⁻³ (`check_mesh` passes and the mesh is the body's), as at the other two ε rows:

    PASS [   0.282s] mesh::all reach_probes::a_ringless_cone_face_near_its_apex_meshes_sound

**A second red, same cause, in the slow set.** `reach_probes::ringed_curved_faces_that_still_refuse_answer_by_cause` (nightly) pins the same refusal at ε = 1e-12 for `t1_toward_the_apex` at `s = 0.03`, `0.01` and `10⁻³`, and all three now build: "T1's box at s = 0.03, A ∪ B: builds at ε 1e-12, where it refused CurvedPierceUnsupported". With the three 1e-12 pins taken off it passes at ε = 1e-12 (4.1 s): `s = 0.03` and `0.01` are `Got::Meshed` at δ = 2·10⁻², 5·10⁻³ and 10⁻³ for A ∪ B, B ∪ A and A ∖ B, and `s = 10⁻³` is `Got::Unread(Cone, "undecided")`, each what the default row answers. Both rows still pass at the default ε and at 10⁻⁶ with those pins off (the 10⁻⁶ pins, `Escalated` at `s = 0.01` and `CurvedPierceUnsupported` at `10⁻³`, still hold).

**The fix, owed by GERM** (PR 7184's program; the test file is in CIW, GREEN and TESSLANE territory): in `crates/mesh/tests/reach_probes.rs`, the `(1e-12, …)` entry out of each of the four `refused_at` lists naming it (one in the ringless row, three in the ringed one), and the two doc sentences that state the 10⁻¹² refusal ("at `10⁻¹²` the boolean refuses the pose" and "at `10⁻¹²`, the cone at `s ≤ 0.03`"). No kernel change. Not made here: it is another program's.
