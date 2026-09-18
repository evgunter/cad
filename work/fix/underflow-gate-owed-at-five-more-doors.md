---
id: underflow-gate-owed-at-five-more-doors
kind: issue
title: five more decide-then-normalize doors owe the underflow gate, and one of them renders an underflowed component as 0
status: closed
branch: fix/arc-fillet-underflow
pr: 2415
opened: 2026-09-11
closed: 2026-09-12
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

## Closed (branch `fix/arc-fillet-underflow`, PR 2415)

**The fourth arm landed and nothing remains.** `arc_fillet.rs`'s
`carrier_tangent` asks `is_underflowed_length` after
`is_finite_length` and before the sign decision, against
`v.norm_witness()`, and refuses the new
`PathError::UnderflowedDirection { dx, dy }` — a variant, a
`PathErrorKind` twin, a `Display`, and the `underflowed_direction`
tag at the Python door, on the `NonFiniteDirection` shape PR 2401
named.

**The red-first row was executed, not argued, and against the
gate-less tree**: it was pushed alone as this branch's first commit
and run on hosted CI (run 34669963687, GREEN on `9dd5d438d`), pinning
that an arrival carrier anchored `1e-200` from its centre refused
`DegenerateArcCenter { radius: 0.0 }` with the sentence "the authored
centre is within tolerance of an endpoint (radius 0 m)" —
**bit-identical, payload and prose, to the row three lines above it**
that authors the centre AS the anchor. The second commit flipped that
row onto the new arm. It runs through the public door
(`Open.at(..).toward(..).fillet_arc(r, Center { .. })`), so it is
evidence about the library rather than about a private helper.

**The class is closed at all six `is_finite_length` call sites**,
re-derived at this merge base rather than carried from the table
above: `topo::query` (PR 2359), `geom-core`'s `frame`,
`sweep`'s `revolve::axis` and `topo`'s `sector_shape` (PR 2401),
`profile`'s `arc_fillet::carrier_tangent` (here), and
`profile`'s `unit_from_components` — **declined, and now PINNED as
declined** rather than merely unmentioned: its pair is spelled by the
caller, so `ZeroDirection`'s sentence is true of an underflowed pair
and its recourse is already the one that works. A lane adding an arm
there for symmetry breaks
`the_two_director_doors_split_at_the_underflow_end`.

`is_underflowed_length`'s hand-kept "which doors ask it" roster names
the new door and says why the fifth is declined, so the two answers
are in one place.

Residue disclosed by this unit, filed rather than left in prose:
`arc-carrier-refusal-register-misses-two-format-arms`.
