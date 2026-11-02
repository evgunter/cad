---
id: cylinder-sphere-reads-its-pose-on-the-ruling-sweep
kind: issue
title: Cylinder x sphere is a ruled pair still read by the 3-D seeding and accounting sweeps: it reads its pose on the ruling sweep next
status: open
opened: 2026-11-02
priority: P1
cost: M
refs: [a-near-parallel-pairs-accounting-grows-with-the-operation-region]
---


Filed by the cylinder × cylinder ruling-sweep lane (branch
`sectarms/cylinder-ruling-sweep`, 2026-11-02) as the next ruled pair.

## What

Fork row 227 (Ev on `[ev]` PR 6137) put every pair with a ruled side on
the ruling sweep: a cylinder or cone against a plane, a cylinder, a cone
or a sphere. The sweep is built for two cylinders
(`geom_brep::pair_section::ruling`, `RuledCylinders`), and its `Ruled`
trait is the shape a second pair hands it.

Cylinder × sphere still marches through `ssi::march_cylinder_sphere` →
`implicit_pair_r3`, which seeds by `exhaust::seed_r3` and closes by
`exhaust::account_r3`, the 3-D tree over the sphere's ball the decision
retires for ruled pairs. Its pose counts the loops in closed form
(`pair_section::cylinder_sphere`), but its two-loop case lists two
witnessless loops matched by winding, not by witness.

## What the change is

- A `Ruled` impl for a cylinder's rulings against a sphere: a ruling
  meets it where `t² + 2(w·a)t + |w|² − R² = 0`, `w` from the sphere's
  centre, so `Δ = (w·a)² − |w|² + R²`, the squared distance of the
  ruling from the centre against `R²`.
- `cylinder_sphere` lists the sweep's components (the sphere's ball holds
  the section, so no cut ends), each witnessed.
- `march_cylinder_sphere` marches from those witnesses
  (`ssi::march_seeds`), skipping `seed_r3` and `account_r3` because the
  pose states it, and the carry's one-to-one match closes it; the
  coverage oracle (`ruling::coverage`) gains the sphere's quadratic.

Left out of the cylinder × cylinder PR to keep that diff to one pair.
