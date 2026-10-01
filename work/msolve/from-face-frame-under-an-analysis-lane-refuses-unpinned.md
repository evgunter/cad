---
id: from-face-frame-under-an-analysis-lane-refuses-unpinned
kind: issue
title: A FromFace mate frame under an analysis-lane evaluation refuses Unpinned rather than resolving
status: open
opened: 2026-09-20
priority: P1
cost: H
---

Found by the MSOLVE-9 lane while wiring `MateReach::face_pose`
(`crates/editor-core/src/eval/mod.rs`, `face_pose_over_cache`).

## The shape of it

A `FromFace` mate frame resolves from the mated part's cached product,
whose scalar is the evaluation's (`PartCache<T>`). The solve works
over `f64` frames, so the pose crosses as `f64` through
`SectionScalar::pinned_f64` — the value at `f64`, `None` on every
analysis scalar (an enclosure, a sensitivity, a symbolic lane), by
that trait's own rule that a bracket has no single `f64` that is not a
fabricated choice. So under an analysis-lane evaluation a `FromFace`
side refuses `FacePoseRefusal::Unpinned`, wrapped as
`MateFault::FaceUnresolved`, and the mate faults — where an `Authored`
side of the same mate solves (its numbers are the document's, not the
product's), and where the lever's reach answers the bracket's `hi`
(an upper bound is a safe direction; a frame has none).

## What is missing

A lane-aware resolution: a rule for which `f64` a bracketed face pose
crosses to the solve as — the nominal lane's product evaluated beside
the analysis one, or the pinned lift's embedding read back before the
arithmetic widens it — ratified as the analysis lanes' own decision
(`SectionScalar`'s doc is where the present rule is stated). Until
then the refusal is typed at the mate, the side, the instance and the
part, and `undecided` treats it as no verdict (the maintenance refuses
rather than records a frame).

## Not this unit

MSOLVE-9's fence took the readback as it is and the evaluator's lanes
as they are; the spec's stop clause did not fire (the product is in
the part's own coordinates, at the lane's scalar), so the arm was
landed typed and this residue filed rather than argued past.

## The doors it blocks (MSOLVE-9 fix pass, 2026-09-24)

The refusal is honest and stays: a face pose follows the part's
parameters, so reading a `Dual64`'s real part would drop the pose's own
sensitivity, and pinning an `Interval` at the nominal would certify a
clearance for a face that moves inside the box. What it costs is two
doors, on every assembly holding a face frame:

- **`stackup::sensitivities`** (`crates/editor-core/src/stackup.rs`) —
  its `Dual64` passes fault the mate `Unpinned`, so no ∂m/∂p crosses a
  mate that names a face.
- **certified `clearance`** (`crates/editor-core/src/clearance.rs`) —
  its `Interval` leaf does the same.

Authored vectors still solve on both lanes. The viewer's mate tool now
authors only face frames, so every GUI-authored assembly loses both
until this row lands. Pinned by
`msolve9_from_face::a_face_frame_under_a_dual_evaluation_refuses_unpinned`;
disclosed at `MateFrame`'s doc, the `.pyi`'s `MateFrame`/`from_face`
and `docs/guide/assembly.md`.

## The design it needs

The solve deciding over the lane's own scalar: frames, the witness
ladder and the coset table carried at `T` rather than `f64`, so a face
pose crosses with its tangent or its enclosure intact and the solved
placement carries them onward. Not a choice of which `f64` to read —
every such choice is one of the two losses above.
