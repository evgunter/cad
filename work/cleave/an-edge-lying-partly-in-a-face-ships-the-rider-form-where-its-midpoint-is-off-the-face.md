---
id: an-edge-lying-partly-in-a-face-ships-the-rider-form-where-its-midpoint-is-off-the-face
kind: issue
title: A Boolean edge lying in a face along part of its length ships the rider form where the face holds no edge-on-face record with its midpoint in it
status: closed
opened: 2026-10-25
priority: P1
cost: M
branch: cleave/riders-read-every-edge-in-a-face
pr: 5793
closed: 2026-10-31
---


Found by the review of PR 5546 (the row
`a-boolean-edge-running-past-the-face-it-lies-in-is-not-parted`), with
the reviewer's probe scenes: the plate `x∈[-1,3], y∈[-.5,.5], z∈[0,1]`
less the diamond `|x−.75|+|y|<.25`, whose edge at `(1, 0)` runs along
`z`, against cutters on `x = 1` extruded along `+x`.

## What

`topo::boolean::slit::riders` (`crates/topo/src/boolean/slit.rs:599`)
finds an edge lying in a face only where both its ends lie on the face
(by structure or a record), or an edge-on-face record names the pair
and `lies_in` (`slit.rs:645`) reads the edge's **midpoint** inside the
face. An edge lying in the face along part of its length whose midpoint
the face does not hold is never read, so `A ∖ B` ships the rider form,
one edge whose wedge the face passes through, and tier 3′ refuses it.
D1-wrong topology, shipped silently by the Boolean door, whose gate
does not run the census
(`work/reachhold/boolean-door-runs-the-census-over-its-result.md`):

- the cutter `y∈[−.6,.6], z∈[.6,.9]`: the edge lies in the pocket's wall
  `x = 1` over `z∈[.6,.9]`, past the edge's midpoint `z = .5`; 3′
  `FaceThroughEdgeWedge` at `(1, 0, .75)`, at every ε;
- the same with the midpoint on the wall's boundary edge, or inside a
  hole of the wall (the review's variants);
- a pocket floor bounded by arcs crossed by a tunnel's apex edge lying in
  it: a disc pocket `r = 1`, `z∈[1, 1.8]`, in the block
  `[−2,2]²×[0,1.5]`, less the tunnel over the triangle
  `(y,z) = (0,1), (∓.2,.6)` along `x∈[−2.5,2.5]`; and a D-shaped pocket
  (a chord and a half-circle) crossed along `y`. Their records are
  edge-edge records with the floor's arc edges only, no edge-on-face
  record of the pair, so the edge is never read; 3′
  `FaceThroughEdgeWedgeUndecided` at every ε.

The reviewer's probe holds each scene:
`/tmp/claude-0/-home-user-cad/13a560da-7ecc-50fe-9b1a-ae4821afb423/scratchpad/rev5546c-probe.rs`
(scenes `s6`, `s13`, `s16`), not in the tree.

## Fix

Read an edge as lying in a face along part of its length from what
names that: an edge-on-face record of the pair read on the edge's span
rather than at its midpoint (the piece of the edge the face holds), and
an edge-edge record with an edge of the face's loops whose crossing
leaves the edge on the face. Then `slit::divide` divides it there;
an arc of the loops crossed is its `CrossedCurve` gap.

## 2026-10-26 — read by its crossings (CLEAVE, `cleave/riders-read-every-edge-in-a-face`)

- `slit::riders` (`crates/topo/src/boolean/slit.rs:839`) asks a face
  of an edge where both of the edge's ends lie on it (a vertex-on-edge
  record naming an edge of its loops counts as lying on it), an edge-on-face
  record names the pair, or a record names a crossing of the edge with
  the face's loops: an edge-edge record with an edge of them, or a
  vertex-on-edge record of a vertex of them (`crossed_faces`,
  `slit.rs:928`).
- `slit::holding` (`slit.rs:985`) reads the edge as lying in the face
  where its carrier lies on the face's surface (`carrier_on_surface`,
  `slit.rs:1127`) and an edge-on-face record names the pair, or the face
  holds the midpoint of one of the pieces the recorded crossings cut it
  into (`cuts`, `slit.rs:1061`). The edge's own midpoint decides
  nothing. The edge is divided first unless both its ends lie on the
  face and no record names a crossing. A face on the surface one of
  the edge's own faces lies on is not asked: the edge bounds that sheet.
- `slit::divide` (`slit.rs:1635`) reads each crossing through
  `curve_pair::meet` for any certified carrier of the loops
  (`crossing_named`, `slit.rs:1868`), so an arc of the loops is divided
  where the edge crosses it, once or twice per arc; the resting
  vertices on a crossed edge go to the piece holding them, ordered
  along it (`slot_from_far_end`, `slit.rs:1944`). A meeting the reader
  does not settle refuses `CrossedCurve`.
- An edge lying partly in several faces (each holding a piece of it) is
  divided against all of them at once and each piece parted from the
  face holding it (`slit::part_riders`, `slit.rs:345`); where one of
  them holds it whole the parting refuses `Shared`. An arc lying in
  several faces refuses `Curved` before it is divided.
- Witnesses, `sweep/tests/edges_lying_in_a_face_part_wherever_they_lie.rs`:
  the cutter over `z ∈ [.6, .9]`, over `[.25, .5]` (the midpoint on the
  wall's boundary) and the notched wall (the midpoint in its hole); the
  disc floor, the D floor crossed through its chord and its arc, and the
  D floor crossed twice by its arc; and the slotted wall, whose slot
  cuts the face the keel lies in into two faces holding a piece each
  (`a_keel_across_a_slotted_wall_parts_against_both_faces`). Every op, both orders, at
  1e-6, 1e-9 and 1e-12 builds at its closed-form volume with tiers 3
  and 3′ and its material at sample points, and `A ∖ B` holds two
  coincident edges over each piece the face holds. Against main's
  reading every `A ∖ B` holds one edge there and fails 3′
  (`FaceThroughEdgeWedge`, or `…Undecided` on the pocket floors).
- The review's probe (its 22 scenes, every op, at three ε), against
  main: the scenes above move from the rider form to the parted form,
  and the shallow crossing tilted 1e-7 (at 1e-9) or 1e-4 (at 1e-6,
  `an_edge_running_past_a_face_reads_its_crossings.rs` re-pinned)
  moves from an in-band read of the edge's midpoint to a sound build;
  nothing else moves.
- The reading still finds no edge lying in a torus or spline face, nor
  one lying in a face where no record names the pair or a crossing; it
  ships one edge there. Filed as
  `work/cleave/the-parting-reads-no-edge-lying-in-a-torus-or-spline-face.md`
  and `work/cleave/the-parting-reads-no-edge-lying-in-a-face-no-record-names.md`.

## Closed (2026-10-31)

PR 5793. `slit::riders` asks a face of an edge where the edge's ends
lie on it (by structure, a vertex-vertex partner, a vertex-on-face
record, or a vertex-on-edge record on its loops), where an edge-on-face
record names the pair, or where a record names a crossing of the edge
with the face's loops. `slit::holding` reads the edge as lying in the
face where its carrier lies on the face's surface and the face holds
the midpoint of a piece the recorded crossings cut it into; the edge's
own midpoint decides nothing. `slit::divide` divides the edge at each
crossing, line or arc of the loops (`curve_pair::meet`), against every
face holding a piece of it at once, and the parting builds two
coincident edges over each piece. Witness:
`sweep/tests/edges_lying_in_a_face_part_wherever_they_lie.rs`, every op
in both orders at 1e-6, 1e-9 and 1e-12 at its closed form with tiers 3
and 3′. Residue is filed: the closed edge, the torus or spline face, and
an edge no record names (`work/cleave/`).
