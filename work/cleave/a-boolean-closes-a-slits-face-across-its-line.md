---
id: a-boolean-closes-a-slits-face-across-its-line
kind: issue
title: A Boolean whose result keeps only a slit's face on both sides of the slit's line refuses SlitLineCrossed: the join does not close the face across the line
status: open
opened: 2026-10-20
priority: P1
cost: M
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
first, typed `SlitLineCrossed` with no lever, where every sheet the
result keeps on the line is a half of F (`recl_edges`, the
`kept_slit_face` reading).

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
