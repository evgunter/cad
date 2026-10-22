---
id: exact-frame-mints-cover-three-of-the-world-frames
kind: issue
title: The frame witness's decision-free mints cover three world frames, so every other exact frame costs a K funnel name
status: closed
opened: 2026-09-15
priority: P1
cost: M
branch: frame/exact-frames-are-turns
closed: 2026-10-21
pr: 5573
---


## What

Disclosed by FRAME-WITNESS (`docs/FRAME-WITNESS-SPEC.md`), which built
`geom_core::RigidMotion` and its mints.

`RigidMotion`'s decision-free mints are `axes_xy`, `axes_yz` and
`axes_zx` — the three CYCLIC world frames, which is what
`profile::SketchPlane`'s three sugar doors needed. Every other frame
whose axes are exact basis vectors has to go through
`RigidMotion::gram_schmidt`, which is a DECISION and therefore wants a
band and a K funnel name the caller has to invent. On an exactly
orthonormal pair that mint changes no bit (pinned by
`gram_schmidt_keeps_an_exact_pair_bit_for_bit` in
`crates/geom-core/src/linalg/rigid_motion.rs`), so what the caller pays
is not arithmetic — it is a name in the K roster and a refusal path
that cannot fire.

The receipts are the two helpers the unit had to write:

- `demos/tour/src/scalar.rs` `sketch_frame` / `authored_frame`, under
  the invented site `tour_frame_axis` — the tour's `(x̂, ẑ)` planes
  (`klein.rs` ×3, `lily.rs` ×2) are exact and could not use a mint;
- `crates/sweep/src/test_support.rs` `sketch_from_axes`, under
  `fixture_frame_axis`, for the same reason across the fixtures.

`demos/tour` IS in the k-lint sweep's corpus (`scripts/k_probe_sweep.sh`'s
demo-scenes leg), so `tour_frame_axis` is a roster name a demo minted
because the library had no decision-free door for the frame it meant.

## The two shapes a fix could take

1. **Widen the exact mints** to the signed basis frames — an axis pair
   drawn from `±x̂, ±ŷ, ±ẑ`, refusing the parallel pairs at compile
   time by taking an axis-and-sign enum rather than a vector. Twenty-four
   frames, no decision, no name.
2. **Leave it, and say so on the type** — the doc currently says the
   three are "the only frames the type admits without a decision"
   without saying why the other twenty-one are not there.

Either wants PROPS' call, since the type is `geom-core/src/linalg`'s.

## Not the same as a "trust me" mint

Both shapes above stay inside the ruling
(SCALAR's `unit-vector-invariants-carried-as-prose` (ruled on PR 2457; `docs/doc-ledger/scalar-leaves-the-tracker.md`) §RATIFIED):
they add EXACT frames whose axes are literal basis vectors, never a
constructor that accepts a pair a caller believes is orthonormal.

## Part answered by SYM-17 (2026-10-13)

`RigidMotion` (PR #4197, which replaced `OrthoFrame` with it) adds
three more decision-free mints, the exact half-turns `half_turn_x`,
`half_turn_y` and `half_turn_z`, and makes `UnitVec3::exact_x`,
`exact_y` and `exact_z` public, so a caller can mint
`RigidMotion::rotation(UnitVec3::exact_z(), angle)` with no name and
no band. The signed basis frames as frames (shape 1) are still not
there: a product of the exact mints reaches them
(`RigidMotion::compose`, not decided again), but there is no door
that names one, and `sketch_frame` / `sketch_from_axes` still decide.

## Decided (2026-10-21): the written turn is the vocabulary

FRAME orchestrator, on a designer pair that converged in round 1 (no ratified text moves, so no `[ev]` PR). No signed-basis mint family: every axis-aligned frame is already reachable with no decision as an exact axis (`UnitVec3::exact_x/y/z`) turned by a written fraction of a turn (`RigidMotion::rotation_by` over an `Angle` with `Turns`, whose sine and cosine are exact) and composed (`compose` decides nothing again) — the same vocabulary D10 `[placement.values]` gives a placement's values and the document already uses (`mate::value::unmated_motion`, `eval::wire::station_plane`). A 24-frame enum would be a second way to say it, and two `SignedAxis` arguments could still name a parallel pair. Corrections to this row's premise: `fixture_frame_axis` is not a roster name (INSTR's ruling on PR 2733); the quoted "only frames the type admits" sentence is no longer in `rigid_motion.rs`; and of the tour's `sketch_frame` sites only the exact pairs move, the computed ones are real decisions and keep `tour_frame_axis`.

What is owed (unit `frame/exact-frames-are-turns`): an `Angle` door from a `Turns` alone (today `Angle::of_turns(radians, turns)` takes the one fact twice); the exact-pair receipts rewritten as turns (the tour's Klein and lily planes, lily's re-decision of a witness's own axes as a `compose`, and `sweep::test_support::sketch_from_axes`'s axis-aligned callers onto the exact doors); the docs that say otherwise (`TOUR_FRAME_AXIS`, `sketch_from_axes`, `rigid_motion.rs`'s list of mints).

## Closed

2026-10-21, PR 5573. The written turn is the vocabulary:
- `Angle::from_turns` derives the radians from the fraction.
- `Turns::quarters(n)` cannot refuse.
- `RigidMotion::turned(origin, axis, turns)` is infallible, with `rotation_by`'s linear bits. A quarter turn about x̂ is pinned bit for bit to the frame `gram_schmidt` decided from `(x̂, ẑ)`, and products of quarter turns reach all 24 signed-basis frames.
- The tour's exact planes and 30 of 58 `sketch_from_axes` fixtures moved onto it. Computed pairs keep their decision and their K name.

Residue filed:
- `exact-axis-and-reference-frames-still-decide` (the aim-mint class, about 50 calls)
- `a-frame-turned-by-a-radian-angle-is-spelled-as-its-columns`
- `turns-has-two-answers-to-an-integer-past-the-mantissa`
- `profile/sketch-plane-places-only-the-xy-world-plane-at-an-origin`
- `rigid-motion-radian-rotation-doors-twin-their-angle-doors`
- `python-sketch-plane-from-frame-decides-exact-axes` (parked on D′)
