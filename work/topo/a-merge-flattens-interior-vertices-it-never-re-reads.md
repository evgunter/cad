---
id: a-merge-flattens-interior-vertices-it-never-re-reads
kind: issue
title: A face merge deletes the interior vertices of the faces it merges without re-reading them against the kept description, so an interior drift past ε may be flattened silently
status: open
opened: 2026-11-02
priority: P1
cost: M
---

Filed by the REFS orchestrator (2026-11-02). It comes from a research lane answering Ev's question on PR 6342: can a chain of near-coincidences be snapped by more than ε?

**Unconfirmed; probe first.** `Body::merge_coplanar_faces_staged` (`crates/topo/src/merge_faces.rs`) groups faces by adjacent-pair continuation and grows each group transitively. `kept_description` then keeps one constituent's surface, and the pruning step deletes the vertices left inside the merged face (`killed_vertices`). Only the merged face's BOUNDARY edges are re-described and certified afterwards (`describe_edges` in `crates/topo/src/boolean/ops.rs`). No check is made that every constituent, or every deleted vertex, lies within ε of the kept description.

So on one body whose interior facets drift (a terraced dome inside a flat ring, say), the boundary would certify while the interior is flattened by more than ε, with no refusal. That would break `[intrinsic.absolute]`. Within a union, the fold's per-member `VertexOnFace` read should catch it for each joining member; the standalone merge op has no such read.

**First job:** build the terraced-dome fixture and measure. If the merge flattens past ε silently, this is a live wrong answer: re-band it P0, and add the missing read (each constituent's vertices within ε of the kept description, refusing typed otherwise). If it refuses, close the row with the measurement.

Related: `work/contacthold/a-fold-step-glues-a-pair-its-door-reads-in-band.md` (the chain probe's measured cases); probe on branch `reads/chain-drift-probe`.
