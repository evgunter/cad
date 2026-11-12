---
id: a-row-on-a-merged-face-of-a-cuts-material-names-one-tool-face
kind: issue
title: A row naming a merged face of a cut's material is restated on the first tool face that face holds
status: open
opened: 2026-11-12
priority: P3
cost: E
refs: [a-cut-across-its-tools-seat-leaves-a-sliver-between-the-seats-two-sections]
---



Filed by SEAL's lane on `a-cut-across-its-tools-seat-leaves-a-sliver-between-the-seats-two-sections`.

## What

A cut by a tool read as its material is restated in the tool's cells (`crates/topo/src/boolean/material.rs`, `Through`). A face of the material that holds several of the tool's faces is the merge of flush faces of two pieces: the stacked plate's and base's sides in `a_tool_of_three_stacked_solids_is_one_material`. The holdings state it whole, so the editor names it `Merged`.

A row of the cut that names that face as an operand-B cell is restated on one of those faces (`Through::input_cell`): the face's own key where it kept one of the tool's, else the first face it holds. A point decided on that face's carrier lies on each tool face's carrier, since they are flush. But a row that decides the face pair itself would name one face of the several.

Not measured: whether any cut's row names such a face. `respell_through_cuts` refuses `RechartedRowUnstated` for the same shape at a sphere re-chart, where only a point's row is stated by the faces that hold it.

## Done when

A row on a merged face of a cut's material is stated by the faces that hold it, or refuses as `respell_through_cuts` does, pinned by a row whose cut decides a vertex on the merged face.
