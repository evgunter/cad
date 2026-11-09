---
id: a-reach-straddle-goes-through-the-straddle-door
kind: unit
title: a carrier pair read Zero by its data and past the band by its extent bound is a straddle (C21, the torus witness), minted by k_stats::straddle through one geom-brep helper
status: dispatched
opened: 2026-11-09
priority: P1
cost: M
parent: glue-disagreements-and-held-contradictions-after-stage-4
---


Step 2 of the parent's design (fork `glue-disagreements-carry`, 2026-11-09).

## What

Two sites hold a lower and an upper bound on one quantity: the faces' displacement over their extent. The lower is decided Zero and the upper past the band. That is `k_stats::straddle`'s contract, and geom-brep's `tori_coincide` already mints it (`tt_coincident_straddle`).

- **C21, `carrier_eq` `coincident_over_extent`.** Every datum decides Zero (lower bounds), and the displacement's upper bound over the extent decides past the band. `pair_door_reading` returns `straddle("carrier_<kind>_reach_straddle", joint datum Zero, sum, band)`.
  - The "past it where the bound stands off" clause leaves `CoincidenceMeasure::Undecided`'s doc.
  - The case is reachable only at K below about 3–4, but K is a per-run dial, so it is not an `unreachable!`.
- **`carrier_pair` `torus_pair_witnessed`.** The witness (the farthest boundary vertex from the other carrier) is a lower bound on how far the faces stand off. `TorusDisplacement::upper` holds at every point of both tori, so it is an honest upper bound.
  - `TorusCoincidence::Apart { datum, by: Decided, upper: Decided }`, and the witness mints the straddle against `upper`.
  - Interval hazard: `upper`'s axis term is the chord (`line_tilt`) where the datum uses the sine. Sum `max(chord, sine)` so that a datum decided apart makes `upper` decided past the band. The type then needs no "apart with no upper" arm.
- **One rule:** one geom-brep helper ("data as lower bounds, a sum as the upper") serves `tori_coincide` and `carrier_eq` both.

CONTACTHOLD's `the-torus-witness-reads-only-vertices-two-tori-share` is orthogonal: a stronger witness makes the straddle rarer. Its fixture should read the enclosure [≈1e-16, 0.8].

## Done when

C21 and the torus witness are funnel-minted straddles on the escalation log, pinned at a K that reaches them, and no hand-built `Undecided` carries a decided margin on either path.

## Dispatch (2026-11-09)

Branch `seal/reach-straddle`. Review tier: **single, FULL**. The step joins two straddle sites through one geom-brep helper and widens an interval bound (`max(chord, sine)`), whose soundness is to be checked rather than read. It is no design decision: the fork settled the shapes.
