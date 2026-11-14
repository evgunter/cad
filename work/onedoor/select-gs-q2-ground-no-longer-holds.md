---
id: select-gs-q2-ground-no-longer-holds
kind: issue
title: SELECT-DESIGN's GS-Q2 ruled against an edge-convexity selector atom on the ground that the kernel read one point of an edge; the wedge fold now gives a whole-edge verdict, so the ground is gone and the ruling is Ev's to revisit
status: open
opened: 2026-11-14
priority: P3
cost: E
design: true
---


Filed by the ONEDOOR orchestrator on 2026-11-14, from wedge fold stage 3 (PR 6913).

`docs/SELECT-DESIGN.md` GS-Q2 decided that the edge-convexity selector atom is not built. Its ground, at `:276`, was: "the kernel predicate reads one point of an edge (`geom_brep::classify_edge_wedge`), and a selector atom needs a whole-edge verdict, and the demand evidence is a comment rather than a request."

PR 6913 retired `classify_edge_wedge`. `topo::edge_wedge` now reads every interior station and returns a whole-edge verdict, so the first part of that ground no longer holds. The PR kept the ruling and moved its ground into the past tense ("read one point of an edge when this was ruled"). That rewording describes the code change; the decision itself is untouched.

The remaining ground is the demand evidence. Whether that alone holds the ruling, or whether a selector atom over `topo::edge_wedge` should now be built, is a design question and Ev's. It goes to Ev as an `[ev]` PR on `docs/SELECT-DESIGN.md` when a program picks it up.
