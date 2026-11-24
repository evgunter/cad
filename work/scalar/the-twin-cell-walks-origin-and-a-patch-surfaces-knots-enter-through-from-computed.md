---
id: the-twin-cell-walks-origin-and-a-patch-surfaces-knots-enter-through-from-computed
kind: issue
title: The twin cell walks' translation origin and Patch::surface's knots enter T through from_computed: f64 structure a constructed object is built from
status: open
opened: 2026-11-23
priority: P2
cost: E
refs: [f64-built-data-is-built-at-t, a-place-is-an-f64-argument-answered-on-the-value-channel]
---


Filed by the `f64-built-data-is-built-at-t` lane, 2026-11-23, from the note PR 7250's fix pass left on that unit's file. That unit closes with PR 7292, which does not do these; this row holds them.

Two sites left as `T::from_computed` because they are data a constructed object is built from, not places:

- **The translation origin of the twin cell walks**: `topo/src/spline_trim.rs` `bezier_cells` (`centre`) and `geom-brep/src/pcurve_cache/spline_incidence.rs` `Net::of` (`centre`), the middle of the control net's `f64` box. Any origin gives the same floors in exact arithmetic; at `Interval` it is a point, so the moved net and every floor are as wide as the control points make them. Built at `T` it would be the box's middle over `T` coordinates, wider at `Interval`, which moves floors near the band: measure before choosing.
- **`topo/src/spline_trim.rs` `Patch::surface`'s knots `(a, b)`**: a cell's own parameter rectangle, the knot vector of the surface the arms are metered on. They are the cell's rect, which is `f64` structure. Whether they are a constant of the cut (as a path sweep's station shares and their averaged knots are: PR 7292, `sweep::skin` `stated_knots`) or a value to build at `T` is this row's question.

Both run only at a scalar that carries no tangent (the modules are bounded `Decide<Value = T>`), so no dual reaches them.
