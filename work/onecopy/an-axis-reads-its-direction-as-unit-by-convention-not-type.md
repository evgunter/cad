---
id: an-axis-reads-its-direction-as-unit-by-convention-not-type
kind: issue
title: Line::height/distance read dir as unit by a debug-checked contract; a typed unit axis would make it a type and force the nonzero decision six sites skip
status: open
opened: 2026-11-06
---


Filed by `onecopy/feet-and-params` (PR 6568's fix pass), whose ruling (option A) gave `geom::Line` its metric readings, `height`, `distance` and `height_and_distance`. They read `dir` as unit with no division. That is the tightest reading at `Interval` in every regime measured (the table is in `Line::distance`'s doc), but it makes unit length a contract: on a non-unit `dir` they answer a wrong number, not a wide one. Today the contract is a `debug_assert!` (`Line::unit_contract`) plus docs, which is the root-cause fix's stand-in, not the fix.

**The root-cause fix is option B:** a typed unit axis, for example `geom::Axis { origin, dir: UnitVec3<T> }`, carrying the metric readings. Minting it is a banded decision that can refuse, so a reader can no longer reach the readings with an undecided direction. `Line` keeps the parameter readings, which divide by `|dir|²` and so hold for any nonzero `dir`.

Sites that read an axis with no decision that it is nonzero, which a `UnitVec3` would force (the PR 6568 audit):
- `crates/sweep/src/blend/open/end_face.rs` `line_clearance` (~`:545`): `e = chord / len`, with only a comment that the edge has positive length.
- `crates/topo/src/boolean/join.rs` `line_of` (~`:3555`): `ApexLines.dirs`, the `fan.ruling(φ ± acos …)` of a plane×cone apex-line pair, unit by trig identity and never decided.
- `crates/topo/src/boolean/rim_wedge.rs` `on_locus` (~`:703`) and `rides` (~`:934`): `dir / dir.norm()` of a computed tangent line (`boolean/mod.rs` ~`:7668`).
- `crates/topo/src/offset_axial.rs` `mint_carrier` (~`:2332`): cone×cap generator rulings, not normalized at the site.
- `crates/topo/src/splitting/join.rs` `ruling_pairs` (~`:594`): `dir.normalize()` of a section ruling.
- `crates/geom/src/curves/on_carrier.rs` `Curve3::clearance`, when called on a freshly built section curve rather than a body edge's carrier.

The same audit found the other 50 or so ported sites read a stored at-rest carrier axis, an axis normalized at the site, or a `UnitVec3`. Those would mint an `Axis` from a value already decided or checked at rest.

Before you fix it, decide the type's home and its mint. One option is a `Frame::axis`-shaped door, `Frame::axis_line` returning the typed axis. Say also whether an at-rest carrier's stored axis mints by check or by assertion.
