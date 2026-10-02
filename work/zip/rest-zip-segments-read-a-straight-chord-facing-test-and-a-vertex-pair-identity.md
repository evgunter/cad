---
id: rest-zip-segments-read-a-straight-chord-facing-test-and-a-vertex-pair-identity
kind: issue
title: The REST zip's enumerate_segments faces germs by a straight-chord test the join no longer uses, and identifies a segment by its vertex pair, so two arcs between one pair refuse ParallelSeamEdges
status: open
opened: 2026-10-02
priority: P0
cost: H
refs: [dumbbell-joint-union-leaves-four-loose-ends]
---


Found by JOIN's in-face measurement (2026-10-02, probe branch
`join/inface-probe`, switch `CAD_REST_LOOSE`), on the dumbbell of
`work/join/dumbbell-joint-union-leaves-four-loose-ends.md`.

## What

`crates/topo/src/boolean/rest.rs` `enumerate_segments` says it reuses
"the SAME mutual-facing tests as the join". The join's test is now
locus-aware (`germ_section_frame` + `germs_face_each_other`); this one
is still the straight-chord test. At the two ends of a semicircle the
tangents are perpendicular to the chord, so both margins read about
±3.7e-17 (`0.3·sin π`), in band Zero, and no segment forms. The zip
then declines, and the join's `UnpairedLooseEnds` surfaces.

With the test loosened, the zip forms two segments, both between the
same vertex pair. `realize_seam` → `fan_edge_between` finds both
semicircle edges between that pair and refuses
`RestZipUnsupported { ParallelSeamEdges }`: a `Segment` is a vertex
pair, so it cannot say which arc it means.

Two spellings of one facing rule (P1's class), and an identity too
weak for the geometry. The second is the same weakness that builds
the rod's wrong body in the JOIN measurement. Its answer may come from
JOIN's open design fork on how a section segment coinciding with an
existing edge is identified; read that before building.

## Measured (TANG, 2026-10-02, branch `tang/abutting-rim`)

Both halves of this row are addressed there for circle arcs, which TANG
needed for an abutting rim (a dome on a tube): `enumerate_segments`
first matches a germ pair along a circle arc each operand carries from
one site to the other, leaving along the germ (`arcs_along`), and the
`Segment` carries those two arcs, which `realize_seam` uses in place of
`fan_edge_between`. The straight-chord test still matches every other
pair. The dumbbell builds. JOIN-2's plan (the zip reads the join's
segments) would replace both; whether this row closes on that branch or
on JOIN-2 is the owner's call.
