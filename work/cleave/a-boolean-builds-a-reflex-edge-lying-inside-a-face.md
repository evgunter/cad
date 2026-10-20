---
id: a-boolean-builds-a-reflex-edge-lying-inside-a-face
kind: issue
title: The boolean builds a reflex edge lying inside a face, and returns one solid for a wall-to-wall trench
status: closed
opened: 2026-10-12
priority: P1
cost: M
branch: cleave/boolean-builds-the-slit-form
pr: 5380
closed: 2026-10-20
---

Found by the keel design fork (`[ev]` PR 4795, designer reports in its
body), measured on main `360cf55eb2` at `Tol::witness`.

## What

The slab `[0, 4]² × [0, 1]` less the trench of
`a-keel-in-the-plane-wrapped-round-at-both-ends-refuses-a-degenerate-section`
(prism over the (y, z) triangle (2, 0), (2.6, 2), (1.4, 2)), with the
keel resting on the floor face z = 0:

- x ∈ [1.5, 2.5] (both ends closed): `topo::subtract` keeps the keel as
  one edge between the two walls lying inside the floor face, with an
  Edge(keel)–Face(floor) record and two Vertex–Face records. Tiers 2, 3
  and 3′ pass. The keel edge's wedge reads ~327° of material; the
  material is two disjoint ~73° wedges with the floor between them.
- x ∈ [1.5, 4.5] (one end open): the same form, with Edge–Face and
  Vertex–Face records.
- x ∈ [−0.5, 4.5] (wall to wall): v14 e23 f11, **one solid and one
  shell**, no records, tiers 2, 3 and 3′ pass, volume 14.8. D1's
  definition of a solid (PR 3901) makes pieces touching only along an
  edge distinct solids; `topo::split`'s rule (b) gives two for the same
  geometry.

The split builds the same bodies in the slit form (two coincident
distinct edges; PR 4622 for the one-end case). One body, two
topologies, by door.

## Fix

Under PR 4795 as written, the boolean builds the slit form: no face
passes through an edge's material wedge, and the keel is two
coincident distinct edges (a zero-area ring where it ends inside the
face). If Ev rules for the rider form instead, the split changes and
this row is re-scoped. Either way the two doors must agree, and the
wall-to-wall case returns two solids. Nearest row:
`work/contacthold/a-bridge-union-fuses-a-declared-tangent-rest-into-one-shell-with-an-edgeless-contact.md`
(the curved analogue).


**Unblocked (CLEAVE, 2026-10-19).** `[ev]` PR 4795 merged (`0a702a9bc1`). Ev ratified the D1 sentence in `crates/topo/README.md` tier 3: where more than two sheets meet on a line, the material decides the edges, and the complement re-pairs. Radial edges were declined.

**Built (CLEAVE lane, 2026-10-19).** `topo::boolean::slit` parts every
edge of a Boolean result lying in one of its faces whose material both
the edge's faces leave the line into: a ring, a slit, or a divided face
and parted ends, on the seam path and the containment fallback (the
cavity twin, a trench inside the slab resting its keel on the floor,
had the same rider form across two shells). The three extents agree
with the split on main (PR 5350 merged). Where the parting is not built
it refuses `BooleanError::EdgeInFace`, typed, never the rider form: a
tangent wall (the doubled cusp) and an arc (the curved twin) refuse
before parting. A later Boolean reads an operand's slit tip as its face's
full turn (`sectors::SlitTips`). Where another solid's edge runs along
the slit's line, or glues away its face, it refuses typed
(`SlitLineCrossed`, `SlitFaceGlued`). Filed:
`a-boolean-edge-running-past-the-face-it-lies-in-is-not-parted` (p12's
`A ∖ B`), `a-boolean-edge-lying-in-a-curved-face-is-not-read`,
`the-boolean-parting-refuses-its-rarer-configurations`,
`a-boolean-places-two-germs-along-a-slit-line` and
`a-boolean-glues-away-the-face-a-slit-lies-in`. The curved twin's
evidence is on
`a-curved-slit-fails-tier-3-prime-on-its-two-coincident-arcs`. The
editor names all three extents, the two keel edges apart.

## Note from RESTREAD (2026-10-19): tier 3′ refuses the rider form

Branch `restread/face-in-edge-wedge`: tier 3′ reads an edge lying in a
face of its own solid for sides and refuses this row's form
`FaceThroughEdgeWedge`. Every test that now expects that refusal, and
that this row's fix should turn back to passing:

- topo `a_face_inside_an_edges_wedge_refuses_at_tier_3_prime`: the
  three trenches and the glued tip subtracted with its records (pinned
  refusals; flip them to pass);
- topo `a_vertex_read_by_two_sector_passes`, the `RIDER` cells (all
  `y − x`, at every pose): the pyramid through the glued faces out the
  side, out the bottom and out a far corner; around the lying pyramid
  mirrored into the void; on the void's face in the arch, and the
  pyramid there; on the void's face with the island in it. Tests:
  `an_edge_through_two_blocks_glued_faces_builds_{at_rest,in_every_op}`,
  `a_touching_vertex_beside_a_partner_along_the_face_builds_sound_in_every_op`,
  `a_vertex_in_several_pairs_or_beside_nested_partners_reads_each_edge_once`,
  `every_scene_builds_sound_or_refuses_typed_at_every_pose`;
- topo `union_flush_onto_edge_contact::a_face_holding_the_pinch_line_ties_the_pierces_germs`:
  `cutter ∖ pinch` at slope −0.5;
- sweep `join1_r2_rand::r2_random_zprism_pairs`: 10 `∖` cases of the
  default draw, pinned as a count (`Verdict::Rider`);
- sweep `join_pierce_runs_sweep::the_sweep_subset_ships_no_bad_body`: 8
  edge and corner lines, pinned as a count (`RIDERS`);
- sweep `near_tangent_pierce_poses::near_tangent_pierce_poses_build_their_closed_form_or_refuse_typed`:
  7 runs at ε = 1e-6 and 7 at 1e-12 (`w345` and `convex` poses whose
  near-tangent edge the op decided onto the other's face, `∖` either
  way round), pinned as counts per ε (`RIDERS`); the edge and germ
  witness tests accept the same lines.
