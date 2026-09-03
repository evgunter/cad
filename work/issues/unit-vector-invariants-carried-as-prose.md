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
