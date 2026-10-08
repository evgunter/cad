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
