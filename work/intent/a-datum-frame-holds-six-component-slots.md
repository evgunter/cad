---
id: a-datum-frame-holds-six-component-slots
kind: issue
title: Datum::Frame still holds six u/v component slots, which FORK-MOVE retired (no slot holds a direction component)
status: open
opened: 2026-10-18
priority: P0
cost: H
refs: [a-pose-is-moved-by-lengths-and-angles, poses-move-by-lengths-and-angles, a-mate-placed-seat-is-unproven-at-the-doors-budget]
---

FORK-MOVE (fork log row 149, ratified on PR 5189) rules that no slot holds a direction component. Its problem statement names `Datum::Frame`'s six `u`/`v` components first. Stage 3 A′ (`poses-move-by-lengths-and-angles`, PR 5243) retired `InFrame`/`PoseCoords`, `Project`, `Standoff` and `Revolve`'s direction slots. `Datum::Frame { origin, u: [S; 3], v: [S; 3] }` (`crates/editor-core/src/node.rs`) is still on main, and the fixtures, the guide (`docs/GUIDE.md`) and the tour's parts (`demos/tour/src/assembly.rs` `prism_part`) author frames through it.

Besides D10's offer, it costs the coincidence door. Every cap normal built on such a frame is `normalize(u × v)` over six free symbols. Identities through a mate-placed copy then exceed the door's budget (`a-mate-placed-seat-is-unproven-at-the-doors-budget`). A frame moved by slides and spins makes normals trigonometric polynomials with no `sqrt` chain.

No unit names this retirement. Check stage 3 D's spec before building, in case D carries it.
