---
id: the-sketch-plane-is-its-frame
kind: unit
title: The sketch plane is its frame: SketchPlane holds one OrthoFrame; embed/pin crossings; turned_between for path sweeps; loft takes planes
status: parked
opened: 2026-10-02
priority: P0
cost: M
blocked_on: [store-constructed-carriers]
---


The implementation of the design Ev approved on #3775 (2026-10-02). It closes `sketch-plane-holds-the-affine-and-the-witness-dies-at-the-read-boundary` and `work/round/sketch-plane-lift-choice-has-no-home.md`; the latter's choice is not a real one, because the evaluator already makes it (`ProfileLift`).

The design is in `geom-core`'s `linalg/ortho_frame.rs` and `unit_vec.rs` (their mint lists and "Crossing scalars") and in `profile`'s crate docs on the sketch plane. Both designer reports are in #3775's body.

- **The type.** `SketchPlane<T>` holds one private `OrthoFrame<T>`. The placement is derived (`placement()` = `frame.to_affine()`). `SketchPlane::new(Affine3)` and the public `placement` field go, and `u()`/`v()`/`normal()` return `UnitVec3`s. Ev: deleting `SketchPlane` for `OrthoFrame` directly is equally fine. Keep the newtype, which both designers leaned to, unless the implementation shows the wrapper earns nothing.
- **Crossings.** `OrthoFrame::embed` (f64 → T, exact through `Real::from_f64`) and `OrthoFrame::pin` (lane → f64, refusing on the analysis scalars) are the only crossings. `pin` needs editor-core's `SectionScalar::pinned_f64` fact in geom-core, beside `Real`. `UnitVec3` gets the same two doors. The closure `map`/`try_map` retire.
- **Sweeps.** Extrude, revolve and loft take the plane. Loft's `places` become `&[SketchPlane<f64>]`.
- **Station frames.** `sweep_places` makes them by `OrthoFrame::turned_between(a, b, about, band)` (the Rodrigues-between form `I + [k]× + [k]×²/(1 + a·b)`, `k = a × b`) and `translated`. The one decision is `1 + a·b > 0` under a funnel name, which is a new K site. The tangents become `UnitVec3`s, retiring `skin.rs::unit_tangent`'s `!(n > 0.0)` check. `path_start_frame` returns the `OrthoFrame`.
- **Expected moves.** The half-turn knife edge changes: `review_half_turn_path_builds_on_the_float_knife_edge` flips to the refusal its own message asks for. Sweep rows re-baseline: `turning_orientation`, `m7_skin_integral`, `m8_14_long_turn_sweep`, `review_m5_pr10` (and `_interval`), `s393_start_frame_door`, editor-core's `review_m5_pr10_sweep_node`, step-export's `swept_elbow.expect`, and the teapot/klein/lily/skinned tours. Attribute each by toggle.
- **Out of scope.** geom-brep's `MappedCurve::PlacedSegment.place` stays a bare `Affine3`, per the at-rest rule.
- **Off-question, filed separately if not fixed here.** `editor-core/src/placement.rs` lifts an assembly placement by `affine_f64().map(T::from_f64)`. Tests that build planes from non-orthonormal affines move to geom-core's `decided_corpus`.

Parked on 5b because both change `sweep`'s loft surface.
