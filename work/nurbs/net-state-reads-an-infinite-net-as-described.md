---
id: net-state-reads-an-infinite-net-as-described
kind: issue
title: NetState answers Described for a net of infinities, so only tier-3 check 1 refuses it and every other three-state consumer takes it as geometry
status: open
opened: 2026-10-10
priority: P3
cost: M
design: true
refs: [described-net-two-state-reads-hand-a-poisoned-net-the-described-arm]
---


## What

`NurbsSurface::net_state` (`geom/src/surfaces/nurbs.rs`) separates the
three states on the scalar's poison, and at `f64` `Real::is_poison` is
`is_nan` (`geom-core/src/real.rs`, the `f64` impl). A net with a `±∞`
control point is therefore `NetState::Described`, although it describes
no locus. Its evaluations give `∞ − ∞ = NaN` and its folds give
infinite boxes.

Tier 3 knows this and patches it locally. Check 1 in
`topo/src/validate.rs` adds `NetState::Described if
!net_is_finite(payload.control())` and refuses
`PoisonedSurfaceDescription`. `net_is_finite`'s doc says the
discriminator "stays as it is" because it answers the placeholder
question. No ratification of that sentence was found. This repo's
checkout is shallow, so `git log -S` could not reach the commit that
wrote it.

The gap is that every other consumer told to match on `net_state` (the
`NetState` docs: "every consumer that tells the states apart matches on
`NurbsSurface::net_state`") has to relearn validate's extra read, and the
first one did not. `topo::merge_faces`'s `MergeKind::of` maps
`Poisoned` to `Err(PoisonedNet)` and `Described` to `Curved`, so a net
of infinities groups as a curved run and gets the surgery that the
poisoned net is refused before. The output is refused at rest, which is
why this is P3. Reachability: STEP import can produce such a net,
because `as_real` parses `1E999` to `+∞` and the lexer admits it. The
import's closing `validate_geometric` refuses the body.

## The question

There are three viable answers, so `design: true`:

1. widen `NetState::Poisoned` to "some channel of some point is not a
   finite number" (at `f64`, NaN or `±∞`), with the doc saying the state
   is about describing a locus, not about the scalar's poison;
2. add a fourth state for a non-finite net;
3. keep the discriminator and move `net_is_finite` into `geom` as a
   door every `Described` arm is told to ask.

Option 1 retires validate's local patch and changes `merge_faces`'s
answer with no edit there. The interval scalar already reads `±∞` as
NaI (`geom-core/src/interval.rs`, `from_f64_poisons_non_reals`), so
option 1 would also make the state sets agree more closely across
scalars.

## Two more consumers (PIPE, from S350's lane and its review, PR 4482)

Two box doors pass `±∞` straight through, so an infinite net folds to a
box with infinite ends:
- `geom::surfaces::boxes::nurbs_surface_aabb`, whose `any_poison` screen
  is NaN-only at `f64`;
- `Aabb::from_points`.

The census reach arm (`census.rs`'s `face_reach`, after S350) reads
`net_state()` and so inherits the same reading. Not reproduced as a
false clear. A finiteness screen at `net::any_poison` (options 1/3
above) closes both. This was filed separately on 4482's branch, and
folded here as one item on one decision.

## Re-homed from FLUX to KNOT (2026-10-10)

(FLUX orchestrator) FLUX measured 125.5 budget points against 30 and was cut on its priority seam: FLUX kept the curved closed-form arms. KNOT collects the spline, net and fit doors in `geom-core` and `geom`: refinement inside an enclosure, the net-state reads, the NURBS derivative and projection doors, and the fit's refusals and solves. The id and the body above are unchanged; the move may have set `priority`, `cost` or `status` in the header.
