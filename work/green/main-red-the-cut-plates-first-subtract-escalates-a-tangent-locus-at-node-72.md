---
id: main-red-the-cut-plates-first-subtract-escalates-a-tangent-locus-at-node-72
kind: issue
title: main red (nightly tour): tolerance::the_cut_plates_first_subtract_refuses_a_bores_u_ref_over_the_study escalates Coincidence(TangentLocus, Moot) at node 72, with no open row
status: open
opened: 2026-11-26
priority: P1
---


Filed by the LATHE orchestrator. LATHE's one-band lane measured this, running each red row alone while landing PR 7414.

- **The row:** `demos/tour/tests/tolerance.rs`, `the_cut_plates_first_subtract_refuses_a_bores_u_ref_over_the_study`, a nightly tour row (`--cargo-profile ci-tour`).
- **On main `5e04ac6561`:** red. The first Subtract escalates `Escalated{Coincidence(TangentLocus, Moot)}` at node 72 (`cc_parallel`), where the row expects its refusal.
- **On LATHE's merged head:** red the same way.
- **On LATHE's stack before the main merge (`a8a9d3a9b6`):** green.

No open row names it. `work/ciw/a-slow-row-added-to-a-demo-suite-runs-nowhere-on-the-pr-that-adds-it.md` explains why no PR saw it: demo slow rows run only nightly. The file's recent changes are the directed-segment fix passes (29d77acbb8, 5a19addabc, 2026-11-23).
