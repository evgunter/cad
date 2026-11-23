---
id: f64-built-data-is-built-at-t
kind: unit
title: Data built whole at f64 and lifted is built at T
status: closed
refs: [7292]
closed: 2026-11-23
opened: 2026-11-20
priority: P1
cost: M
parent: from-computed-claims-a-zero-tangent-at-a-dual
branch: scalar/f64-built-data-is-built-at-t
---

The rule is `crates/geom-core/README.md`, "Reading a bracket where nothing certifies" (DL5.2, DL5.3), ratified by Ev on PR 7155. The site counts below are two designers' readings (`work/scalar/log.md`, 2026-11-20), mostly not traced to consumers: re-measure before building on one.

Ev, PR 7155: no lift behind `CertifiedEnclosure` unless a place for one is shown. Both designers then read every site and found none that needs it:

- `sweep/src/loft.rs` `lift_loop_walls` and `LoftBuild::enclosure` (6 calls): an `f64`-built skin lifted into `T`. Built at `T`; `LoftBuild::in_t` already does it. The four `launders` rows of `from-f64-sites.tsv` close with it.
- `sweep/src/skin.rs` `minted` (a frame's reference vector) and `skin_validated` (a path sweep's section parameters): trace where the `f64` path comes from. Authored data enters as a constant; a path read down from `T` stays `T`.
- `edge_nurbs` `on_carrier_domain` (2): a fitted pcurve image, "evidence that limb 2 bounds, not a certified quantity". It stays `f64` data.
- `profile` `lift_computed`: demos only. The demos build at `T` and the function goes.

Not checked: whether `in_t` at `Interval` for point sections is wider than the lifted skin, and it is not bit-identical to the `f64` loft; whether a `Pcurve<T>` consumer forces the fitted image into `T`. C6's own lift (a fit's knots, weights and poles, once, at the fit) is behind the fitted lanes and is not this row's.

## Review

Letter M, tier single, a FULL review: the loft at `Interval` and at a dual changes what it computes (built at `T`, no longer lifted), and whether the enclosure widens or a tangent appears is not settled by reading.

## Note from `a-place-is-an-f64-argument-answered-on-the-value-channel`

Two more sites of this row's kind, left as `T::from_computed` by that unit's fix pass (PR 7250) because they are data a constructed object is built from, not places:

- The translation origin of the twin cell walks: `topo/src/spline_trim.rs` `bezier_cells` (`centre`) and `geom-brep/src/pcurve_cache/spline_incidence.rs` `Net::of` (`centre`), the middle of the control net's `f64` box. Any origin gives the same floors in exact arithmetic; at `Interval` it is a point, so the moved net and every floor are as wide as the control points make them. Built at `T` it would be the box's middle over `T` coordinates, wider at `Interval`, which moves floors near the band: measure before choosing.
- `topo/src/spline_trim.rs` `Patch::surface`'s knots `(a, b)`: a cell's own parameter rectangle, the knot vector of the surface the arms are metered on. They are the cell's rect, which is `f64` structure; whether they are a count-like constant of the cut or a value to build at `T` is this row's question.

Both run only at a scalar that carries no tangent (the modules are bounded `Decide<Value = T>`), so no dual reaches them.

## Closed

PR 7292. Production builds a loft and a skin in `T` with no lift; `profile`'s `lift_computed` is gone and the demos build at `T`. A loft whose skin needs enclosure knots or weights refuses at the carrier's door until C6's build reaches it (`work/inlane/an-enclosure-carrier-holds-point-weights-and-knots`); a path sweep's knots are constants and build at every station count. The fitted chart image is not this unit's: `the-fitted-chart-image-is-c6s-fit-lift-and-a-pcurve-holds-it-at-t`, with designers.
