---
id: a-boolean-places-two-germs-along-a-slit-line
kind: issue
title: A Boolean whose other solid has an edge along an operand's slit or ring line refuses SlitLineCrossed: recl does not place two germs apart along the slit's line
status: open
opened: 2026-10-20
priority: P1
cost: M
---


Found by the review of PR 5380 (`cleave/boolean-builds-the-slit-form`),
which builds D1's slit form for an edge lying in a face, so an operand
may hold a slit or a zero-area ring: two coincident distinct line edges
in one face.

## What

When the other solid has an edge along the slit's line, three sheets
meet on that line, and the vertex pair at the slit's tip reads two
edge-edge germs along one ray (`crates/topo/src/boolean/recl.rs`,
`recl_edges` and `resolve_edge_edge`). The insertion does not place two
germs apart along a line, and recl refuses
`CrossingsAlongAContactLine`. That refusal's recourse names "the
pieces", which a slit is not made of, so the reduction retypes it
`BooleanError::SlitLineCrossed` (`crates/topo/src/boolean/mod.rs`,
`slit_line_refusal`). This is a construction gap with no lever.

On main the operand held the slit's line as one edge, wall | wall, lying
in its face (the rider form), and these cells built. They regress to a
typed refusal on PR 5380. Witnesses, all at the default ε:

- `crates/topo/tests/a_vertex_read_by_two_sector_passes.rs` (`REFUSING`):
  "around the lying pyramid, mirrored into the void", all six ops at all
  five poses. That is 30 cells: 20 built on main, and 10 refused there
  `LineWedges`.
- `crates/topo/tests/a_vertex_read_again_classes_every_edge.rs`,
  `partners_along_the_face_build_sound_against_the_germ`: the partners
  "void under φ 250 …" against "under the ray", "on the ray …". That is
  60 cells: 40 built on main and read right by the germ probe, and 20
  refused there `LineWedges`.

The census reads the related touch the same way. Two solids of a result
touching along a slit's line, where the other holds an edge, give tier
3′ `CensusUndecidable` `TouchUnanalysed`. The results are right by
material, volume and tier 3, so the test admits it (`SLIT_TOUCH` in
`a_vertex_read_by_two_sector_passes.rs`).

## Fix

Read the slit's two edges as one line with a full-turn face round each
tip (`sectors::SlitTips`, which the sector reader already uses), so the
other solid's edge along the line meets one material reading. Or place
the two germs apart along the line, one per coincident edge. Then
remove `SlitLineCrossed` and the `REFUSING` rows that name it.
