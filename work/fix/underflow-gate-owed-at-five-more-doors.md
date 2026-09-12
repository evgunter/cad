---
id: underflow-gate-owed-at-five-more-doors
kind: issue
title: five more decide-then-normalize doors owe the underflow gate, and one of them renders an underflowed component as 0
status: open
branch: fix/underflow-gate-doors
pr: 2401
opened: 2026-09-11
---



Disclosed by the `direction-underflow-reports-zero-length` lane (PR
2359) in its discipline §5 sweep and **not filed at the time**, which
is the failure mode `work/README.md` names: a residue disclosed in a
PR body and nowhere else is invisible to the re-homing sweep. Filed
here by the FIX orchestrator on reading the merged PR.

## The class

`is_underflowed_length(len, witness)` (`geom-core/src/real.rs:699`)
now separates *a direction whose length underflowed out of the format*
from *a direction that is zero*. PR 2359 fitted the gate at exactly one
of the six `is_finite_length` call sites. The other five decide a
length and then normalize it, and each names the underflowed case as
something it is not — with a recourse that cannot work.

Re-verified against this head, not carried over on trust:

| site | today's refusal | why the gate is owed |
|---|---|---|
| `geom-core/src/linalg/frame.rs:332` `definitely_positive` | `FrameError::Degenerate { input: MirrorNormal }`, recourse *"declare the coincidence, move the geometry, or lower the tolerance"* | **executed by the 2359 lane**: lowering the tolerance cannot recover a norm the format lost. Takes `length: T` only, so the witness has to be plumbed from its four call sites — a signature change, not a line |
| `sweep/src/revolve/axis.rs:91` `AxisFrame::build` | `RevolveError::DegenerateAxis`, *"has no definite length (zero or sliver)"*, recourse `COINCIDENCE_RECOURSE` | "sliver" is nearer true than "zero length" was, but the recourse is coincidence, not scale |
| `topo/src/sector_shape.rs:283` | `invalid(band, SECTOR_ARM)` — an `Indeterminate` with `MarginDiag::Invalid` | its arm is `min(norm_own, norm_next)`, so as at the overflow end a `min` hides an underflowed chord behind a good one; the witness question is **per chord**, not per arm |
| `profile/src/path/arc_fillet.rs:915` `carrier_tangent` | `PathError::DegenerateArcCenter { radius: 0.0 }` | **executed by the 2359 lane**: a different arm entirely, reached because the radius underflowed first. The zero-naming is in a PAYLOAD FIELD, not in prose — which is why no prose grep could see it |
| `profile/src/path.rs:2907` `unit_from_components` | `PathError::ZeroDirection { dx, dy }`, *"whose norm is within tolerance of zero"* | the **weakest case for a new arm**: that sentence is true of an underflowed direction and its recourse (*"scaling them up costs nothing"*) is already the right one. What is wrong here is the RENDERING, and that is a separate defect with its own file — `path-error-numbers-below-1e-9-render-as-zero` |

## Scope, and what a taker owes

Four arms, not five: `unit_from_components` may well want no new arm
at all once its formatter is fixed, and a taker should say which way
it went rather than adding an arm for symmetry.

**This is not one line per door.** Two of the four (`frame.rs`,
`sector_shape.rs`) need the witness plumbed through a signature or
asked per chord, so the cut is by fence rather than in one sitting:
`geom-core` + `sweep` are PROPS's ground, `topo/src/sector_shape.rs`
is unowned, `arc_fillet.rs` is `profile`.

**Read `is_underflowed_length`'s contract before calling it.** The
`witness` argument must be the largest `|component|` of the vector the
norm was taken of; nothing enforces that, and passing anything else
makes the answer meaningless. PR 2359 registered that as a place it
was unsure. A taker who fits the gate at four more doors is the second
consumer that decides whether the predicate should be
vector-shaped instead — which would need a `Vec2` twin for the two
`profile` doors.

**The gate is a point-scalar gate.** At `Interval` it answers `false`
by construction (`witness / len` is an unbounded enclosure, not
poison), and PR 2359 pinned that in two rows deliberately: an
enclosure still contains the true length and nothing underflowed out
of the format. Any new door gets the same posture, or argues against
it at the site.

## Taken: three of the four arms (branch `fix/underflow-gate-doors`)

**Landed.** `frame.rs`'s `definitely_positive` (the witness plumbed
through its signature, all four call sites), `revolve::axis::AxisFrame::build`,
and `sector_shape` (per CHORD, before the `min`). Each got a new typed
arm beside its existing non-finite one, its own rendering, and a
red-first row that was executed against the gate-less tree rather than
argued. The witness derivation moved into `Vec2::norm_witness` /
`Vec3::norm_witness`, and `decide_unit_direction` was re-spelled onto
it so the workspace has one derivation.

**Not taken: `profile/src/path/arc_fillet.rs:915` `carrier_tangent`.**
The arm it needs is `PathError::UnderflowedDirection { dx, dy }` with a
`PathErrorKind` twin and a `Display` — and `PathError`,
`PathErrorKind` and that `Display` all live in
`crates/profile/src/path.rs:782`, `:1273`, `:1660`, which a concurrent
lane held for the whole of this unit's life. The door itself is one
`if` in `arc_fillet.rs`; the variant it refuses with is three edits in
a file this branch could not touch. Everything else it needs is in
place: the witness is `v.norm_witness()` on the `Vec2` it already
binds, and the sibling overflow arm
(`PathError::NonFiniteDirection { dx, dy }`) is the shape to copy.
Nothing pins the current refusal (`DegenerateArcCenter { radius: 0.0 }`
at an anchor ~1e-200 from the centre), so the red-first row is owed
with it.

`unit_from_components` remains the fifth door and remains declined:
its sentence and its recourse are already right, and its rendering is
`path-error-numbers-below-1e-9-render-as-zero`, not this item.
