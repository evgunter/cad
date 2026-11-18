---
id: an-imported-edge-on-a-pair-the-c5-table-does-not-build-refuses
kind: issue
title: An imported edge between two surfaces whose section the C5 table does not build refuses not-yet-supported
status: open
opened: 2026-11-18
---



Found by `importgate/build` (PR 7128).

## What

The build constructs an edge's carrier between two distinct surfaces
through `topo::section_near` (the C5 closed forms: a plane against any
quadric, cone × coaxial cylinder, sphere or torus) and, for the other
pairs, the pair section's classified-and-carried component
(`geom_brep::pair_section`). A pair neither serves, or a section that
answers a verdict the build reads no carrier from (closed, no branch, no
component through the printed middle), refuses
`StepImportError::NotYetSupported` naming this row (`SECTION_LANE` in
`crates/step-import/src/build.rs`). No file of the committed corpus
reaches it at PR 7128's head; the coaxial cone × torus arm
(`geom_brep::cone_torus_section`) was added there because the demo
tour's five-wall sleeve did.

## What would close it

A section construction for each pair a file states that the kernel does
not build: the C5 table's arms and the general rung's carry for the
pairs it leaves.
