---
id: a-sliver-arc-beside-a-joinable-vertex-refuses-the-boolean
kind: issue
title: The join levers its vertex reading over the shorter edge, a scale that is not the joined edge's (a few-ε arc beside a joinable vertex cannot reach it: its own certification refuses first)
status: open
opened: 2026-10-07
priority: P3
cost: E
---

## The finding

`joinable` reads the surface pair's dihedral at a valence-2 vertex
levered over the shorter of its two edges' extents
(`crates/topo/src/boolean/edge_join.rs:353`, `classify_dihedral` at
`:367`), so the reading does not depend on which edge the arena lists
first. A curved arc only a few ε long beside an otherwise joinable
vertex shrinks that lever until the wedge reads in the band, and the
whole boolean refuses `BooleanError::JoinUndecided`, where main, which
did not join curved vertices, built the body and left the vertex. The
ruling allows it (a reading in the band refuses typed), and the offer
site says so (`offer_rows::EDGE_JOIN_SITE`). No `ci` row reaches it.

## What it needs

A lever that does not shrink with the sliver: the longer edge's
extent, or the locus's own curvature scale at the vertex, so a short
arc beside a transverse pair still decides. Whichever is chosen must
stay independent of which edge the arena lists first.

## The premise does not hold (designer pair, 2026-10-13)

Two designers, Opus and Fable, were dispatched concurrently with the same problem statement. Each measured independently on main, and they agree.

**Why the arc cannot reach the join.** A certified edge's own certification reads the same margin, `sin θ · min(arm, extent)` over its own extent, against the same band. It does so at mint, at `split_edge` and at the boolean's seam description, and refuses in band.
- **A short arc never reaches the join:** its cut refuses at mint, typed, with certification's `Transversality` escalation.
- **Past that line every join decides.** Measured on two poses:
  - a cap arc at sin θ = 1, where cuts below 10ε refuse and from 10.5ε the joins decide;
  - a plane-cylinder ruling at sin θ = 0.5 / 0.1 / 0.02, where the cut refuses exactly where `sin θ · s < Kε`;
  - an oblique rim at sin θ from 1.0 to 0.021: 48 cells, with no undecided join reading.
- **"Main built the body" was never observed.** The same gate predates the curved join.

**The vertex reading stays.** At a locus crossing, for example two equal cylinders' figure-8 tangency points, the wedge reads zero. The vertex is correctly left, and a vertex near the crossing refuses with the move-the-vertex recourse.

**Owed (cleanup, both designers):**
- Lever the vertex reading over the extent of the edge the join would make. That extent is symmetric and never smaller than either edge's. It makes the reading predict the certificate `joined_spec` is about to mint. Its order-free lower bound is the longer edge's extent (`min` → `max` in `joinable`).
- Correct the comment above `offer_rows::EDGE_JOIN_SITE`, which asserts the unreachable scenario.
- Land the measured boundary as a `ci` row. Cuts within `Kε / sin θ` of a corner refuse at certification; past it, the join joins and passes tiers 3 and 3′.

**Rejected by both:**
- curvature arm only, with no extent: it decides at a scale the body does not have, and the joined edge's certification then refuses it;
- dropping the vertex reading: it would join through a branch point.

No ratified text changes. This was not a fork for Ev.
