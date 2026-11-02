---
id: a-rigid-motion-rewrites-and-recertifies-every-edge
kind: issue
title: transform_rigid rewrites every coordinate of a finished body and re-certifies every edge, where an isometry of a certified body could stay certified or be carried as a placement
status: open
opened: 2026-11-02
priority: P1
cost: M
design: true
---

Filed at Ev's request on PR 6329 (2026-11-02): "this sounds pretty inefficient though maybe it's unavoidable? i'd have hoped it would fall into the general 'pretend float noise doesn't exist' case. also wait doesn't transform_rigid keep the same underlying representation anyway".

## What

`topo::transform_rigid` (`crates/topo/src/transform.rs`) returns a new body with the same topology and arena keys. Every point, surface and curve is rewritten through the motion, each rounding independently, and then every edge carrier is re-certified against its mapped endpoints and surfaces (`map_isometry`, then `mint_pcurves`). Its `# Panics` documents a refusal there as a kernel bug. So a rigid motion:
- pays a full re-certification;
- can panic on valid input near ε_min when it carries the body out of the session box (PR 6329's discussion).

## The question

Ev's principle for f64 (PR 6329): make operations well-conditioned, then treat values as reals and ignore float error beyond that. An isometry is perfectly conditioned. Two final states to weigh, as a design question:
- **(a)** A rigid image of a certified body is certified, so nothing is re-read.
- **(b)** The motion is carried as a placement (DESIGN `[placement]`, "a moved body is a placement") and the representation is not rewritten at all.

Read with: PR 6329's outcome on charges and the session box, and `the-at-rest-result-gate-and-its-typing-live-in-five-homes` (rigid placement is a door returning the finished type).
