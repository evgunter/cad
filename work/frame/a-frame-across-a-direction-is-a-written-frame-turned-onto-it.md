---
id: a-frame-across-a-direction-is-a-written-frame-turned-onto-it
kind: unit
title: A frame across a direction is a written frame turned onto it by the least turn; path_start_frame and its ladder retire
status: closed
opened: 2026-11-03
priority: P1
cost: M
branch: frame/turned-onto
closed: 2026-11-03
pr: 6465
---


## Decision

Two designers weighed `path-start-frame-is-the-only-frame-from-a-tangent-and-is-named-for-the-start` and `two-doors-answer-an-axis-off-a-tangent-with-no-rule-for-which` together, and converged after one round. The labels are on `analysis/design-fork/frame-tangent-doors`. The orchestrator decides it, because no ratified text moves. The ladder (world +Z, then +X) has no ratifying commit. D10 already gives the unauthored roll across a direction as the LEAST TURN: `[placement.values]`' spin, and the bare-disc sketch frame Ev ruled on PR 6298. So the ladder is a second rule for one question (`[one-way]`). It also flips the profile by a half turn across its pole band.

## Spec

1. **`RigidMotion::turned_onto(self, direction: UnitVec3, site, band) -> Result<Self, TurnError>`** in `crates/geom-core/src/linalg/rigid_motion.rs`. It is defined as `turned_between(self.w, direction, self.origin) ∘ self`: a frame the author WROTE, turned about its own origin by the least rotation carrying its `w` onto `direction`. Its only decision is `turned_between`'s, and it refuses across the closed band at the pole, exact pole included, as `turned_between` does. The recourse at the pole is to write a different frame (e.g. `RigidMotion::turned(p, x̂, Turns::halves(1))`). The door never chooses a half turn itself. A sweep's start is spelled `RigidMotion::axes_xy(p).turned_onto(t̂)`.
2. **`path_start_frame` retires**, along with its ladder and the `FrameError` vocabulary that only it uses (`FrameInput::{Tangent, ReferenceLadder}`, `FrameVector::Tangent`, if nothing else reads them). Its callers move to `turned_onto` on the written frame they mean: the tour (`skinned`, `klein`, `projectbox`), `crates/pncad/examples/sweep_start_frame_and_roll.rs`, `crates/sweep/tests/common/mod.rs` (`normal_start_place`) and the sweep tests that import it. Python's `Frame.path_start_frame` in `crates/pncad-py/src/py/place.rs` goes, together with its `.pyi`, its tests and the ty fixtures. No Python call takes a `Frame` any more.
3. **Interior stations read `sweep::sweep_places`.** That covers `demos/tour/src/skinned.rs` `tube_place` and §3 of the façade example: the start frame carried by the rotation-minimizing frame, with the authored roll composed on top. `loft_body`'s doc (`crates/sweep/src/loft.rs`) stops sending loft callers to a per-point door.
4. **One "which door" paragraph** in `linalg::frame`'s module docs, where its constructor list stands:
   - a stated reference: `point_at`, `from_axis_and_reference`;
   - no reference: a written frame `turned_onto` the direction;
   - along a path: `sweep_places`;
   - `Vec3::orthonormal_basis`: scratch and input coordinates only, never a kept frame.
   
   Fix `orthonormal_basis`'s doc in `vec.rs`, which still tells a consumer to store its output as `u_ref`, against the D10 gate. Update `docs/LIBRARY-DESIGN.md` U4's wording ("a kernel door for the path-start frame") to name the door that now answers it. That is a re-wording forced by this change, not a new decision.
5. **Pins.** `editor-core/tests/scalar_frame_r1_probes.rs` pins the ladder bit for bit against a retired recipe, so it retires. `sweep/tests/s393_start_frame_door.rs` drops the ladder's cone differential and keeps its "identity placement is not an alternative" row. New tests:
   - `turned_onto` is the identity at `w = direction`;
   - it is continuous through the tangent `(k·ε, 0, 1)` that the ladder flipped on;
   - it refuses at `−w` and in the band around it.
6. **Renders.** Tour sweeps with an asymmetric section re-roll. Push a `[render]` commit, look at the result, and say in the PR body what moved.

Sweep for the class: any other site that picks a roll off a world axis by a rule of its own, e.g. `geom/src/curves/on_carrier/rows.rs` and `topo/src/census/curved.rs` (unverified; decide test-only vs production, and file whatever is production). Report the hit list.

Out of scope, filed: the least turn's two arithmetics (`msolve/the-least-turn-has-two-arithmetics-and-the-chart-decides-its-pole-a-third-way`). STRUT's `sweep-body-makes-every-caller-derive-its-start-frame` may build sugar on `turned_onto` later; this unit does not.

Review tier: **single FULL**. The door is a one-line composition, but the unit retires a public door, moves its callers and re-baselines renders. The claims to falsify are spec items 1, 3 and 5.

## Closed

Landed by PR 6465 (2026-11-03): `RigidMotion::turned_onto`, the retirement of `path_start_frame` and its ladder, interior stations on `sweep_places`, and the "which door" paragraph in `linalg::frame`.
