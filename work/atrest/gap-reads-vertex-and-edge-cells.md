---
id: gap-reads-vertex-and-edge-cells
kind: unit
title: Gap reads vertex and edge cells, so an assertion quiets an at-rest contact at whatever dimension its cells are
status: closed
closed: 2026-10-26
branch: atrest/gap-reads-vertex-and-edge-cells
opened: 2026-10-26
priority: P0
cost: M
refs: [the-at-rest-census-is-a-check]
---

DESIGN `[assertions.quiet]` quiets a contact when a holding `Gap = 0` is taken between the finding's two cells, "at whatever dimension they are (two faces, a vertex or an edge on a face, an edge on an edge, two vertices)". `Gap`'s door admits faces only (`MeasurePrimitive::admits`, `crates/editor-core/src/measure.rs`: `Self::Gap => matches!(kind, K::Face)`). So no assertion can yet be written over a lower-dimensional at-rest row, and stage 5 C leaves such rows loud (`docs/INTENT-STAGE5-SPEC.md` §4 and §11 Q2). This is a scheduled gap. It is not a frontier.

**What to build.** C5's signed gap (`crates/topo/README.md`, CONTACT-DESIGN C5) gets an arm per cell pair:
- **vertex–face:** the face's signed distance to the vertex, along its outward normal;
- **edge–face:** the edge's signed distance to the face;
- **edge–edge:** the signed distance between the two edges;
- **vertex–vertex:** their separation, signed by what each cell's body reads at the other.

Each arm is zero exactly at the contact, and its sign follows C5's strata. `MeasurePrimitive::admits` admits `Edge` and `Vertex` for `Gap`, and `admitted()`'s wording follows. The selection kinds already exist (`[operations]`, *Selections*). Fix the sign convention for each pair so that a contact reads `= 0`, and a pair whose sign has no meaning refuses typed at the measure's door rather than reading unsigned.

**The seam.** `measure.rs` is ATREST's own ground (on its `paths` since FLUXHOLD closed, `docs/doc-ledger/fluxhold-leaves-the-tracker.md`). Its evaluation, `crates/editor-core/src/eval/measure.rs`, is TALLY's (`crates/editor-core/src/eval/*`, `work/tally/program.md`), so the PR announces the seam there.

The unit can be built beside C, because C's contact half is cell-generic. Done when a vertex resting on a face and an edge lying on a face, between two arrivals of a `⊔`, are each quiet under a holding `Gap = 0` over their two cells and loud under `Gap ≥ 0`.

## Closed

`MeasureVerb::admits` admits a face, an edge or a vertex for `Gap`, and `gap` (`crates/editor-core/src/eval/measure.rs`) has an arm per cell pair. A face reads a vertex or a line edge in either seat by its outward normal. An edge reads a vertex in either seat by its wedge. Of two edges or two vertices, the outer's wedge or corner reads the inner. The module's table states each convention. `a_vertex_and_an_edge_resting_on_a_face_read_their_gap` (`crates/editor-core/tests/gap_reads_vertex_and_edge_cells.rs`) is the acceptance: the prism's apex edge and its apex vertex, each resting on a block between two copies, hold `Gap = 0` at rest and read `+0.01` and `−0.01` when lifted and sunk, in either seat. The quieting half (quiet under `= 0`, loud under `≥ 0`) is stage 5 C's contact half and is not built here. The cells this convention cannot read refuse typed and are filed as `gap-reads-no-curved-cell-below-a-face`. A cell read past its own extent is `gap-reads-a-cells-carrier-beyond-the-cell`, and the three readers of a cell's material are `material-and-convexity-are-read-three-ways`. The contact half that must quiet a vertex-on-face and an edge-on-face row is scheduled on `the-at-rest-census-is-a-check` and in `docs/INTENT-STAGE5-SPEC.md` §6 row 12.
