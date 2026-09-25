---
id: a-touch-at-a-two-fan-vertex-refuses-unanalysed
kind: issue
title: A touch at a vertex where two fans of one solid meet refuses TouchUnanalysed, because the census's cone analysis reads one vertex orbit
status: open
opened: 2026-09-25
priority: P3
cost: E
---


Filed by CONTACT-1, as a class its analysis refuses by name. The
census's touch analysis reads a vertex's material cone from ONE orbit
(`crates/topo/src/census.rs`, `Cone::vertex`, from the vertex's
`emanating` half-edge). A pseudomanifold vertex where two fans of the
same solid meet has half-edges that orbit does not visit; the analysis
counts the half-edges leaving the vertex against the orbit and, on a
mismatch, refuses the touch as `Undecided::TouchUnanalysed` rather
than reading half the material. The fix reads every fan (one orbit per
unvisited half-edge) and takes the cone as their union; the complement
and separating-plane tests then run per fan. No row exercises it yet —
the first fixture is two blocks of one solid sharing a corner, touched
there by a third part. Difficulty S.
