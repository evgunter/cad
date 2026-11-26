---
id: the-partings-curved-face-gate-refuses-a-division-its-reason-does-not-cover
kind: issue
title: The parting's CurvedFace gate refuses every parting on a curved face, though its reason covers only a zero-area ring or a slit; a division (and a closed edge inside one face) has no arm
status: review
opened: 2026-11-25
priority: P1
cost: M
refs: [a-curved-slit-fails-tier-3-prime-on-its-two-coincident-arcs, a-tube-touching-a-ball-from-inside-along-its-rim-refuses-the-extent-scan, 7481, 7602]
pr: 7783
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

## 2026-11-25 — built on `heldrecords/split-on` (`heldrecords/parting-division`, PR 7783)

**The gate.** `boolean/slit.rs` `part_riders` no longer refuses by the
face's kind ahead of the surgery. Each parting on a curved face is read
once every parting is made: it builds where its two edges bound two
faces of the face's surface, and otherwise refuses `CurvedFace`. A
slit one parting leaves can close into a division when a later parting
ends on it: the ball less a keel whose rim crosses both seam meridians
reaches a lune as two arcs meeting at the keel's own vertex. A division
into two pieces of the result touching only along it refuses the new
`EdgeInFaceGap::PartsTouch`: the census does not read whether curved
faces of two pieces touch
(`work/atrest/the-census-cannot-record-a-curved-seat-between-two-solids.md`).
A wall tangent to a curved face refuses `Tangent`, the cusp it would
bound, rather than `CurvedFace`.

**The closed arm.** A closed conic inside a face with no rings divides
it (`divide_closed`): the gusset's ring through the edge's one vertex
parts there (`mev_null` across the fan, the minted edge's run taking
the copy, then `kemr`), and the circle bounding the disc becomes a face
(`mfkrh`). Which circle bounds the disc is decided at the conic's
midpoint (`disc_loop`, `bool_rider_closed_disc`): toward the centre on a
plane, and on a curved surface the side of the conic's plane the face's
outline is not on. The plane's closed rims take it too, in a face with no
rings. A face with rings still takes the zero-area-ring arm, which is
sound per PR 7783's review probe.

**Measured** at the default ε, 1e-6 and 1e-12, tiers 1, 3 and 3′:

- built: the ball `√2` less a keel whose rim lies in one lune (three
  poses) or across both seam meridians, the revolved ball the same
  (`a_closed_rider_lying_in_a_face.rs` `a_rim_on_a_sphere_divides_its_face`,
  one solid each, one or three edge–edge records); the plane rims
  (`a_closed_rim_in_a_plane_face_divides_it`); PR 5793's pierce rows
  and the keel on a rod's ruling (splitdoor row's 2026-11-25 note).
  Arm per rim: a closed rim in one lune takes the closed arm; a rim
  across both meridians is three arcs, the first loop to loop (both
  ends joined to meridian vertices), the other two a slit and then the
  arc that closes it;
- refused, next refusal named: the bore's and the torus's closed rims,
  the torus's quarter-turn arc, and SECT's torus less a puck at both
  turns, `Shared` (the vertex lies on the wrap edge, filed
  `a-rim-whose-vertex-lies-on-a-wrap-edge-refuses-shared`); the half
  ball less the tube, a=0.7 turned, `PartsTouch`; a=1, `EndOnBoundary`;
  the dome rim on the tube's seam ruling, `EndOnBoundary`; the slower
  dome and rod rim rows, `PartsTouch`, `Unplaced` or `EndOnBoundary` by
  pose (`pi_seam_and_kiss_through_the_boolean.rs`).

**The tube witnesses are not reached on this base.** Every "inside"
pose of `a_tube_ending_on_a_ball.rs` refuses
`FallbackExtentUnsupported` in every op before the parting, as PR 7602
(on main, not on `heldrecords/split-on` at `80f11deb86`) fixes. Expected
once a base carries main: `ball ∖ tube` divides the sphere faces (an
upright rim crosses both seam meridians, so it is loop to loop), and
since the results are 3 and 2 solids touching along the rims, they
refuse `PartsTouch` until the census reads curved touches between two
parts.

**The design note.** The parting cannot cite the sweep's decision
inside this diff: the sweep keeps no row of an arc lying on a face, only
its ends' records. Filed: `the-parting-re-decides-an-edge-lying-in-a-face-the-sweep-decided`.
Meanwhile the parting's own reading of a conic on a curved surface
carries its samples' margin, which the arc's `ArcOnFace` row cites.

Also filed: `work/parting/a-rim-on-a-spun-balls-seam-ships-one-edge-tier-3-prime-cannot-read`
(found by the sphere probes; the parting is not reached there, the
same on the base).
