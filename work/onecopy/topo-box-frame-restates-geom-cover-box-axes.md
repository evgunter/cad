---
id: topo-box-frame-restates-geom-cover-box-axes
kind: issue
title: topo::boolean::boxes::BoxFrame restates geom::cover::BoxAxes and converts between them
status: open
opened: 2026-11-03
priority: P2
cost: E
---


Found by the frame-readings unit, which renamed `geom::cover::Frame` to `geom::cover::BoxAxes` (the axes a box is read in: the world's, or three rows).

`crates/topo/src/boolean/boxes.rs` keeps its own `BoxFrame` with the same two arms (`World`, `Aimed(rows)`), its own `aimed` constructor, and `cover_frame`, which converts one into the other at every `Cover::box_in` read. Two types for one fact, kept in step by a conversion.

Fix: `topo` reads `geom::cover::BoxAxes` and `cover_frame` goes; what `BoxFrame` adds (its `point` and `unit` readings) moves onto `BoxAxes` if `geom` has a use for it, else stays as free functions over it.
