---
id: the-partings-curved-face-gate-refuses-a-division-its-reason-does-not-cover
kind: issue
title: The parting's CurvedFace gate refuses every parting on a curved face, though its reason covers only a zero-area ring or a slit; a division (and a closed edge inside one face) has no arm
status: dispatched
opened: 2026-11-25
priority: P1
cost: M
refs: [a-curved-slit-fails-tier-3-prime-on-its-two-coincident-arcs, a-tube-touching-a-ball-from-inside-along-its-rim-refuses-the-extent-scan, 7481, 7602]
---

Filed by INSIDE's orchestrator from the two-designer weighing of
`work/inside/a-tube-touching-a-ball-from-inside-along-its-rim-refuses-the-extent-scan.md`
(2026-11-25). Both designers agreed it is not a design fork; this is the
mechanism the row's remaining refusal waits on. Measured by one designer
on main `133ca95e02` (a probe, not committed); the rest is code reading.

## What

`boolean/slit.rs`'s parting refuses `EdgeInFaceGap::CurvedFace` for any
parting on a non-planar face. Its doc and words give the reason: the
parting "would leave a slit or a zero-area ring in a curved face", and a
zero-area ring is not measured
(`work/nearpoint/a-zero-area-ring-in-a-cylinder-face-measures-degenerate.md`).
A parting that **divides** a curved face leaves neither: an arc running
from one of the face's boundary loops to another, or a closed edge
inside one face, cuts it into two ordinary faces. The gate refuses a
case its reason does not cover.

Second half: a **closed** rider edge inside one face separates that
face, and tier 3 allows one region per face, so the "neither end on the
boundary → zero-area ring" arm cannot represent it. The parting needs a
division arm for a closed edge. Latent while closed edges refuse
`Curved`; live once PR 7481 (`heldrecords/split-on`) lifts `Curved`.

## Witnesses

- `crates/sweep/tests/a_tube_ending_on_a_ball.rs`
  `the_probe_family_ships_no_wrong_body`, poses `inside, both rims`,
  `inside, one rim`, `inside, one rim tilted` (radii 1 and 0.7). Since PR
  7602 ∪ builds as the ball, ∩ as the tube, `tube ∖ ball` is Empty, and
  only `ball ∖ tube` refuses, `EdgeInFace { gap: Curved }`; `CurvedFace`
  is next in line. Expected once built (`[topology.conventions]`: pieces
  that only touch are distinct solids; PR 5967's edge–edge pair): 3
  solids for both rims, 2 for one, volume the ball's less the tube's, one
  edge–edge record per rim. Which arm each rim takes is unmeasured: the
  upright rims likely split at the ball's seam meridians (loop to loop),
  while the tilted one-rim pose may be the closed-in-one-face case.
- SECT's torus less a puck on a parallel (this program's
  `a-curved-slit-fails-tier-3-prime-on-its-two-coincident-arcs`,
  2026-11-20 note), and the four rows PR 5793 moved onto this gap.

## Done when

The gate refuses only the arms that would leave a zero-area ring or a
slit in a curved face; a division builds, including a closed edge inside
one face; the witnesses above pin SOUND with their solid counts and
records. Re-word `CurvedFace`'s doc and words to its narrowed reason.

## Design note for PR 7481's review (both designers)

The parting re-derives "does this edge lie in that face"
(`riders`/`holding`: `carrier_on_surface` plus the piece holding the
midpoint) after the reduction sweep has already decided it
(`reduce.rs` `lying_on`, `SpanVerdict::LiesOn`), and keeps only its end
vertices' records. One final state reads one home: the sweep records
its lie-on for any curve and surface and the parting cites that row,
rather than a second `ArcOnFace` decision. Weigh it at PR 7481's review;
the curved edge–face record also waits on the census's curve-touch lane
(`an-edge-face-record-off-a-line-and-a-plane-has-no-curve-touch-lane`).
