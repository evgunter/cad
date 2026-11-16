---
id: vtxfac-sector-coplanar-tilt-beside-on-reads
kind: issue
title: check whether vtxfac's sector-coplanar tilt and its On reads are decided one at a time
status: closed
opened: 2026-10-07
priority: P2
cost: M
closed: 2026-11-10
branch: seal/corner-sectors
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

## Closed (SEAL PR 6693, 2026-11-10)

Its subject is deleted. SEAL's `a-corner-reads-only-its-own-sectors`
removes `bool_sector_coplanar`: `rg -n bool_sector_coplanar crates`
prints nothing on `seal/corner-sectors`. A flat sector of a pair the
glue door did not glue no longer reads a tilt beside its bounds' On
reads. It refuses on the steeper of its two bounds' ties
(`crates/topo/src/boolean/vtxfac.rs` `unglued_flat_sector`,
`Coincide::CornerBound`), and those ties are the readings that put the
bounds On. So there is one decision, not two decided one at a time. A
glued pair lumps by the relation the door recorded and reads no tilt.
