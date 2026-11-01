---
id: census-passes-a-face-inside-an-edges-material-wedge
kind: issue
title: Tiers 3 and 3′ pass a face lying inside an edge's material wedge
status: closed
opened: 2026-10-12
priority: P2
cost: M
closed: 2026-10-19
---

Found by the keel design fork (`[ev]` PR 4795), measured on main
`360cf55eb2`.

## What

The boolean's wall-to-wall trench body of
`work/cleave/a-boolean-builds-a-reflex-edge-lying-inside-a-face.md`
puts the floor face through the keel edge's material wedge, and the
census reads nothing on it: tiers 3 and 3′ pass with no records. The
census skips every structurally adjacent pair (`sweep_vertex_edge`,
`sweep_edge_face` and the edge-edge sweep in `crates/topo/src/census.rs`
skip a shared vertex or shared face), and its only side test is for
`EdgeEdgeCross`. Compare `a_guest_seated_into_a_reflex_edge_is_not_a_rest`
in `census.rs`, which already knows a reflex edge's material reaches
across a plane. Not traced further.

## Fix

Under PR 4795's clause "no face passes through an edge's material
wedge", tier 3 (or 3′) refuses a face whose interior lies inside an
adjacent edge's wedge. Pin the boolean's three trench bodies (both ends
closed, one open, wall to wall) red until the boolean row lands.

## Note from CLEAVE (2026-10-14): the radial-edges fork on PR 4795

Both designers ask for one general check: **touching cells' material
lies on opposite sides of each other**, inside one solid as well as
between two. Concretely:
- a face lies outside an edge's wedge (this row);
- coincident distinct edges on one line have disjoint material
  sectors;
- a face lies outside a vertex's material cone.

Today the side-reading touch analysis runs only between two solids,
and inside one solid an edge-on-face record is confirmed by position
alone. A reverted slit body is the coincident-edge case: see topo's
`reverting-a-slit-body-overlaps-its-coincident-edges-wedges-and-passes-every-tier`.
The D1 sentence on 4795 names this check, so this row's scope grows
to cover it if Ev ratifies.

## Measured witnesses (topo row, 2026-10-14)

Each passes tiers 3 and 3′ on `origin/main` `ebf6c05239`, at rest:
- **Coincident edges, overlapping.** `subtract` of the slab
  `[−1,5]×[−2,4]×[0,1.5]` less the split's one-end slit body (the
  `slit_body` fixture of `crates/topo/tests/a_doubled_line_does_not_revert.rs`):
  one solid, volume 46.1875 (right), its floor carrying the keel as two
  edges each bounding about 286.7°. The same with the slab ending at
  `x = 3`, which cuts the slit (30.1125). ∖ now refuses both
  (`BooleanError::Revert`), so neither reaches the gate today; the
  check is what would catch the next door that writes one.
- **A pinch reverted.** On main, `B − pinch` in
  `crates/topo/tests/pierce_strut_at_a_pinch.rs` and `cutter ∖ pinch` in
  `crates/topo/tests/union_flush_onto_edge_contact.rs` kept each brick's
  own two faces paired on the pinch line, 270° each, and 3′ passed
  them; ∖ refuses them now.
- **A face inside an edge's wedge.** ∖ of a glued tip, the tool's
  records carried: topo's
  `subtracting-a-glued-tip-reverts-its-edge-on-face-line-into-the-rider-form`.

## The reading exists (PR 5106)

`census::line_wedges` reads, for one line, the material wedge of every
line edge on it (its two face sheets, the sector rungs for an interior
direction, and tier 3's own pinch-corner reading,
`validate::wedge_holds`, for membership) and answers whether two
wedges overlap, refusing typed (D4) where a reading is in band. Two
more spellings of a wedge's membership stand beside it,
`boolean/recl.rs` `wedge_is_reflex`/`Half` and `splitting/rules.rs`
`edge_wedge`, each with its own conventions. ∖ runs it on the lines its operand's coincident
edges name. Run over every line where coincident edges lie, it is this
row's side check for coincident edges, and it would catch the whole
class at the gate: CLEAVE's
`boolean-doors-build-overlapping-coincident-edges-from-inputs-that-hold-none`
lists Boolean results on main that pass every tier with overlaps of
40° to 180°. What it does not read is a face crossing the line with no
edge there (the rider form): that is a point-in-face question.

Unverified, from PR 5106's review diagnostic (to be measured):
`join1_r2_rand`, `join_reflex_wedge_probes`, editor-core's `emit_topo`
`touch_reread` rows, the `offer_rows` sliver cavities, and
`a_vertex_read_again_classes_every_edge`.


**Unblocked (CLEAVE, 2026-10-19).** `[ev]` PR 4795 merged (`0a702a9bc1`). Ev ratified the D1 sentence in `crates/topo/README.md` tier 3: where more than two sheets meet on a line, the material decides the edges, and the complement re-pairs. Radial edges were declined.
## Seam note from CLEAVE (2026-10-19): the Boolean gate refuses the coincident-edge class

`cleave/gate-refuses-overlapping-wedges` runs `census::line_wedges` at
the Boolean's result gate over every line `census::collinear_overlaps`
finds in the result, every op, and refuses an overlap typed
(`BooleanError::LineWedges`). So no Boolean result with overlapping
coincident wedges reaches rest. This row's side check is still owed at
tier 3′ for the doors that do not build through the Boolean: STEP
adoption admits any body tier 3/3′ passes, and `insert_voids` grafts a
record-backed or partial pair structurally (PR 5106's
`insert_voids(box, [s, s], unequal-pinch cavity)` witness). Measured
over the three crates' suites at three ε with an at-rest probe: no
other door (extrude, revolve, shell, offset, split) handed back such a
body; hand-built tier-3 fixtures and assembly copies placed on top of
each other did. The face-inside-a-wedge half (the rider form) is
untouched. The reader also refuses a cusp or slit edge on a line typed
(`WedgeRefusal::Unread(_, TouchUnread::Degenerate)`) instead of as an in-band escalation.

## Evidence from the pinch fork (CONTACTHOLD, 2026-10-20)

Ev's decision on `[ev]` PR 5395 makes a material-crescent pinch along a
tangent line two coincident cusp edges (D1 tier 3, C7), so the sign of
a tangent pinch now lives in topology as well as in the face senses.
Both designers name this row's check as what keeps them in step: a
complement that fails to re-pair leaves two coincident edges each
claiming a 2π wedge, which tier 3 reads one edge at a time and passes.
The check over coincident edges on one line, that their material
sectors are disjoint, covers the tangent case too (sectors 0 and 2π,
read at second order). Builder: CONTACTHOLD's
`a-material-crescent-tangent-line-builds-two-coincident-cusp-edges`.
That half of the check is
`tier-3-prime-reads-no-side-between-coincident-edges-on-one-line`'s.

## Closed

2026-10-19, branch `restread/face-in-edge-wedge`. Tier 3′ reads an edge
lying in a face of its own solid for sides (`census.rs`
`edge_in_own_face`, at every cell of pass 4's in-plane overlap that its
bounds back): the edge's link against the face's, through the touch
analysis's `rest::judge`. A crossing refuses `FaceThroughEdgeWedge`; a
reading that does not decide refuses `FaceThroughEdgeWedgeUndecided`,
naming why (`TouchUnread`). Arm 2 reads either as a touch inside one
solid.

- Pinned refusal: the keel in the floor, the slab less a wall-to-wall
  V-trench built as one prism with the records its contacts need. The
  Boolean's slit form of the three trenches and of the glued tip
  subtracted with its records passes, and so does a ridge resting on
  its own floor
  (`crates/topo/tests/a_face_inside_an_edges_wedge_refuses_at_tier_3_prime.rs`).
- No shipped result refuses decided. Since the Boolean parts an edge
  lying in a face (`topo::boolean::slit`, CLEAVE's
  `a-boolean-builds-a-reflex-edge-lying-inside-a-face`), the results
  that kept the keel-in-face form build the slit form or refuse
  `EdgeInFace`, and measured on the merge none reaches tier 3′:
  `a_vertex_read_by_two_sector_passes`, `union_flush_onto_edge_contact`,
  `join1_r2_rand`, `join_pierce_runs_sweep` and
  `near_tangent_pierce_poses` at ε = 1e-6 and 1e-12 carry no rider pin.
- Refused undecided, a curved face of the edge's own
  (`tier-3-prime-reads-no-side-for-an-edge-with-a-curved-face-lying-in-a-face`):
  38 results of `a_pierce_held_in_a_curved_face`'s plane-face row and
  two `rest_zip_admission` tests, presumably valid, each pinned by
  run.
- Refused undecided in band with no `CensusEscalated` beside it: pose
  `Lbot nt e0 a12` at `d = −1e-9` (ε = 1e-9) and `d = −1e-6`
  (ε = 1e-6), both orders of `prism ∪ cube`, which passed before;
  pinned per ε as `near_tangent_pierce_poses`' `UNREADS`.
- Out of this row: a face against a vertex's cone, where the literal
  corollary refuses valid cavity corners
  (`tier-3-prime-reads-no-side-for-a-face-touching-a-vertex-of-its-own-solid`),
  and coincident edges on one line at tier 3′
  (`tier-3-prime-reads-no-side-between-coincident-edges-on-one-line`).
