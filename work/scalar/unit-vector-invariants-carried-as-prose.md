---
id: unit-vector-invariants-carried-as-prose
kind: issue
title: Investigate — unit-vector invariants carried as prose across function boundaries
status: open
opened: 2026-08-13
github: 457
refs: [447]
---

## From GitHub issue 457

Opened 2026-08-13; 0 comments.

Spun out of the codebase scan that followed #447. Item 7 of that sweep — flagged deliberately as *investigate*, not *do*, because unlike the rest of the sweep it is not mechanical.

## The observation

#447's move was to take an invariant established once, and make it structural: `Span` is a span index proven in range and nonempty, carrying the control window it selects, so "invalid span index" stops being a representable state and the guards at the use sites disappear.

Two places carry a *value*-level invariant the same way #447's span index used to — established somewhere, then relied on across a function boundary as prose:

- `crates/geom-core/src/linalg/frame.rs:238`, `frame_from_unit_aim` — takes an `aim` documented as **unit**, unchecked. The recipe's right-handedness and the `y = aim.cross(x)` normalization both depend on it.
- `crates/profile/src/lib.rs:353`, `Plane::from_frame` — "normal = u × v (computed, keeping the frame right-handed by construction when u ⊥ v are unit — **which is the caller's conventional obligation, unchecked**)."

Structurally identical to the `span − degree` situation: a fact proven at one site, consumed at another, with only a comment connecting them. The analogous fix is a `UnitVec3<T>` minted once by the ladder that already computes the norm — `frame_from_unit_aim` notably already receives `cross_len` precisely so the caller's decision is not recomputed, which is the same "compute once, carry it" instinct #447 applied to `index − degree`.

## Why this is a design question and not a follow-up PR

Span validity is **structure** — checkable from the knot vector without looking at any parameter value, which is exactly why `Span` could be a total, cheap, `Copy` newtype with no policy in it. Unitness is a **value** property of a `T: Real`, and this kernel deliberately routes value decisions through `decide`/`Margin`/`Band` rather than making them type-level, so that a near-degenerate input gets a *policy* answer (definite, or an honest refusal) instead of a silent one. Questions a design pass has to answer first:

1. What does `UnitVec3::new` do at the boundary — refuse via `FrameError`, abstain, or accept a band? Who owns the band?
2. Does it type-check unitness at all, or is it a witness type minted only by `normalize()` (proof-by-construction, no runtime decision)? The second is much closer to `Span` and much cheaper — it does not add a decision, it records that one already happened.
3. What happens under the generic scalars, where "unit" is not a predicate that answers cleanly — `Interval`, `Dual`, `Probe`? `Span` sidestepped this entirely by being pure structure. A unit-vector type does not get to.
4. Does it pay for itself, given the consumer count is currently two?

Question 2 is the crux: a witness type that carries no decision is a genuine `Span` analogue, while anything that validates on construction is a new policy surface and needs Ev's sign-off per the working-style convention.

## Not urgent

No known bug behind this — both call sites' current callers do normalize. Filing so the observation is not lost; the rest of the #447 sweep (the span/window items) is mechanical and is being done separately.

## Home

`work/issues/`: `geom-core/src/linalg/frame.rs` and `profile/src/lib.rs` are not covered by any open program's `paths`, and no open charter names the scalar-newtype question.

## Re-homed to SCALAR (2026-09-11, the cut in `docs/WORK-TRACKS-2026-09.md` addendum 3)

SCALAR collects the scalar lane: the lift doors, the newtypes that would
carry an invariant, and the generic-scalar questions the kernel has been
answering by hand. This row is one of them.

Its class at the cut was **H** — stated design question; a validating
newtype is new policy surface needing Ev, plus generic-scalar fallout.
The class is a dispatch estimate made by reading the row against the
tree on 2026-09-11, not a verdict on the finding, and a lane that finds
it wrong says so in its PR. The id, the `track:` letter where the row
carries one, and the body above are unchanged by the move.

## Put to Ev (2026-09-12, SCALAR's first `[ev]` sitting)

**What the tree says, corrected.** A validating unit-vector type already
exists: `topo::query::UnitVec3<T>`, `new(v, band)` deciding the length
under the band with typed refusals, minted for datums and for
`eval/wire.rs`'s frames — and `frame_plane_lane` already feeds
`SketchPlane::from_frame` from two of them. `profile::path::Dir<T>` is
the 2-D witness-by-construction analogue: decided at the door that built
it, never re-decided. The row's `Plane::from_frame` is a one-line
wrapper; the real unchecked door is `geom_core::Affine3::from_frame`
(nine production callers including the Python bindings with hand-typed
axes). `frame_from_unit_aim` is private and both callers normalize a
decided length immediately before it. The prose-precondition count is
about twenty sites, not two, and they split in two: kernel DATA fields
(`Curve3::Line.dir`, `Surface::Plane.normal`, the axes) governed by a
stated rule — "conventional, unchecked, tier 3 certifies at rest"
(`geom/src/lib.rs`) — and FUNCTION parameters, of which
`Affine3::from_frame`, `Vec3::orthonormal_basis` and `frame_from_unit_aim`
are the ones on the row's path. Evidence: `work/scalar/log.md`.

**Recommendation, by the row's four questions.**

- **Q2 (the crux): witness-by-construction, no second validating
  type.** The validating door stays where a USER's vector arrives and a
  policy answer is owed (`topo::UnitVec3::new`, band owned by the
  caller). Inside the kernel the fact is already established by a
  decision the ladder makes (`definitely_positive` then `normalize`);
  what is missing is the type that RECORDS it. So `geom-core` gains the
  witness (`Dir3<T>` beside profile's `Dir<T>`, name the lane's), minted
  only by the decided-normalize ladder, by exact negation and by
  `sin_cos`; `topo::UnitVec3::new` becomes one mint of the same fact
  rather than a second type. Q1 is then answered by the existing door:
  the witness has no `new`.
- **Q3 (generic scalars): no new policy.** The witness says "produced by
  a normalize whose length decided positive under the band at this
  scalar" — at `Interval` an enclosure of a unit vector, at `Dual` the
  value channel with the tangent carried. It does not claim
  `‖u‖ == 1` bit for bit, which is the same thing Q1 says of every
  decided fact.
- **Q4 (does it pay): yes, at the function boundary only.**
  `Affine3::from_frame` takes two witnesses, `orthonormal_basis` and
  `frame_from_unit_aim` one; the Python-reachable hand-typed axes then
  go through the validating door. **The geom carrier fields stay under
  their at-rest rule** — moving unitness into those types is a Q1-scale
  change I do not recommend now; it gets its own row with a pointer to
  this one.

**Rejected:** a validating `Dir3::new` in `geom-core` (a second policy
surface duplicating topo's, and every site would re-litigate who owns
the band); and leaving it as prose (twenty sites, one of them reachable
from Python with axes nobody checked).

**Question:** (a) witness in `geom-core`, validating door stays at the
boundary, one fact; (b) carrier fields out of scope, rowed separately.

## RATIFIED (Ev, PR 2457, 2026-09-12)

"The unitvec plan sounds great." As put, with two refinements from the
thread:

- The witness lives in `geom-core`; `topo` does NOT re-export it —
  imports are repointed to `geom-core` instead (Ev: "probably don't
  re-export unless you need to").
- One mint, the normalizing constructor (`new(v, band)` as `topo`'s
  does today: decide the length under the band, divide), plus exact
  negation and `sin_cos`. No "check it is already unit" constructor:
  the one door with that posture, `sweep::tube_along_arc`
  (`crates/sweep/src/revolve/tube.rs`), takes `UnitVec3<T>` for both
  `axis` and `u_ref` instead, retiring `TubeError::NonUnitAxis` and
  `NonUnitURef` (two Python refusal tags go with them — PORT
  announced); `FrameNotOrthogonal` stays.
- Consumers: `Affine3::from_frame`, `Vec3::orthonormal_basis`,
  `frame_from_unit_aim`, the tube door; the geom carrier fields stay
  under their at-rest rule and get their own row.

This row is now a unit to cut: one or two units after the door rows,
with `D6`'s.

**Refinement (Ev, same thread): the tube takes a FRAME, not two unit
vectors.** Its inputs `center, axis, u_ref` are an origin and a
right-handed orthonormal pair (the roll is `u_ref`; a plane would not
fix it). The plan gains a second rung of the same shape: a frame
witness in `geom-core` — origin plus a right-handed orthonormal triple
— minted only by the decided Gram–Schmidt ladders (`eval/wire.rs`
`frame_axes`, `geom-core` `path_start_frame` and `frame_from_unit_aim`),
consumed by `Affine3::from_frame` and the tube door, whose three frame
refusals (`NonUnitAxis`, `NonUnitURef`, `FrameNotOrthogonal`) all
retire; the wire's private `AxisFrame` becomes that type. `Affine3`
stays the general affine map; the witness converts into one.
