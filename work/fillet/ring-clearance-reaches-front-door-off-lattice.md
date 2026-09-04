---
id: ring-clearance-reaches-front-door-off-lattice
kind: issue
title: RingClearance reaches fillet_edges when the ring's closest point is off the screen's sample lattice; the screen's exactness claim is narrower than written
status: closed
opened: 2026-09-04
closed: 2026-09-04
pr: 1753
refs: [recourse-sentences-owe-followability-pin]
---

## The witness

A 2×2×2 square prism turned 30° about its axis, with a radius-0.3
sphere subtracted at its top centre (ring radius 0.2828 on the top
face; true gap to each top edge `1 − 0.2828 = 0.7172`). Fillet all
twelve line edges:

| r | door answers |
|---|---|
| 0.700, 0.715 | builds, tier-3 valid |
| 0.718, 0.720 | `RingClearance` carrying `FILLET3_RING_RECOURSE` |
| ≥ 0.722 | `FaceClearanceUncertified`, gap reported **0.7205** |

Probe: `crates/sweep/tests/review_fillet_e2_probes.rs`.

## Mechanism

`consumption_sweep` samples every boundary edge at `CHAIN_SAMPLES = 9`
places (`crates/sweep/src/blend/battery.rs:53`, `:1544`) — a 45°
lattice on a circle. When the ring's closest point to an edge sits 15°
off that lattice the sampled gap is `R·(1 − cos 15°)` too large, the
screen passes, and the surgery's exact `ring_clearance`
(`crates/sweep/src/blend/surgery.rs:1769`) refuses. The same
overestimate shows for two PARALLEL line features whose sample points
do not align: the pocketed-cube probe reports a gap of 0.35089 where
the true gap is 0.35.

## What is narrower than written

- `face_clearance`'s doc (`battery.rs` ~`:632`) says the screen is
  exact "when the two boundary edges face each other (parallel,
  opposed inward normals)" and "cannot pass a request whose support
  face really is consumed". The first is true only when the two
  features' samples align; the second is true only because the surgery
  re-checks rings exactly — the screen itself passes such a request.
- `ring_clearance`'s doc (`surgery.rs:1747–1754`, "FRONT-DOOR-SCREENED
  by predicate 2") and the doc comment PR 1753 adds to
  `FILLET3_RING_RECOURSE` ("No caller has been handed this sentence")
  are false in general; they hold on axis-aligned fixtures.

The recourse itself is followable — "reduce the blend size" builds at
0.715 — so this is not a dead recourse, and the existing rows keep
their soundness. The row
`blend_recourse_followability::the_ring_recourse_has_no_front_door_witness_the_clearance_screen_answers_first`
is fixture-scoped and does not go red on this witness.

## The ask

State the screen's exactness as it is (sampled; exact where samples
coincide with the closest approach), retract the two unreachability
claims, and give `FILLET3_RING_RECOURSE` the composed pin its class
owes: the front-door witness above, followed to the build.

## Resolved (PR 1753)

All three asks are done.

- **The screen's exactness is stated as it is.** `ring_clearance`'s doc
  (`surgery.rs`) no longer says the refuse arm is FRONT-DOOR-SCREENED.
  It now states the one-sided invariant that is actually true — the
  sampled gap is never SMALLER than the true one, so nothing this check
  would pass is refused by the screen — and says plainly that the screen
  does not always answer first, naming the turned-prism witness and the
  axis-aligned fixtures the old wording was written against.
- **Both unreachability claims are retracted.** The doc at
  `FILLET3_RING_RECOURSE` now describes the off-lattice reach and points
  at the witness row.
- **The composed pin is adopted**, as
  `review_fillet_e2_probes::the_ring_recourse_reaches_the_front_door_off_the_sample_lattice_and_is_followable`.
  The followability suite's row is rewritten as its lattice-ALIGNED twin,
  `the_ring_recourse_is_screened_first_on_a_lattice_aligned_dimple`, whose
  doc names axis alignment as the premise doing the work.

**Not addressed here, deliberately:** `face_clearance`'s own doc
(`battery.rs` ~`:632`) makes the wider version of the same overstatement
about two parallel boundary edges, and this file measured it (0.35089
reported for a true 0.35). That doc governs the battery's screen rather
than the fillet's recourse prose, and correcting it is a claim about
every caller of `face_clearance`, not just the ring path. Left for the
battery's owner, with the measurement above as the input.
