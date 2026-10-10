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
- at ε = 1e-6 the wall refuses at its seam with the next wall, at the
  iso-row arm's guard, ahead of routing: `FittedBoundaryUnsupported {
  what: "a row of this fit shared with a spline face" }`
  (`crates/topo/src/replace_face.rs:2033`). The seam is a row of the
  moving wall's own fit, and the neighbour is the unmoved NURBS wall, so
  in `shell` the pair is never two fits.

What each of the wall's edges does on its own, read per edge at its plan
(`topo::offset_edge_plans_for_tests`), one wall moved alone:

- the door plans the edges in the order top rim, seam, bottom rim, other
  seam, and stops at the first seam, so one rim is planned before it;
- by `d = 5e-10` (offd's row): the bottom rim derives as the cap plane's
  section of the fit; the top rim's section is refused and deferred to
  the corners, `NoBranch` at ε ≥ 1e-9 (the fit's window, the base's own,
  stops short of the top cap plane on this twisted wall) and an `Ssi`
  tube refusal at 1e-12;
- by `d = −0.05` (the shell's thickness) at 1e-6: both rims derive;
- the other seam is not a row of the fit, so it routes as
  `Approx × Nurbs` and refuses `NeighborPairUnroutable`.

Pinned in `crates/sweep/tests/encl_curved_loft_shell.rs`,
`shelling_the_curved_loft_refuses_at_a_walls_fit`, and in
`crates/sweep/tests/offd_r1_probes.rs`,
`the_fitted_obstruction_holds_on_a_curved_fit` (the seam) and
`a_fitted_walls_rims_answer_for_themselves_behind_its_seams` (each
edge).
