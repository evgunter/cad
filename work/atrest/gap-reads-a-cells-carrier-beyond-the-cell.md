---
id: gap-reads-a-cells-carrier-beyond-the-cell
kind: issue
title: Gap reads a cell's carrier beyond the cell, so a cell beside a face reads as inside it
status: open
priority: P3
cost: M
opened: 2026-10-26
refs: [gap-reads-vertex-and-edge-cells]
---

`Gap` reads the reading cell's material as the half-spaces of its faces' carriers (`crates/editor-core/src/eval/measure.rs`, `gap`, `face_offset`, `Fold::offset`). A carrier runs past the cell's own boundary, so a cell standing beside a face, outside it but below its plane, reads as inside its material.

The witness is reviewer R1's `probe_a_vertex_beside_a_face_reads_the_extended_plane` (branch `review/atrest-gap-cells-R1` @ 8444113d). A unit cube's corner at `(1.005, 0.5, 0.99)` stands 0.005 clear of a block's `+x` side and 0.01 below the plane of its top. `Gap(top, corner)` reads `−0.01` in either seat. The block's `(1, 1)` strut's wedge reads the same corner `+0.005`.

The face–face arms read carriers the same way (C5 is "the carrier-relative signed offset"), so this is not new with the cell arms. It is harmless for quieting: a census row exists only where the two cells touch, so a `Gap = 0` over a row's cells reads them where they meet. But an author's `Gap` assertion over two cells that do not touch can read the wrong sign, which misleads.

**What a fix looks like (not decided).** Either read the cell's offset only where the other cell projects into it, refusing typed where it does not, or state the carrier-relative reading at the measure's door.
