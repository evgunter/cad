---
id: revolve-pole-export-interior-on-axis-vertex
kind: issue
title: "M9/D1 revolve naming: a subdivided axis run is now representable, so the pole export's deleted-interior branch is editor-reachable"
status: open
opened: 2026-09-03
github: 1610
refs: [1573, BOOL-12]
---

## From GitHub issue 1610

Opened 2026-09-03; 0 comments.

**Found by BOOL-12's third addendum (PR [#1573](https://github.com/evgunter/cad/pull/1573), Ev's sixth-round ruling: every zero-turn joint is a declared tangent joint; the lattice never asks whether carriers are the same) — filed as the durable home; it is M9/D1's question, not S-BOOL's.**

`editor-core`'s `m9_d1_r1` probe pinned that a SUBDIVIDED axis run (an on-axis side carried by several collinear legs) was *unrepresentable* through the program layer, because the lattice refused the collinear joints. That unrepresentability was the premise for revolve naming's pole rule: "every live on-axis vertex is a run TIP, so the pole export is `Some`". Under the sixth-round ruling the continuation verbs declare their zero-turn joints and the seam takes one arrival token, so a subdivided axis run authors through the program layer, and the pole export's **deleted-interior branch (`None`)** is now reachable from the editor where the old refusal made it unreachable.

BOOL-12 renamed the row, kept its fixture, asserts that the chain now applies, and names the change; it did not decide whether that branch is RIGHT when reached from the editor (an interior on-axis vertex of a revolved profile is not a pole — it is a point on the axis between two axis segments; what the naming lane should emit for it is D1's rule to state).

**Owed:** the M9/D1 decision on what the pole export yields for an interior on-axis vertex reached through the program layer, a row per direction, and the `m9_d1` premise text re-recorded. Refs M9, D1, the revolve naming's pole elimination, BOOL-12.

## Home

`work/issues/` — the issue states it is M9/D1's question and not S-BOOL's, and the revolve-naming/M9 lane in `editor-core`'s names module is in no open program's territory globs.
