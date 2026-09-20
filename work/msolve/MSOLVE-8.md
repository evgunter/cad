---
id: MSOLVE-8
kind: unit
title: A levered clash names its arm as a typed lever, the coset's directions carry the unit witness, and MateFault names its two consumers
status: closed
opened: 2026-09-19
branch: msolve/8-levered-clash
closed: 2026-09-20
pr: 2896
---


Spec: `docs/MSOLVE-8-SPEC.md`. Gathers `plan.md` items 11 and 15:
`levered-clash-margins-hide-their-arm` (the decision the item names,
ruled: a closed `Lever { Roll, Residual }` enum, no unit string),
`subgroup-directions-are-unit-by-prose` (the witness taken at the mate
frame read through `OrthoFrame`, re-minted under the band where the
fold rotates a direction) and `mate-fault-subject-spelled-in-three-
crates` (ruled: no `subject()`; one sentence on the enum). No verdict
moves. Dispatches from main after MSOLVE-7 merges (both touch
`solve.rs`).

## Closed (2026-09-20, PR 2896)

Landed: `Lever { Roll, Residual }` and one closed `Clash { Structural,
Length, Levered(Lever) }` on `MateFault::Contradictory`, so every
levered clash names the number its predicate decided and the arm it
was decided over (a roll the mate's own, a residual the fold's — each
honest, stated at the type), with the second `Display` sentence and
no string compare; `Subgroup`'s directions and `Rotations::About` are
`geom_core::UnitVec3`, `parallel` decides ONCE through the
normalizing constructor under `mate_axes_parallel` and returns the
witness it minted (the planar-pair site takes it; no second
decision), and `invert`'s transport re-mints under the band with the
frame ladder's own vocabulary on refusal; `MateFrame::frame` takes
the `OrthoFrame` from geom-core's new `point_at_frame` sibling —
`point_at` is its `to_affine`, so the two are one construction bit
for bit — with `placement` and `axis` derived from it (the fence
widened by that one door, announced on SCALAR's tracker, which closed
`point-at-drops-the-frame-witness` as done here); one sentence on
`MateFault` names its two consumers and the `Band` /
`PosesOfAnotherDocument` asymmetry, no `subject()`. Reviews on
`f3896c554`: correctness C1/C3/C4 HOLD, C2 PARTIAL with one MAJOR
(a second decision on the planar-pair line, two ulps from
`parallel`'s, refused ten door-built documents under its own name
where `parallel` had said non-parallel; under the one decision above
those pairs, one to two ulps inside K·ε, escalate under
`mate_axes_parallel` itself — the band's own answer — and the row pins
every verdict against the one spelling); style no MAJOR; the twenty-two-item fix pass
landed, with the lane's first deviation (a re-minted axis under
`point_at`'s name, past the spec's stop clause) reversed. Filed here:
`lever-refusal-respells-reach-refusal`, `near-parallel-planes-refuse-
under-a-false-predicate` (pre-existing: a nearly-parallel pair
classifies as a line and then refuses a non-finite translation margin
under the wrong predicate name). Closes the three items. The spec is
deleted into `docs/DOC-LEDGER.md` (recoverable at the unit head named
there).
