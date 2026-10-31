---
id: a-partial-revolve-by-a-fraction-of-a-turn-ends-off-its-axes
kind: issue
title: A partial revolve or a tube window by a written fraction of a turn ends off its axes by an ulp
status: open
opened: 2026-10-20
priority: P3
cost: M
refs: [5506]
---

Found by INTENT stage 3 D's exact-turn lane. A rotation fed by an angle the recipe writes as a rational fraction of a turn now takes its sine and cosine from the fraction (`geom_core::Angle`, `Turns`), in every lane: pose and world-read spins, loft station spins, a revolve's `axis_heading`, a circular pattern's steps and a placement's spins (`crates/editor-core/src/expr.rs` `eval_angle`, `crates/editor-core/src/eval/slots.rs` `angle`).

Two angle slots still cross into the sweep as radians, so their sine and cosine are the rounded radian's (`cos(π/2) ≈ 6.1e-17`):
- a revolve's sweep angle: `crates/editor-core/src/eval/wire.rs` `wire_revolve` hands `Revolution::Partial(angle)` (a `T`) to the sweep, so a quarter-turn revolve's end cap stands an ulp off the profile's plane turned;
- a tube's window: `TubeWindowStart`/`TubeWindowEnd` (`wire.rs`, the tube arm) reach the sweep as `T` too.

**Done when** the sweep takes the angle as a `geom_core::Angle` (or its sine and cosine) where the recipe writes a fraction of a turn, so a revolve by `turn/4` ends on its axes bit for bit in every lane; add that row.
