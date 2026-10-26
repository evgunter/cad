---
id: gap-reads-vertex-and-edge-cells
kind: unit
title: Gap reads vertex and edge cells, so an assertion quiets an at-rest contact at whatever dimension its cells are
status: open
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
