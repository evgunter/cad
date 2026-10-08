---
id: vtxfac-sector-coplanar-tilt-beside-on-reads
kind: issue
title: check whether vtxfac's sector-coplanar tilt and its On reads are decided one at a time
status: open
opened: 2026-10-07
priority: P3
cost: M
---

## What

Investigation, found by PR 4280's sweep and not traced to a served
verdict there. `topo::boolean::vtxfac` decides `bool_sector_coplanar`
(a tilt, `crates/topo/src/boolean/vtxfac.rs:258`) beside sector bound
codes read On earlier, where the vertex's position comes from those
side reads.

## The shape of a fix

Trace whether a vertex whose position reads On just inside the band
and whose tilt reads Zero just inside it reaches a served verdict; if
it does, decide their sum.
