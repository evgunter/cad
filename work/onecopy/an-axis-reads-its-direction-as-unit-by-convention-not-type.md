---
id: an-axis-reads-its-direction-as-unit-by-convention-not-type
kind: issue
title: A unit axis is a convention to ε/arm, not a type, so Line's metric readings divide by |dir| and six sites read an axis never decided nonzero
status: open
opened: 2026-11-06
---


Filed by `onecopy/feet-and-params` (PR 6568's fix pass). `geom::Line`'s metric readings (`height`, `distance`, `height_and_distance`) divide by `|dir|`, so they are correct for any nonzero `dir`. A unit read with no division is tighter where the axis carries width: about 1.5× for `height` and 1.7× for `distance` (the tables are in their docs). It is not offered, because nothing makes the direction unit to the precision the band needs.

**The evidence (PR 6568).** A debug contract on a unit read (`|dir|²` within 64 ulps of 1) fired across the suite:
- `topo tier3_tests::a_far_thin_curved_body_is_read_by_its_exact_volume`: a stored at-rest circle frame's axis has `|dir|² = 1 + 4.07e-10`. A unit read puts a radius of about 1 m off by about 2e-10 m, some 200× the 1e-12 band.
- `Curve3::Line` carriers are minted unnormalized: the step-import FreeCAD corpus and `curve_face::window_fuzz` tripped it through `Curve3::clearance`'s Line arm and `validate`'s locus gap.
- `census::depth_gap`'s axis depth, now `geom::cover::Around::Line` (it replaced `Depth::Axis`), takes any line. Its segment test and fuzzer feed it `|dir|²` of 3.45 and 4.0. The variant's doc says so (a line whose direction need not be unit), which `Line::distance` reads correctly.

**What tier 3 enforces.** Check 1 refuses only a zero direction (`validate.rs` `is_direction`). Unit length is a convention with a stated tolerance, read by `geom::convention`'s representability margins: `ε − (‖axis‖ − 1)·arm` and `ε − (1 − ‖axis‖)·arm`, each decided against the band's zero. So an at-rest FRAME axis (a surface's or a conic's, read by `frame_margins`) is unit to `ε / arm`, with `arm` the frame's lever, and not to ulps. A `Curve3::Line` carrier's `dir` has no such check at all: only check 1's nonzero refusal applies, and the step-import corpus stores them unnormalized. A unit read of a stored frame axis is therefore wrong by up to `ε·|w|/arm`, which is the band itself where `|w| ≈ arm`. A unit read of a line carrier's `dir` is wrong by its whole length error.

**The root-cause fix (option B):** a typed unit axis, e.g. `geom::Axis { origin, dir: UnitVec3<T> }`, minted by normalizing a stored axis and deciding it nonzero, so it is unit by construction. Its metric readings could then drop the division and take the unit read's width. Minting is a banded decision that can refuse, so a reader can no longer reach the readings with an undecided or zero direction.

**The zero-axis hazard it would close.** These sites read an axis with no decision that it is nonzero, and `Line`'s readings divide by `|dir|` (NaN on a zero axis). Found by the PR 6568 audit:
- `crates/sweep/src/blend/open/end_face.rs` `line_clearance` (~`:545`): `e = chord / len`, with only a comment that the edge has positive length.
- `crates/topo/src/boolean/join.rs` `line_of` (~`:3555`): `ApexLines.dirs`, the `fan.ruling(φ ± acos …)` of a plane×cone apex-line pair, unit by trig identity and never decided.
- `crates/topo/src/boolean/rim_wedge.rs` `on_locus` (~`:703`) and `rides` (~`:934`): `dir / dir.norm()` of a computed tangent line (`boolean/mod.rs` ~`:7668`).
- `crates/topo/src/offset_axial.rs` `mint_carrier` (~`:2332`): cone×cap generator rulings, not normalized at the site.
- `crates/topo/src/splitting/join.rs` `ruling_pairs` (~`:594`): `dir.normalize()` of a section ruling.
- `crates/geom/src/curves/on_carrier.rs` `Curve3::clearance`, when called on a freshly built section curve rather than a body edge's carrier.

The other 50 or so ported sites read a stored at-rest carrier axis, an axis normalized at the site, or a `UnitVec3`. Those would mint an `Axis` from a value already decided, or checked at rest to the convention's tolerance.

Before you fix it, decide the type's home and its mint. One option is a `Frame::axis_line`-shaped door returning the typed axis. Say also whether an at-rest carrier's stored axis mints by renormalizing, and accepts the convention's `ε/arm` move, or by deciding it unit within the band.
