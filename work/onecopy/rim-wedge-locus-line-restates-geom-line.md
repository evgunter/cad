---
id: rim-wedge-locus-line-restates-geom-line
kind: issue
title: rim_wedge's Locus::Line restates geom::Line, and spells two radial offsets inline; left for PR 6443
status: open
opened: 2026-11-03
priority: P2
cost: E
---


Found by `local-point-direction-line-types-restate-geom-line`, whose other types the frame-readings unit converted. `crates/topo/src/boolean/rim_wedge.rs` `Locus::Line { origin, dir }` restates `geom::Line`, its `at` and parameter readers restate `Curve3::eval` and `Line::param`, and two radial offsets there (`d − dir·(d·dir)`, the rim's reading and the wedge's) restate `geom::Line::offset`.

Left untouched there because PR 6443 (ONEDOOR) rewrites `rim_wedge.rs`; it goes with `rim_wedge::Rim` (`circle-frame-structs-re-spell-geom-circle`) once 6443 lands.

Fix: `Locus::Line(geom::Line<T>)`, read through the line; the two offsets through `Line::offset`.
