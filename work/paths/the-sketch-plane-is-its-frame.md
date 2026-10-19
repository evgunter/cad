---
id: the-sketch-plane-is-its-frame
kind: unit
title: The sketch plane is its frame: SketchPlane holds one RigidMotion; embed/pin crossings; turned_between for path sweeps; loft takes planes
status: closed
opened: 2026-10-02
priority: P0
cost: M
needs_ev: true
branch: paths/sketch-plane-is-its-frame
pr: 5068
closed: 2026-10-14
---


The implementation of the design Ev approved on #3775 (2026-10-02). It closes `sketch-plane-holds-the-affine-and-the-witness-dies-at-the-read-boundary` and `work/round/sketch-plane-lift-choice-has-no-home.md`; the latter's choice is not a real one, because the evaluator already makes it (`ProfileLift`).

The design is in `geom-core`'s `linalg/rigid_motion.rs` and `unit_vec.rs` (their mint lists and "Crossing scalars") and in `profile`'s crate docs on the sketch plane. Both designer reports are in #3775's body.

- **The type.** `SketchPlane<T>` holds one private `RigidMotion<T>`. The placement is derived (`placement()` = `frame.to_affine()`). `SketchPlane::new(Affine3)` and the public `placement` field go, and `u()`/`v()`/`normal()` return `UnitVec3`s. Ev: deleting `SketchPlane` for `RigidMotion` directly is equally fine. Keep the newtype, which both designers leaned to, unless the implementation shows the wrapper earns nothing.
- **Crossings.** `RigidMotion::embed` (f64 → T, exact through `Real::from_f64`; `embed_computed` through `Real::from_computed` for a frame the kernel computed) and `RigidMotion::point_value` (lane → f64, `None` where a component has no single finite `f64`) are the only crossings; `UnitVec3` carries each axis through crate-private halves of them. `pin` (refusing on the analysis scalars) is not built: its consumer, `SectionScalar::pinned_f64`, is gone (the 2026-10-11 EDIT), and the lane → f64 reads that remain are value-channel reads of a placement at the lane scalar, which `point_value` is. The closure `map`/`try_map` retire.
- **Sweeps.** Extrude, revolve and loft take the plane. Loft's `places` become `&[SketchPlane<f64>]`.
- **Station frames.** `sweep_places` makes them by `RigidMotion::turned_between(a, b, about, band)` (the Rodrigues-between form `I + [k]× + [k]×²/(1 + a·b)`, `k = a × b`) and `translated`. The one decision is `1 + a·b > 0` under a funnel name, which is a new K site. The tangents become `UnitVec3`s, retiring `skin.rs::unit_tangent`'s `!(n > 0.0)` check. `path_start_frame` returns the `RigidMotion`.
- **Expected moves.** The half-turn knife edge changes: `review_half_turn_path_builds_on_the_float_knife_edge` flips to the refusal its own message asks for. Sweep rows re-baseline: `turning_orientation`, `m7_skin_integral`, `m8_14_long_turn_sweep`, `review_m5_pr10` (and `_interval`), `s393_start_frame_door`, editor-core's `review_m5_pr10_sweep_node`, step-export's `swept_elbow.expect`, and the teapot/klein/lily/skinned tours. Attribute each by toggle.
- **Out of scope.** geom-brep's `MappedCurve::PlacedSegment.place` stays a bare `Affine3`, per the at-rest rule.
- **Off-question, filed separately if not fixed here.** `editor-core/src/placement.rs` lifts an assembly placement by `affine_f64().map(T::from_f64)`. Tests that build planes from non-orthonormal affines move to geom-core's `decided_corpus`.

Parked on 5b because both change `sweep`'s loft surface.

## Released from the D10 hold (2026-10-08)

Nothing D10 changes gates this row, so it is open: a kernel witness refactor in profile/sweep/geom-core (SketchPlane holds a RigidMotion); stage 3's Frame variables lower onto this witness rather than replace it. (INTENT's re-homing of the parked rows, `work/intent/log.md`.)

## `SectionScalar` is gone (EDIT, 2026-10-11, PR 4741)

The Crossings bullet's `SectionScalar::pinned_f64` no longer exists.
A loft now reads each section's plane at the lane scalar from its
evaluated profile value (`sweep::loft_profiles`), so editor-core
pins no section plane to `f64`. `pinned_plane`, `pin`'s would-be
editor-core consumer, was deleted, and `loft_body`'s placements are
in `T` since PR 4726. Whether `pin` is still owed needs a consumer
named again before this unit builds it.


## Two deciders SYM-17 left for this row (2026-10-13)

SYM-17 (PR #4197) made `RigidMotion` the rigid-motion witness, with
mints by construction. Two of its sites decide a direction only because
the sketch placement is still a general `Affine3`; this row's frame
turns both into constructions:
- **The revolve's placed axis.** `sweep/src/revolve/axis.rs`'s
  `AxisFrame::build` decides `place.linear · (dir_sk, 0)` into a
  `UnitVec3` under `revolve_placed_axis` (refusal
  `RevolveError::PlacedAxis`) before the revolve turns about it. With
  the plane a `RigidMotion`, it is `frame.turn` of the lifted sketch
  direction, unit by construction, and the decision and its K name go.
- **`path_start_frame` / `point_at`** return the witness's
  `to_affine()`; with the plane a frame they return the witness.

## Outcome

Built as the design states, with these deviations (the PR body is the
decision record):

- `SketchPlane<T>` keeps the newtype over one private `RigidMotion<T>`;
  `new`, the `placement` field, `map` and `try_map` are gone;
  `placement()` is derived; `u()`/`v()`/`normal()` are `UnitVec3`s;
  `xy_at` is the sugar the demos and fixtures wanted.
- The crossings, as Ev ruled on #5068 (answer A, 2026-10-14): a frame
  crosses no scalar upward, and `point_value` is the one crossing.
  `embed`, `embed_computed`, `Profile::embed` (the rename of
  `Profile::map_scalar`) and `UnitVec3`'s crate-private halves are
  gone; `pin` is not built. The demos mint their frames at their
  scalar from inputs entered through `S::from_computed`; fixtures mint
  at `T` through the world mints or `check_rigid`.
- `ProfileLift` retired with the ruling: every evaluation elaborates
  profile geometry at its scalar, guided by the `f64` record. The
  `f64` lane over the nominal takes the first pass's validated form
  (`editor_core::ProfileLane`, sealed, per scalar; held bit for bit
  against the second pass run at `f64` by
  `m10_p_lift::the_f64_lane_is_the_second_pass_bit_for_bit`), and
  logs the first pass's decisions again in the second pass's place
  (`k_stats::relog`), so its verdict log is the one a second pass
  leaves and compares with any lane's. INLANE's `the-pinned-profile-lift-launders-at-sym` is
  narrowed to the loft's enclosure arm, `lift_onto`'s last production
  consumer.
- `RigidMotion::turned_between` (one decision, `1 + a·b > 0`, ledger
  row F22, site `sweep_path_turn`; built as the quaternion
  `(1 + a·b, a × b)`'s matrix rather than the Rodrigues-between form,
  which was not rigid within the band near opposed) and `translated` mint the station
  frames; the station tangents are `UnitVec3`s under
  `sweep_path_tangent`; `unit_tangent` is gone. `path_start_frame` and
  `point_at` return the `RigidMotion`. `revolve_placed_axis` and
  `RevolveError::PlacedAxis` retire (the revolve turns the sketch
  axis's witness by the frame).
- Loft's `places`, `loft_parameters`, `loft_geometry`, `sweep_body`,
  `sweep_geometry` and `sweep_places` take `SketchPlane`s.
- Moved baselines: `swept_elbow` (mesh digests, STEP fixture, kernel
  sidecar), attributed to `sweep_places` by toggle; the half-turn row
  flips to the refusal; the klein scene's loop now lofts through stated
  normal-plane frames because `sweep_body` refuses its half turn. The
  other rows this file predicted would move (`turning_orientation`,
  `m7_skin_integral`, `m8_14_long_turn_sweep`, `review_m5_pr10*`,
  `s393_start_frame_door`, `review_m5_pr10_sweep_node`) pass unchanged:
  they assert within tolerances the rounding-level moves do not reach.
- The off-question (`editor-core`'s `Frame::affine`) is filed on WIRE:
  `work/wire/a-literal-placement-frame-crosses-scalars-by-closure-and-is-decided-again-at-the-lane.md`.
