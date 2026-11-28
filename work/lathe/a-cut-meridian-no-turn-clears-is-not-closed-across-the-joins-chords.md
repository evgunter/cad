---
id: a-cut-meridian-no-turn-clears-is-not-closed-across-the-joins-chords
kind: issue
title: A crossings-path cut whose meridian no turn clears of a first-pass section is not cut in: the closing cannot read the join's chords along a cut
status: open
opened: 2026-11-27
priority: P2
cost: H
refs: [close-cuts-is-not-total-on-crossings-path-cut-ins, a-section-circle-through-both-chart-poles-leaves-a-cut-face-with-no-pole-to-read]
---


## What

A crossings-path interior-loop cut-in on a sphere face runs its cut in a
meridian plane chosen clear of the face's first-pass sections
(`crates/topo/src/boolean/ops/interior_cuts.rs`, `closing_meridians`).
It tries a fixed sequence of turns of the meridian about the axis, and
takes the first plane decidedly off every partner plane (a plane, or a
sphere's radical plane) and every other cut-in circle's plane on the
face. That choice holds the closing's first premise: no join chord runs
along a cut (`crates/topo/src/boolean/ops.rs`, `cuts_in_result`).

**The ladder** (`meridian_turns`) is nine turns, `t ∈ {0, ±1/8, ±1/4,
±3/8, ±1/2}`, of the normal `k0 + t·(axis × k0)`, where `k0` is the
circle's own meridian. A section plane blocks a candidate only where it
holds the axis and the centre, so it is one meridian plane, and it blocks
one candidate at most. Nine distinct candidates so clear any eight
blocking planes on the face. The turns are literals, so every pose picks
the same plane at every scalar. The largest turn is `atan(1/2) ≈ 26.6°`
either side of the own meridian. The reach is kept small because the
own meridian is the plane that puts both of an interior circle's
crossings on the cut's half-plane, and a large turn can carry a crossing
of a circle near the pole across the axis to the other half.

A pose can so fail to cut in for three reasons:

- **every candidate within ±26.6° is blocked**, decidedly: by more than
  eight section planes, or where the circle's chord leaves the half or
  is not read meeting the circle twice at every turn. The op goes on
  uncut, to the guard. Where a turn is read in band and none clears, the
  op escalates as that reading instead.
- **a partner on the cut face that is no plane or sphere** (a cylinder,
  a cone, a torus). Its section is no plane, so `closing_meridians` has
  no plane to read a meridian against, and it returns `Ok(None)`.
- **a loop's cut**, which keeps its own meridian (`axis × toward`) and
  is not turned. Where a section crosses that meridian, the op does not
  cut in.

## What would close it

The closing reads the join's chords on the cut and closes across them
(ORBIT's `crates/topo/src/boolean/slit.rs`, `close_cuts`). The meridian
choice then stops being a premise.

## Witnesses

None measured. No row reaches either case today. This item is what
remains of `close-cuts-is-not-total-on-crossings-path-cut-ins` (1b's
generality) after `lathe/close-cuts-total`.
