---
id: the-cone-closure-decide-is-a-fifth-spelling-of-the-periodic-chart-question
kind: issue
title: The cone's loop closure and winding are decided apart from the periodic-chart closure decision
status: open
opened: 2026-10-13
priority: P3
cost: M
---


Found by the FLUX periodic-closure unit's sweep
(`flux/periodic-closure-one-decide`). That unit made the cylinder and
torus closed forms, the cylinder quadrature and the spiric wall decide
"a loop closes and winds as a boundary does" through one decision
(`crates/geom-brep/src/props/closure.rs`): per junction, a chart gap
less whole periods against a stated slack, then integer windings held
to `|k| ≤ 1` and `Σk = 0`, refusing `ChartLoopOpen` / `LoopsWindChart`.

The cone's azimuth is periodic too, and its lanes still ask the same
question their own way, in `crates/geom-brep/src/props/curved.rs`:

- `cone_loops` decides `props_loop_closed` per junction in space, with
  no stated slack (`Premise::Inventory`, so an open loop refuses
  `NotIsoRectangle` or `ConeLoop`, not `ChartLoopOpen`);
- `props_cone_loop_winding` and `props_cone_ring_contractible` read a
  loop's turns about the axis at half-turn thresholds, which is the
  shape of `closure::loop_turns`, without the face-boundary integer
  checks.

Fix shape: build the cone's `closure::ChartLoop` (the turn about the
axis at each junction, levered at the after-point's distance from the
axis, beside the slant gap) and read its windings from
`closure::closed_form_windings`, keeping what is the cone's own (the
apex, the nappe). Rows: an open cone loop and a two-turn pair refusing
as the cylinder's do, and a junction open by `2ε` measuring.
