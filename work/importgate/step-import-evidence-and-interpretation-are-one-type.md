---
id: step-import-evidence-and-interpretation-are-one-type
kind: issue
title: step-import: SolidSpec/EdgeSpec still conflate the file's evidence with the interpreted model
status: open
opened: 2026-11-17
priority: P1
cost: M
parent: adoption-rebuilds-caches-at-eps-from-eps-in-interpretation
---

Found by `importgate/interp-layer`, which was to carry it
(`import-interpretation-layer-reads-the-file-at-its-band`, its "Two
types" bullet) and did not.

## What

`crates/step-import/src/entities.rs`'s `SolidSpec` / `FaceSpec` /
`EdgeSpec` hold both what the file states (an edge's parsed carrier and
its `t0`/`t1`, the vertex points) and what interpretation made of it (the
kernel surfaces, built by `build_axes` and made one record by
`one_record`, the normalized topology). Nothing in the types says which
is which, so every reader downstream of interpretation (`assemble`,
`adopt`, the normalizations) can read a file-derived carrier as if it
were a description. D7's interpretation stage (the parent row) wants the
file's statements as *evidence*, readable by the interpretation readings
and the move report only, and the interpreted model as a second type;
until the build unit makes every cache, edges carry their file carrier
into adoption as the one visible seam.

## Why not done with the interpretation layer

The split touches every `EdgeSpec` read (about seventy, across
`adopt.rs`, `normalize.rs`, `normalize/singularity.rs`, `one_record.rs`
and `knot_split.rs`), and `importgate/knot-place` was rewriting
`knot_split.rs`'s reads at the same time. Landing it after that unit, or
with the build unit that removes the seam, avoids two lanes re-spelling
one file's every read.

## Fix

An evidence type the resolver produces and the interpretation readings
consume (stated carriers, parameters, points, records), and the
interpreted model `assemble`/`adopt` take; the edge's file carrier
crosses as an explicitly named evidence field until the build unit.
