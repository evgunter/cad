---
id: a-wall-seam-between-two-fits-has-no-section
kind: issue
title: C5 implements no NURBS x NURBS section, so a shelled loft's wall-wall seams (two moved fits meeting at a crease) refuse; the next gate for every lofted shell
status: open
opened: 2026-10-08
priority: P2
cost: H
refs: [a-fitted-wall-has-no-section-with-a-moved-cap]
---

Filed from the designer pair on `a-fitted-wall-has-no-section-with-a-moved-cap`
(`analysis/design-fork/shell-fitted-wall-section`). Once plane × `Approx`
routes over the fit, a shelled loft's cap edges have a section, but
every wall moves too: the twisted loft's wall–wall seams are creases
between two moved fits, a NURBS × NURBS section, which C5
(`crates/geom-brep/README.md`) lists as unimplemented even for plain
NURBS (it is the same gap on the straight prism's walls). It refuses
`NeighborPairUnroutable(Nurbs, Nurbs)`.

Owed: a NURBS × NURBS arm — a marcher, both charts' uniqueness tubes,
an exhaustiveness and seeding story — or a measured reason the seam can
be read another way. Before pricing, re-measure the 2026-09-25 rows the
designers cited: the twisted loft's wall fit failing the default ε
(4.14e-9 achieved) and the vase's interior-knot crease gate.

## Measured

2026-10-09, after plane × `Approx` routed over the fit
(`a-fitted-wall-has-no-section-with-a-moved-cap`). The twisted loft never
reaches `NeighborPairUnroutable(Nurbs, Nurbs)`:

- at ε = 1e-9 (the default) and 1e-12 the first wall's offset fit refuses
  before any edge (`Fit { BudgetExhausted }`, best bound 4.12e-9 m on a
  (27, 17) grid);
- at ε = 1e-6 the wall's rims with the caps derive, and its seam with the
  next wall refuses at the iso-row arm's guard, ahead of routing:
  `FittedBoundaryUnsupported { what: "a row of this fit shared with a
  spline face" }` (`crates/topo/src/replace_face.rs:1953`). The seam is a
  row of the moving wall's own fit, and the neighbour is the unmoved NURBS
  wall, so in `shell` the pair is never two fits.

Pinned in `crates/sweep/tests/encl_curved_loft_shell.rs`,
`shelling_the_curved_loft_refuses_at_a_walls_fit`, and in
`crates/sweep/tests/offd_r1_probes.rs`,
`the_fitted_obstruction_holds_on_a_curved_fit`.
