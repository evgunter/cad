---
id: a-boolean-closes-a-slits-face-across-its-line
kind: issue
title: A Boolean whose result keeps only a slit's face on both sides of the slit's line refuses SlitLineCrossed: the join does not close the face across the line
status: dispatched
opened: 2026-10-20
priority: P1
cost: M
branch: cleave/boolean-closes-a-slits-face
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

## Fix

Close F across the line in the join: the slit's two edges, each with
its wall dropped, become one F | F edge, which the merge then removes
with the ring or slit it bounded. Then let recl place the germs there
too, and pin the void-ray unions building the box.

## Built (branch `cleave/boolean-closes-a-slits-face`)

recl places the germs on the slit's line in the union too: the
`Closes` arm of `recl_edges` and the `SlitLineGap::Closes` case are
gone (`crates/topo/src/boolean/recl.rs`, `recl_edges`;
`crates/topo/src/boolean/mod.rs`, `SlitLineGap`). Two pieces build
what it then reaches.

- **The zip keeps a strut section.** Where the result keeps only F
  about the line, one operand's section along it is a strut (an edge
  of the other's, both halves in the section face) and the other's is
  the slit's two edges. Each seam pair's loop of two dies by `kef`
  across B's edge; where B's edge is that strut, both its halves are
  paired, and the first `kef` killed the second pair's half too (the
  `y ∪ x` panic in `glue`). `kill_pair`
  (`crates/topo/src/boolean/zip.rs`, `kill_pair`, called from `glue`)
  kills across A's edge there instead, and the report names the strut
  as the surviving seam edge; both edges so paired refuses
  `ZipCorrespondence`. Either order then leaves F's halves of the two
  slit edges glued into one edge, F on both sides.
- **The output stage closes F across it.** `close_struts`
  (`crates/topo/src/boolean/slit.rs`, `close_struts`; module docs,
  *Closing*) runs first in `finish_output`
  (`crates/topo/src/boolean/ops.rs`, `finish_output`), before the
  coplanar merge, whose entry gate refused the strut (`Merge`
  `InputNotClosed`, `ScaffoldingStrutVertex`): `kev` deletes each edge
  with both halves in one face toward its free end, and a ring closed
  whole leaves a lone vertex that `mekr` to the outline then `kev`
  delete (the bridge targets the face's first cycle left), as the merge
  prunes its own dangling seam edges. The edge
  and its vertices substitute onto F in the op's lineage and naming
  (`output_substitutions`), and the strut leaves the seam edges.

It is the mirror of `merge_slivers`: there the sheets on the line are
the two walls and a sliver between them dies by `kef`; here they are F
twice and the strut between them dies by `kev`.

Measured on main (`f742e4c97b`) and head at ε = default, 1e-6 and
1e-12: the 20 void-ray unions per ε of
`a_ring_crossed_along_its_line_places_its_germs_apart.rs` (two scenes,
five poses, both orders) refuse `SlitLineCrossed` `Closes` on main and
build on head, tier 3 and 3′: "around" is the plate box v8 e12 f6 at
volume 6 (the partner holds the void), "under" v15 e24 f12, the ring
closed from the apex to the partner's edge end and kept beyond; at
sampled points along the line the result's edges are the creases the
operands' material decides, and `|x ∪ y| + |x ∩ y| = |x| + |y|`. The
test now builds every op. `REFUSING` in
`a_vertex_read_by_two_sector_passes.rs` drops "around the lying
pyramid, mirrored into the void" `x ∪ y` and `y ∪ x` (10 cells per ε),
which build sound at every pose. `PARTNERS_SLIT_GAPS` in
`a_vertex_read_again_classes_every_edge.rs` pins 0 `SlitLineCrossed`
at every ε (was 20, 18, 20); those cells build and the germ probe
reads them right, the other counts unchanged (`EdgeInFace` 8, 3, 7;
overlaps 40, 26, 35).

A sweep of the whole topo suite, the slow set included, through the
door meter (`scripts/door-tier3-meter.py`'s feature), main against
head at each ε, moves only `SlitLineCrossed` unions to bodies: 60, 56
and 60 calls at default, 1e-6 and 1e-12, in the three files above and
`a_touching_vertex_beside_a_partner_along_the_face_builds_sound_in_every_op`
(the same `REFUSING` scene). No other call's outcome or face count
moves.

No definite sub-case stays typed for this gap. Two guards stay, each
a state the join never builds: a strut with both ends free that is its
face's outline (a face of no area) refuses `JoinDesync`, and a seam
pair whose two edges are both struts of their section faces refuses
`ZipCorrespondence`. A ring closed whole on a curved face (a line
edge parted along a ruling) would take the same `mekr`, whose chord is
scaffolding killed at once; no witness reaches one.
