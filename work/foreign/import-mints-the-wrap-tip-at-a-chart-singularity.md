---
id: import-mints-the-wrap-tip-at-a-chart-singularity
kind: issue
title: import's pole form flips to the wrap tip: geom::pole_form mints one wrap edge to each singularity, DegenerateApexCone's two-half form retires, OCC's seamed cap and cone import as written
status: parked
opened: 2026-10-29
priority: P1
cost: E
blocked_on: [a-full-revolves-curved-wall-carries-two-seam-meridians-nobody-wrote]
---


Ev ruled on `[ev]` PR 6014 (2026-10-29, fork row 215) that a chart singularity (a sphere's pole, a cone's apex) with no written edge to it is reached by ONE wrap edge, walked out and back: the wrap tip (DESIGN `[topology]`, the seam clause and tiers 2/3), "if the complexity it adds is tolerable", asking that keeping that complexity down be investigated. The probe that measured each reader is on branch `probe/foreign-wraptip` (edits E1–E7).

PR 6007 built import's door minting the two-half form from `geom::pole_form`, its one home. Owed: `pole_form` mints the wrap tip; the three dispositions keep their detection (interior, inside an edge, a valence-1 vertex) and mint one wrap edge; a valence-1 vertex on one meridian (OCC's `seamed.step`, `freecad/cone_apex.step`) imports as stated; `EdgeFreeSphere` mints one vertex at each pole, one meridian, one face. Lands after the tier rules admit a tip and revolve builds one band, so import and native construction agree.

Signed: (FOREIGN orchestrator)
