---
id: ssi-chart-lane-dedups-a-seed-where-it-starts-not-where-it-lands
kind: issue
title: ssi: plane × NURBS dedups a seed against the tubes where it starts, the ℝ³ lane where Newton lands it
status: open
opened: 2026-10-02
priority: P1
cost: E
---


Found in the §5 sweep of `ssi-a-seed-refined-off-the-chart-is-marched`.
Not measured on any fixture.

## What

The two doors deduplicate seeds against the tubes of the branches
already found, and they do it differently.

- `cylinder_sphere_ssi` (`crates/geom-brep/src/ssi.rs` ~:1687) refines
  the seed first and tests where Newton *lands* it. Its comment
  explains why: a cell centre outside every tube can land squarely
  inside one, re-march a branch already found, and return a duplicate
  `SsiBranch`. That is two carriers for one component, and an
  exhaustiveness receipt that counts one tube twice.
- `plane_nurbs_ssi` (~:1933) tests the raw seed `(u, v)` against the
  chart tubes and marches without refining. It is open to the
  duplicate that the ℝ³ lane closed.

## Fix shape

Refine first, then test the landed `(u, v)` (`state[2..4]`) against the
chart tubes, as the ℝ³ lane does. Pin it with a wall whose seeder hands
a seed outside a found branch's chart tube that Newton lands inside
it. Building that fixture is most of the work.
