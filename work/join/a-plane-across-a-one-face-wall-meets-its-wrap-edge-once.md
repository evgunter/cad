---
id: a-plane-across-a-one-face-wall-meets-its-wrap-edge-once
kind: issue
title: A transverse plane across a one-face closed wall meets its wrap edge at one point, and the join refuses SingleSiteSectionLoop; not a coincidence, so not held by D10
status: open
opened: 2026-10-06
priority: P1
cost: H
branch: join/wrap-edge-section-loop
pr: 4345
---


Split out of `closed-in-face-section-loop-has-one-site`. That row is parked on D10 because its original case, a conic lying *in* the partner's face, is a coincidence verdict, and coincidence is D10's ground. The cases appended to it later are a different class: a TRANSVERSE plane crossing a one-face closed wall. The section circle crosses the wall's single wrap edge at one point, there is no coincidence anywhere, and `boolean/join.rs` refuses `SingleSiteSectionLoop` structurally, because a record whose two germs name one locus on both operands can only match itself. This row is that class, and it is not held.

- **On main today:** a full-revolved tube under a box refuses this way (CLEAVE), because full-revolve walls already wrap `u` with one seam.
- **PATHS unit 3 (#4169):** a one-segment circle's extruded wall refuses a slab or pocket cut across it in the same way.
- **PATHS unit 4** (`circle-lowers-to-one-segment`) would make this every slab or pocket floor through a circular boss, so unit 4 is blocked on this row.

**Shape of the arm.** The split already handles the analogous case: `d9244fd60` treats a self-loop chord across a full revolve's seam as the whole section conic.

**Who found it.** Both designers on the PATHS one-segment-seam fork (`[ev]` PR #4175).

## Built (branch `join/wrap-edge-section-loop`)

- **The segment.** `boolean/join.rs` `wrap_site_segments` reads the
  records the matching leaves whole (`join_segments`). It takes a
  one-site record (`one_site`) as one segment from the site round its
  whole conic back to the site, its two ends the record's two slots,
  when both germs lie inside a face on both operands and the site is,
  by `site_cell`, a wrap edge of one operand's face and a pierce of the
  other's planar face. Anything else keeps `SingleSiteSectionLoop`:
  among them a conic lying along an operand edge, a pierce of a curved
  face, and a site on both operands' wrap edges. Where a record is
  taken, another record on its locus pair, B loci unlike A's, and
  germs that agree in sense are `JoinDesync`. The declared-REST lane's
  `section_segments` read is untouched.
- **The wall side** is the split's self-loop chord (`d9244fd60`): two
  `mef`s at the site's two copies, each the whole conic.
- **The planar side** is a pierce ring holding only the site's null
  edge. `chord_join::ChordJoiner::join_lone_ring` handles it:
  - `mfkrh` promotes the ring to a face, with the old face's sense bit;
  - the two chords are each the whole conic (`bool_planar_chord_spec`'s
    `ArcEnd::WholeTurn`), and each walls off a one-conic face;
  - the face whose conic winds against the old face's outer loop goes
    back to it as a ring (`kfmrh`), the hole;
  - the other is the disc inside the conic, which takes the rings the
    conic encloses;
  - the promoted face keeps both halves, and is the null face.
- **The germ normal's reach** for such a segment is the ball through the
  site and its antipode about the conic's centre. A ball of one point
  had no arm wherever the partner plane's origin was the site.
- `cut_pair` runs once per record when a segment's two ends are one
  record.
- **The seam zip reads a one-vertex seam's sense**
  (`boolean/zip.rs` `one_vertex_sense`, `bool_zip_one_vertex_sense`).
  `align`'s vertex test cannot tell a co-wound ring half on a seam that
  closes at one vertex. Without this, a wrong hole choice on the second
  operand was zipped onto the first operand's conic and passed.

Witnesses (`crates/sweep/tests/a_plane_across_a_one_face_wall.rs`, ∪, ∩
and both differences, each in both operand orders, through
`differential::outcome` and `check_mesh`):
- CLEAVE's tube under a box;
- a slab across a one-segment cylinder;
- a blind pocket in a plate, cut by the cylinder;
- a tilted plane;
- planes 0.02 and 0.001 above the wrap edge's end vertex;
- the tube under a box whose side face cuts the outer wall, so the
  inner wall's one-site loop lies beside matched segments in the same
  faces;
- refusing `SingleSiteSectionLoop { count: 1 }` in every op: a circle
  edge lying in the box face (the in-face conic), and a plane tangent
  to the cap at the wrap edge's end vertex.

`one_segment_loop.rs` `a_boolean_on_an_extruded_seam_wall_builds_along_and_across_it`:
the two rows that pinned `SingleSiteSectionLoop`, the slab kept and the
slab cut away, now build, to `π/2` and `3π/2`.

`germ_coplanar_conic.rs`: the fixture's loops are transverse, and it now
builds (`closed-in-face-section-loop-has-one-site`, last section).
