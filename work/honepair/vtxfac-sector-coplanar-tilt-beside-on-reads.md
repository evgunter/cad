---
id: vtxfac-sector-coplanar-tilt-beside-on-reads
kind: issue
title: check whether vtxfac's sector-coplanar tilt and its On reads are decided one at a time
status: open
opened: 2026-10-07
priority: P2
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

## Slate triage (2026-10-26, CLEAVE)

- Re-banded P3 → P2: a tilt decided beside bound codes read On earlier: margins decided one at a time, or one verdict read off two decisions: interval and error propagation, P2 by `work/README.md` Priority.
- Moved to HONEPAIR (`work/honepair/`) by `git mv`, id and body unchanged: the P2 track opened for one verdict decided on one margin at its own reach: rows that decide two margins one at a time, compose two readings, or lever a reading at the wrong extent. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).
