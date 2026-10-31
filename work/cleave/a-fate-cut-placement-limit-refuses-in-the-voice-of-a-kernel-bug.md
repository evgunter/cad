---
id: a-fate-cut-placement-limit-refuses-in-the-voice-of-a-kernel-bug
kind: issue
title: A fate cut's placement limit (Limit::NoCorner, SeveralCorners, …) refuses as ClassificationInvariant, the voice of a kernel bug, where the cause is a limit of the corner reading
status: open
opened: 2026-10-31
priority: P2
cost: M
---


Found by PR 5852's review.

## What

`crates/topo/src/boolean/fate.rs:395`, `impl From<CutError> for
BooleanError`, turns every placement limit of a fate cut (`Limit`,
`fate.rs:322`) into `ClassificationInvariant { what: limit.what() }`.
That variant is the voice of a kernel bug: an invariant the kernel
broke. A placement limit is not one; it is a geometry the corner
reading does not part, and the refusal should say so in a typed
variant a caller can tell from a defect. The containment fallback
already treats a limit as one (`boolean/ops.rs:8267` keeps the face
uncut and records it); the section path, which has no such fallback,
refuses with the invariant's words.

`Limit::NoCorner` ("a fate cut's end lies in no corner of the dropped
face", `fate.rs:921`) is reached by at least these limits of the
reading, none of them a broken invariant:

- **A spline or spiric bound.** Its bend carries no bound
  (`geom_brep::BendBounds::Unbounded`), so a kept edge leaving along it
  at first order is not read one order down (`corner_holds`'s
  `Holds::Along`) and falls to the secant pass, which can find no
  corner.
- **A third-order tie.** A kept edge leaving along a bound with an
  equal bend reads `Holds::Tied`; where it does not run along that
  bound to the cut's other end, `place` refuses rather than read the
  secant (`fate.rs:916`), whose verdict there would follow the two
  edges' lengths rather than their sides.
- **A reflex cusp.** Where a corner's two curved bounds leave along
  one ray and the face takes the reflex side, `wraps` reads only a
  dangling edge or two straight edges at a tip, so the corner is read
  as the cusp between the bounds (`fate.rs:665`, `wraps()? || both`),
  and a direction off the ray, its sides of the two bounds opposite,
  never reads `both`: the corner holds nothing the face holds.

The same voice, for a different limit:
`work/cleave/a-glued-slit-intersection-refuses-a-classification-invariant-since-5505.md`.

## Shape of a fix

A typed refusal for a fate cut's placement limit (a `BooleanError`
variant or a typed reason on an existing one), carrying the `Limit`,
whose words say the cut was not placed rather than that an invariant
broke; `ClassificationInvariant` kept for what is one.
