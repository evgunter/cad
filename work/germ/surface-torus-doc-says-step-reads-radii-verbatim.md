---
id: surface-torus-doc-says-step-reads-radii-verbatim
kind: issue
title: Surface::Torus's doc says step-import reads both radii verbatim; step-import now decides them
status: open
priority: P3
cost: E
opened: 2026-11-23
---


Found by both designers on PR 7645's fork. `crates/geom/src/surfaces.rs`'s `Surface::Torus` variant docs say STEP reads the torus radii verbatim. `step-import/src/entities.rs` decides each through `positive_length`/`root_length` and the ring through `TORUS_RING`. The doc is stale; fix it, or let the `[sizes]` build rewrite it.

Signed: (GERM orchestrator)
