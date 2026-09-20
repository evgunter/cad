---
id: from-face-frame-under-an-analysis-lane-refuses-unpinned
kind: issue
title: A FromFace mate frame under an analysis-lane evaluation refuses Unpinned rather than resolving
status: open
opened: 2026-09-20
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
