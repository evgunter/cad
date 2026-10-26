---
id: a-boolean-closes-a-slits-face-across-its-line
kind: issue
title: A Boolean whose result keeps only a slit's face on both sides of the slit's line refuses SlitLineCrossed: the join does not close the face across the line
status: closed
opened: 2026-10-20
priority: P1
cost: M
branch: cleave/boolean-closes-a-slits-face
closed: 2026-10-26
---


Found building `a-boolean-places-two-germs-along-a-slit-line`
(branch `cleave/recl-places-germs-along-a-slit-line`).

## What

An operand carries a slit or ring in face F: two coincident edges,
F | wall a and wall b | F. The other solid has an edge along the
slit's line, and recl places one germ per wedge pair that crosses
(`crates/topo/src/boolean/recl.rs`, `recl_edges`). Where the result
keeps only F there, on both sides of the line, it closes F across the
line: the result's material about the line is a half-turn, F
continuing, and the slit's two edges should vanish into F. The join
does not build that. Placed, the two germs leave F | F struts after the
zip, and the merge refuses its input (`Merge` `InputNotClosed`,
`ScaffoldingStrutVertex`), or, in the other operand order, the zip's
glue panics on a dead half-edge (`zip.rs`, `glue`). So recl refuses
first, typed `SlitLineCrossed { gap: SlitLineGap::Closes }` with no
lever, where every sheet the result keeps on the line is a half of F
(`recl_edges`, the `kept_slit_face` reading).

It is the mirror of `a-boolean-glues-away-the-face-a-slit-lies-in`,
where F goes and the walls stay (wall a | wall b): here the walls go and
F stays (F | F). Both are the join re-pairing the sheets of a slit's
line, and one construction should serve both.

## Witnesses

- `crates/topo/tests/a_vertex_read_by_two_sector_passes.rs`
  (`REFUSING`): "around the lying pyramid, mirrored into the void",
  `x ∪ y` and `y ∪ x`, at every pose: 10 cells at each ε. Main before
  PR 5380 built them right, the plate box v8 e12 f6 at volume 6, from
  the rider-form operand.
- `crates/topo/tests/a_vertex_read_again_classes_every_edge.rs`,
  `partners_along_the_face_build_sound_against_the_germ`: the unions of
  "void under …" with the partners along the ray, 20 cells at 1e-9 and
  1e-12, 18 at 1e-6 (`PARTNERS_SLIT_GAPS`). Main before PR 5380 built
  them in the rider form, which reads one edge where the material
  creases twice beyond the partner's edge, and which 3′ refused
  (`UnrecordedContact`).
- `crates/topo/tests/a_ring_crossed_along_its_line_places_its_germs_apart.rs`
  pins the unions' refusal in both scenes.
- The same file's
  `a_partner_flush_with_the_rings_face_builds_where_one_copy_is_kept`:
  `y ∪ x` with the partner flush with the top beside the ring, left
  and right, at rest and turned, 4 cells at each ε (5 poses: 10). It
  refused `Flush` before PR 5545; it builds once the output stage drops
  a declared surface pair the closing consumed (PR 5545), as `x ∪ y`
  does.

## Fix

Close F across the line in the join: the slit's two edges, each with
its wall dropped, become one F | F edge, which the merge then removes
with the ring or slit it bounded. Then let recl place the germs there
too, and pin the void-ray unions building the box.

## Built (branch `cleave/boolean-closes-a-slits-face`)

recl places the germs on the slit's line in the union as in every
other op: `recl_edges` has no `Closes` arm and `SlitLineGap` no
`Closes` case (`crates/topo/src/boolean/recl.rs`, `recl_edges`;
`crates/topo/src/boolean/mod.rs`, `SlitLineGap`). Two pieces build
what it reaches.

- **The zip keeps a section strut.** Where the op drops both faces of
  one operand's edge along the line, that edge's section loop runs it
  both ways, so both its halves pair against the other operand's two
  coincident edges there, a slit's or ring's. `kill_pair`
  (`crates/topo/src/boolean/zip.rs`, `kill_pair`, called from `glue`)
  kills each pair's loop of two across B's edge, or across A's where
  B's is that strut, since a `kef` across the strut kills the other
  pair's half with it. The strut survives between the faces the other
  edges bound beyond the line: F twice where the result keeps the
  slit's face on both sides, the two walls (a reflex edge) where it
  keeps them. `ZipReport` names it among its `struts`, and as the
  pair's seam edge where it is B's; a pair of two struts refuses
  `ZipCorrespondence`.
- **The output stage closes F across it.** `close_struts`
  (`crates/topo/src/boolean/slit.rs`, `close_struts`; module docs,
  *Closing*) runs first in `finish_output`
  (`crates/topo/src/boolean/ops.rs`, `finish_output`), before the
  coplanar merge, whose entry gate refuses a strut. It takes the struts
  the zip reports and deletes each that one face lies on both sides of
  with its free end, a ring closed whole with its lone vertex too,
  through the delete the merge runs on its own dangling seam edges
  (`crates/topo/src/strut.rs`, `Body::plan_strut_prune`,
  `Body::prune_strut`). The edge and its vertices substitute onto F in
  the op's lineage and naming (`output_substitutions`), and the strut
  leaves the seam edges. Any other strut reaching the output stage
  meets the merge's entry gate, which refuses it.

It is the mirror of `merge_slivers`: there the sheets on the line are
the two walls and a sliver between them dies by `kef`; here they are F
twice and the strut between them dies by `kev`.

### Evidence

At ε = default, 1e-6 and 1e-12, against main (`f742e4c97b`):

- `a_ring_crossed_along_its_line_places_its_germs_apart.rs`,
  `an_edge_along_a_ring_places_its_germs_apart_and_the_union_closes_the_ring`:
  the 8 unions per ε (two scenes, at rest and turned, both orders),
  which refuse `SlitLineCrossed` `Closes` on main, build tier 3 and
  3′. "around" is the plate box v8 e12 f6 at volume 6 (the partner
  holds the void); "under" is v15 e24 f12, the ring closed from the
  apex to the partner's edge end and kept beyond. At sampled points
  along the line the result's edges are the creases the operands'
  material decides, and `|x ∪ y| + |x ∩ y| = |x| + |y|`.
- `a_partner_grooved_along_the_ring_unites_in_either_order` (same
  file): a box straddling the top less a pyramid lying in it, whose
  reflex edge runs along the ring, ∪ the plate less the void. `y ∪ x`
  panics on main in `glue` ("half-edge … does not resolve"); both
  orders build v24 e36 f17, tier 3 and 3′, the strut kept between the
  void's walls. The claims review of PR 5544 counts 19 such cells at
  rest and 11 turned in its probe.
- `a_vertex_read_by_two_sector_passes.rs`: `REFUSING` holds no
  `SlitLineCrossed` row; "around the lying pyramid, mirrored into the
  void" `x ∪ y` and `y ∪ x` (10 cells per ε) build sound at every pose.
- `a_vertex_read_again_classes_every_edge.rs`: `PARTNERS_SLIT_GAPS`
  pins 0 `SlitLineCrossed` at every ε (20, 18, 20 on main); those cells
  build and the germ probe reads them right, the other counts as on
  main (`EdgeInFace` 8, 3, 7; overlaps 40, 26, 35).
- The whole topo suite, the slow set included, through the door meter
  (`scripts/door-tier3-meter.py`'s feature), main against head at each
  ε: only `SlitLineCrossed` unions move, to bodies, 60, 56 and 60 calls,
  in the three files above and
  `a_touching_vertex_beside_a_partner_along_the_face_builds_sound_in_every_op`
  (the same `REFUSING` scene).

### What stays typed

- Where the partner's edge along the slit ends inside it, `x − y`
  refuses `LineWedges` `Overlap` at the result gate where it refuses
  `Closes` on main (ring-groove short; the keel ring's box less the
  wide vee, `half`):
  `a-difference-whose-partner-ends-inside-a-slit-overlaps-its-wedges`.
- A face closed across a strut that is not a plane refuses
  `CurvedFaceClosesAcrossSlit`: the lone ring's bridge is a line chord,
  which lies in no curved face. No witness reaches it.
- A face closed across part of a slit, the slit's edges running on past
  both ends of the strut, refuses `SlitFaceClosesMidSlit`: deleting the
  strut parts the slit in two (a `kemr`), which is not built. Reached by
  `x − y` of the slab less the keel with a box under the floor less a
  wide vee along part of the keel
  (`a-partner-along-part-of-a-slit-refuses-result-invalid`), pinned by
  `a_keel_on_the_floor_subtracts_as_a_slit::a_face_closed_across_part_of_the_slit_refuses_typed`.
- Guards on states the zip never builds: a strut with both ends free
  that is its face's outline refuses `JoinDesync`; a seam pair of two
  struts refuses `ZipCorrespondence`.

Refusals the same on main, found by the review's probe and filed:
`a-partner-along-part-of-a-slit-refuses-result-invalid` and
`a-union-touching-along-a-rings-line-fails-tier-3-prime`.
