---
id: f64-built-data-is-built-at-t
kind: unit
title: Data built whole at f64 and lifted is built at T
status: open
opened: 2026-11-20
priority: P1
cost: M
parent: from-computed-claims-a-zero-tangent-at-a-dual
---

The rule is `crates/geom-core/README.md`, "Reading a bracket where nothing certifies" (DL5.2, DL5.3), ratified by Ev on PR 7155. The site counts below are two designers' readings (`work/scalar/log.md`, 2026-11-20), mostly not traced to consumers: re-measure before building on one.

Ev, PR 7155: no lift behind `CertifiedEnclosure` unless a place for one is shown. Both designers then read every site and found none that needs it:

- `sweep/src/loft.rs` `lift_loop_walls` and `LoftBuild::enclosure` (6 calls): an `f64`-built skin lifted into `T`. Built at `T`; `LoftBuild::in_t` already does it. The four `launders` rows of `from-f64-sites.tsv` close with it.
- `sweep/src/skin.rs` `minted` (a frame's reference vector) and `skin_validated` (a path sweep's section parameters): trace where the `f64` path comes from. Authored data enters as a constant; a path read down from `T` stays `T`.
- `edge_nurbs` `on_carrier_domain` (2): a fitted pcurve image, "evidence that limb 2 bounds, not a certified quantity". It stays `f64` data.
- `profile` `lift_computed`: demos only. The demos build at `T` and the function goes.

Not checked: whether `in_t` at `Interval` for point sections is wider than the lifted skin, and it is not bit-identical to the `f64` loft; whether a `Pcurve<T>` consumer forces the fitted image into `T`. C6's own lift (a fit's knots, weights and poles, once, at the fit) is behind the fitted lanes and is not this row's.
