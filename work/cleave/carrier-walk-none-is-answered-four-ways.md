---
id: carrier-walk-none-is-answered-four-ways
kind: issue
title: point_in_carrier_loop's None (no ray past an uncrossable edge) is answered four ways under three names, one of them silent
status: open
opened: 2026-10-01
priority: P1
cost: M
---


Filed from the review of PR 3660 (CLEAVE, `rehome-rings-reads-an-arc-bearing-run-through-the-polygon-walk`).

## The four answers

`splitting::containment::point_in_carrier_loop` and its sibling
`carrier_loop_side` return `Ok(None)` when the schedule runs out with
at least one ray abandoned because it could meet an edge the walk has
no crossing row for (a spiric or a spline, held as a ball). Their
callers answer that one outcome in four ways under three names:

1. `boolean::contain` (the `inside` closure over `carrier_loop_side`):
   `ContainError::ArcLoopUnsupported { loop }`, which census routes to
   `CensusUnsupported` (`census.rs`, the `ArcLoopUnsupported |
   RayExhausted | Corrupt` arm).
2. `boolean::solid_contain::point_in_face`:
   `PointInSolidError::EdgeCarrierUnsupported { face }`.
3. `chord_join::rehome_rings`: `SplitJoinError::RingHomingUncrossable
   { ring }` (new in PR 3660).
4. `validate::ring_nesting`: `Ok(None) => {}`. The vertex is skipped,
   and if no vertex decides, the ring reads `Inside`. That is a
   **silent** answer, not a refusal.

The first three are the same refusal under three names. A shared name
(or a `PointInLoopError` arm the walk itself returns) would let each
caller wrap one thing. The fourth needs a decision: either check 9
says it could not place the ring (the `Undecided` arm it already has
for escalations), or the doc states why `Inside` is the honest default
there.

## The scaffold circle (review N2, unreached)

`carrier_loop` reads every certified carrier it meets. A null
self-loop edge certified as `EdgeCurveSpec::self_loop_circle_at`
(`crates/geom-brep/src/certify.rs`: a full-period circle through one
point, "deliberately arbitrary geometry") would be read by
`ConicArc::of` as a real circular arc, not as the zero-length chord
the null scaffolding is. No caller of the walk is known to reach a
loop holding one at rest. It is the same walk, so whoever settles the
`None` question should settle this too: either skip scaffolding by
its description, or state why a walked loop never carries it.
