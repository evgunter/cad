---
id: gap-reads-a-cells-carrier-beyond-the-cell
kind: issue
title: Gap reads a face's carrier beyond the face, so a cell beside a face reads as inside it
status: open
priority: P2
cost: M
opened: 2026-10-26
refs: [gap-reads-vertex-and-edge-cells, material-and-convexity-are-read-three-ways, gap-reads-no-curved-cell-below-a-face, 5827]
---

`Gap` reads the reading cell's material as the half-spaces of its faces' carriers (`crates/editor-core/src/eval/measure.rs`, `gap`, `face_offset`, `Fold::offset`). A carrier runs past the cell's own boundary, so a cell standing beside a face, outside it but below its plane, reads as inside its material.

The witness is reviewer R1's `probe_a_vertex_beside_a_face_reads_the_extended_plane` (branch `review/atrest-gap-cells-R1` @ 8444113d). A unit cube's corner at `(1.005, 0.5, 0.99)` stands 0.005 clear of a block's `+x` side and 0.01 below the plane of its top. `Gap(top, corner)` reads `−0.01` in either seat. The block's `(1, 1)` strut's wedge reads the same corner `+0.005`.

The face–face arms read carriers the same way (C5 is "the carrier-relative signed offset"), so this is not new with the cell arms. It is harmless for quieting: a census row exists only where the two cells touch, so a `Gap = 0` over a row's cells reads them where they meet. But an author's `Gap` assertion over two cells that do not touch can read the wrong sign, which misleads.

**What a fix looks like (not decided).** Either read the cell's offset only where the other cell projects into it, refusing typed where it does not, or state the carrier-relative reading at the measure's door.

## The link arms (2026-10-26, ATREST link lane)

A vertex or an edge no longer reads by its faces' carriers. Its material is its link (`topo::Link::offset`): the cone its faces bound at the point, read as a true distance from that cone. The cone's sectors run on from the point without end, so a reading far from the cell is about the cell's neighbourhood carried out (C5's cell sentence says so), not about a face plane carried past the face's edge. The witness above still stands, because a face reads by its carrier and the face arms are unchanged (`material-and-convexity-are-read-three-ways` item 3). Whether a cell's cone carried far out should read at all is part of this row too.

## A round face overreaches further (2026-10-27, #5827's review)

A sphere or cylinder face reads a vertex or an edge by its carrier (`topo::Round`, `measure.rs` `face_offset` and `face_reads_edge`). A round carrier overreaches the face in three ways a plane does not:
- **past a cylinder's end**: the carrier is infinite along the axis;
- **across the axis**: a point on the far side of a pin face that covers only part of the circle reads the pin's material there;
- **over a sphere cap's far side**: a cap reads as the whole ball.

The witness is `review_probe_an_edge_past_a_pins_end_reads_its_carrier` (branch `review/atrest-round-Nvcrvr` @ 6b7b4f5782, `crates/editor-core/tests/gap_reads_a_cell_against_a_round_face.rs`). The pin is r = 0.5 over z ∈ [0, 2]. A cube's edge at z ∈ [3, 4], 0.14 off the axis, reads `Gap = −0.359`, where the clearance to the pin is about +1.

A wrong-signed `Gap` can make an author's assertion read Holds over two cells that interfere, or fail over two that are clear. That is why this row is P2.
