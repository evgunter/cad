---
id: a-bound-to-decide-against-never-becomes-a-t
kind: unit
title: A bound to decide against is decided at f64 or is an f64 allowance of the margin; it never becomes a T
status: closed
opened: 2026-11-20
priority: P1
cost: H
parent: from-computed-claims-a-zero-tangent-at-a-dual
pr: 7185
branch: scalar/a-bound-never-becomes-a-t
closed: 2026-11-20
---

The rule is `crates/geom-core/README.md`, "Reading a bracket where nothing certifies" (DL5.2, DL5.3), ratified by Ev on PR 7155. The site counts below are two designers' readings (`work/scalar/log.md`, 2026-11-20), mostly not traced to consumers: re-measure before building on one.

## Scope

About 42 calls lift a floor, a supremum, a rounding pad or a lever arm into `T` only to subtract it before a decision: `spline_trim` `floor`, `exits_clear`, `boxed`; `pcurve_cache/projected.rs` `hull_sector`, `axial_floor`, `net_incidence` and the `Meters` helpers; `ssi/certify.rs` `hull_limb`, `tube_transversality`; `editor-core` `mate/value.rs` levers and `Arm::lever`; `topo/src/props.rs` volume fields; `edge_nurbs` `limb2_enclosure_as`; `pcurves_rings` `side`; `rigid_motion`; single calls elsewhere.

- A bound with no `T` operand is decided at `f64` (`hull_sector` is `holds(T::from_computed(piece.x_lo), ..)`).
- A bound subtracted from a `T` quantity is an `f64` allowance term of the margin or lever: the door takes the `f64`.
- `props.rs`'s `volume_lo`/`volume_hi`/`surface_area` and the quadrature midpoints hold bracket ends in `T`-typed fields; they are `f64` fields.
- `coincide/replay.rs`'s lever scale, listed `constant` in `from-f64-sites.tsv` and computed from a bracket, is one of these.

## Not this unit

Places, solved values, `f64`-built nets. `Certifies`.

## Measure first

`Margin::<f64>::lift` is itself a re-entry and `Margin::value` is a public `T` exit: say what the allowance door is before the sweep, and who reads `Margin::value` outside `geom-core`. Whether a bound decided at `f64` gives the verdict it gave at `T` at `Interval` and `Sym`, where the lifted point was exact.

## Decides

The shape of the allowance term on `Margin` and `Lever`.

## Review

Letter H, tier single, FULL: about 42 decision sites move from `T` to `f64` arithmetic, and a verdict that changes at `Interval` or `Sym` cannot be believed by reading.
