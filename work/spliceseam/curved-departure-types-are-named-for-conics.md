---
id: curved-departure-types-are-named-for-conics
kind: issue
title: ConicDeparture, ConicBend and Reach::Conic now carry spirics and splines, and their names say conic
status: open
opened: 2026-10-26
priority: P4
cost: E
refs: [a-nurbs-edges-sector-departure-is-its-chord, 5723]
---


Filed by `delete-the-boolean-operand-edge-gate` (PR 5723's review, S8).

## Finding

`geom_brep::ConicDeparture` and `geom_brep::ConicBend` (`conic_bend.rs`),
and `topo::boolean::sectors::Reach::Conic`, are what every curved edge's
departure at a vertex now reads: a circle, an ellipse, a spiric and,
since `a-nurbs-edges-sector-departure-is-its-chord`, a spline
(`BendBounds::Unbounded`). Their names and the module's opening line
still say "conic", so a reader takes a spline's departure for a conic
reading it is not. The docs were re-worded in that row; the names were
not, since the rename crosses eight files in `geom-brep` and `topo`.

## Fix

Rename to the curved-edge vocabulary (for instance `CurvedDeparture`,
`CurveBend`, `Reach::Curved`) in one mechanical change, docs following.

Moved from SPLICE to SPLICESEAM on 2026-10-28, by `git mv` in SPLICE's split along its seams (Track size; Ev, in chat, 2026-10-28).
