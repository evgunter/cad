---
id: a-reach-straddle-goes-through-the-straddle-door
kind: unit
title: a carrier pair read Zero by its data and past the band by its extent bound is a straddle (C21, the torus witness), minted by k_stats::straddle through one geom-brep helper
status: closed
opened: 2026-11-09
priority: P1
cost: M
closed: 2026-11-09
pr: 6646
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

## Closed (2026-11-09, PR 6646)

C21 and the torus witness mint their straddles through `k_stats::straddle`, on the escalation log. The PR body has the per-site table (each ending before and after), the interval argument for `upper`, the sweep and the premise corrections.

- **`geom_brep::ladder::data_beneath_sum`** is the one helper: data decided `Zero` beside a sum that did not decide `Zero`. `tori_coincide` and `carrier_eq::coincident_over_extent` both call it. The data bound the carriers' separation, not the displacement over a partial extent, and the low end is the first datum's reading. C21's rows are `bool_plane_reach_straddle` and `carrier_{sphere,cyl,cone}_reach_straddle`.
- **`TorusCoincidence::Apart { datum, by, upper }`.** `TorusDisplacement::upper` sums `max(chord, sine)`. The face-pair door (`carrier_eq::pair_door_reading`) hands the ring's apart back as `PairDoorReading::RingApart { upper, .. }`, and `carrier_pair_decided` passes `upper` to the witness, which mints `carrier_torus_witness_straddle`.
- **Premise correction:** "apart ⇒ `upper` decided" holds for readable input only. A datum apart beside a non-finite datum leaves `upper` poisoned, and that pair now ends `Undecided` on the bound's poison, not `Apart`.
- **The K note is C21's alone.** The witness straddle is reachable at any K.
- **Pinned:**
  - C21 at K = 1.5 on cylinder sheets, through `carrier_pair_decided` and `rest_pair_verdict`;
  - the witness at the run's K on a torus patch.
- **Filed:** `the-torus-floor-row-and-across-have-no-reader`.

