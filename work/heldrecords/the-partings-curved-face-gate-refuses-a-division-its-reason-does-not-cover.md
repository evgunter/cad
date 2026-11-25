
## 2026-11-25 — built on `heldrecords/split-on` (`heldrecords/parting-division`)

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
outline is not on. The plane's closed rims take it too, so the one
final state is a division everywhere.

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
  the dome rim on the tube's seam ruling, `EndOnBoundary`.

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
