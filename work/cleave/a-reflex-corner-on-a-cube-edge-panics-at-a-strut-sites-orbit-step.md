---
id: a-reflex-corner-on-a-cube-edge-panics-at-a-strut-sites-orbit-step
kind: issue
title: A reflex corner on a cube edge panics in insert::orbit_step_at where main refused before 3a0eda54
status: open
opened: 2026-10-04
priority: P0
cost: M
refs: [boolean-strut-anchor-splices-at-an-unproven-next-mate-step]
---


## What

Found by `join/pierce-pinch-families` when it brought main `79aafa8`
in: `join_pierce_runs_sweep::pierce_runs_battery` (`#[ignore]`, so no
CI row runs it) panics at its 595th line, the edge placement
`i = 0, j = 3, psi = 4`, prism ∪ cube, on main alone (measured, release):

```
panicked at crates/topo/src/boolean/insert.rs (orbit_step_at):
the orbit step from HalfEdgeKey(31v1) at VertexKey(10v1) lands on
HalfEdgeKey(20v1), which starts at VertexKey(27v1): every public door
keeps the body tier-1-valid, where every such walk closes
```

The stack runs `boolean_reduce_declared_strategy` → `insert::mint_plans`
→ `orbit_step_at`. Main at `81dde823` refused the same op typed
(`Euler(SelfLoopEdge)`), and refused or built the pose's other five ops.
`3a0eda54` routed a holder-less strut corner's first step through
`orbit_step_at` (`boolean-strut-anchor-splices-at-an-unproven-next-mate-step`),
which panics where the step leaves the site vertex. Here the operand
bodies are fixtures that pass the at-rest gate, so the step's
"tier-1-valid ⇒ the step stays at the vertex" premise is false for this
corner: a mid-reduction body whose strut site's next-mate step
legitimately reaches another vertex, or a site read from the wrong half.
A panic on normal geometry is a crash where a typed refusal stood.

The battery aborts at that line, so every later pose is unmeasured on
main until this is fixed.

## The shape to give

Reproduce with `cargo test -p sweep --release --test all
pierce_runs_battery -- --ignored --nocapture` and read which strut the
step starts from at `v`. If the walk is right and the premise wrong,
the step is a refusal (or the site another half); if the premise holds,
find the door that tore the body. Then re-run the battery.
