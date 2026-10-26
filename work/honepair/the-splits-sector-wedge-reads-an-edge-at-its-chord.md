---
id: the-splits-sector-wedge-reads-an-edge-at-its-chord
kind: issue
title: The split's sector wedge reads an edge's two faces at the edge's chord, not their extent
status: open
opened: 2026-10-15
priority: P2
cost: M
---


Filed by TANG U-lever (`tang/u-lever-the-wedge-is-levered-at-the-faces-extent`).

`splitting::rules::edge_wedge` asks whether the edge leaving a vertex
along `he` is a corner between its two faces
(`classify_dihedral(s_own, s_mate, p, extent, band)`, then
`enters_material` at the folded arm) with `extent = along.norm()`, the
edge's chord. Every other reader of an edge's wedge (check 4,
certification, the must-carry rule, the boolean's seam description and
joinability, the shared-rim routing) now levers it at the farther face's
extent (`topo::wedge_reach`). A short edge between large faces at a
small dihedral therefore reads Smooth here where those read it a corner,
and the sector takes the smooth arm for an edge tier 3 calls a corner.
Fix: lever at `crate::wedge_reach` of the edge's two faces (the edge is
certified at that point; a null scaffold keeps a typed refusal). No
fixture is known to reach the difference.

## Slate triage (2026-10-26, CLEAVE)

- Moved to HONEPAIR (`work/honepair/`) by `git mv`, id and body unchanged: the P2 track opened for one verdict decided on one margin at its own reach: rows that decide two margins one at a time, compose two readings, or lever a reading at the wrong extent. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).
