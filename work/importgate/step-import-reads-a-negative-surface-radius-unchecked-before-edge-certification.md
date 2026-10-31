---
id: step-import-reads-a-negative-surface-radius-unchecked-before-edge-certification
kind: issue
title: step-import: a negative CYLINDRICAL/SPHERICAL/TOROIDAL radius passes entities.rs unchecked and reaches edge certification before check 1
status: open
opened: 2026-10-14
priority: P2
cost: E
---

(Filed by the ENCL orchestrator from the adoption survey for `[ev]` PR 5134. Read from source, not executed.)

## What

`crates/step-import/src/entities.rs` reads the radius of a `CYLINDRICAL_SURFACE`, `SPHERICAL_SURFACE` or `TOROIDAL_SURFACE` verbatim through `as_length`, which has no sign check. The torus convention is noted as "enforced at rest" by tier-3 check 1 (`UnrepresentableSurfaceDatum`, `crates/topo/src/validate.rs`).

Adoption, however, certifies every edge first: `adopt::adopt_edges` calls `Body::set_edge_curve`, which runs geom-brep certification. That happens before `gate3` runs check 1. During certification:
- the cylinder residual `(w² − r²)/(2r)` (`crates/geom-brep/src/implicit.rs`) is sign-blind, so residual checks pass with a negative r;
- `curvature_lever_arm` returns the radius verbatim, so `DIHEDRAL_ARM` reads a negative arm.

A file with a negative radius therefore reaches edge certification and the dihedral decision, a state the rest of the kernel treats as unrepresentable.

## Ask

Refuse a nonpositive radius (and the torus convention) at parse time, as a typed import refusal naming the entity, so that no certification ever sees it. Pin it with a fixture. This also bears on `[ev]` PR 5134: once this is fixed, a negative dihedral arm is reachable only by a kernel bug.
