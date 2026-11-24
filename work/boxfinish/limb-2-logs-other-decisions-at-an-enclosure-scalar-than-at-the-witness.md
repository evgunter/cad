---
id: limb-2-logs-other-decisions-at-an-enclosure-scalar-than-at-the-witness
kind: issue
title: Limb 2 logs other decisions at an enclosure scalar than at the f64 witness
status: open
opened: 2026-11-24
---


Found by reading, in the sweep for checks logged at one scalar and not
another; **not reproduced by a run**.

`limb2` (`crates/geom-brep/src/edge_nurbs.rs:2272`) branches on the
scalar's witness kind, and the two arms state different decisions on
the log:

- **`Offset::Miss { bound: Ok(_), .. }`.** A point scalar decides
  `ssi_hull_value` (`measured_miss`, `edge_nurbs.rs:2512`) and, when
  that is in the band, `ssi_hull_sup` (`upper_bound`,
  `edge_nurbs.rs:2543`): two decisions. An exact-witness scalar takes
  `limb2_enclosure_as` (`edge_nurbs.rs:2312`), which states one.
- **`Offset::Bound(_)`.** A point scalar decides `ssi_hull_sup`. An
  exact-witness scalar decides `ssi_hull_value` when the floor is
  positive (`edge_nurbs.rs:2319`) and `ssi_hull_sup` otherwise: the
  predicate's name can differ.

Both doors go through `dihedral::decide_reported`
(`crates/geom-brep/src/dihedral.rs:259`), which is the logged funnel.
If these are checks rather than choices, a leaf whose witness walked
the point arm reads a verdict sequence the leaf's replay does not, and
`drive` would refuse it as it refused the curved leaves of
`a-witness-records-curved-premise-checks-the-interval-replay-skips-so-no-curved-leaf-certifies`.

What is not known: whether a drive in the tree reaches `limb2` with a
break miss in the band at both scalars, and which role these
predicates carry. A row driving a range over a body whose section
edge is certified by rung 3 would say.
