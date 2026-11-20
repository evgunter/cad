---
id: three-dimensions-do-not-cover-the-sentences-already-in-the-tree
kind: issue
title: Length, angle and dimensionless do not cover the units sentences already write
status: open
opened: 2026-11-20
priority: P2
cost: E
design: true
---


Found by the review of the told-quantity build (PR 7159).

`geom_core::Dimension` is length, angle and dimensionless, as
`crates/geom-core/README.md`, "A number told to a person", ratifies.
Sentences already in the tree write other units:
`crates/geom-brep/src/offset_meters.rs` (the `MeterError::NormalFloor`
arm of its `Display`) writes "its length {} m² per unit parameter area
over speed {} m". The sweep that reaches it has to tell an area either
as a `Length` (the wrong grid: ε caps it, and the unit word is wrong) or
as `Dimensionless` with a hand-written `m²` after it (a unit word
outside the type, which the notation-taking rendering then cannot
convert).

Not searched: how many such sentences there are. The explorer's count of
150 to 200 arms on the parent row did not sort them by unit.

**The question.** Whether the dimension set grows (area, curvature, a
per-parameter rate), becomes a power of length the type carries, or
those sentences are reworded to tell lengths.
