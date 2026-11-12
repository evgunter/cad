---
id: an-axis-reads-its-direction-as-unit-by-convention-not-type
kind: issue
title: A unit axis is a convention to ε/arm, not a type, so Line's metric readings divide by |dir| and six sites read an axis never decided nonzero
status: closed
opened: 2026-11-06
closed: 2026-11-10
pr: 6697
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

## Decided, and done

**No typed unit axis** (option A, orchestrator ruling, 2026-11-09).
- `Line::height` and `Line::distance` divide by `|dir|`, which is the renormalized reading `w·(dir/|dir|)`. A typed axis minted by renormalizing a stored axis would read the same answers, to rounding.
- What it would add is width only: about 1.5× on `height` and 1.7× on `distance`, and only where the axis alone carries width (the tables in their docs). No `f64` answer moves.
- Option B (`geom::Axis { origin, dir: UnitVec3 }`) is rejected. Its mint is a banded decision, so about 50 readers of tier-3-certified at-rest axes would each gain a refusal arm and a K row for a value already certified.
- Option C (`Line.dir: UnitVec3`) is rejected. `Line` also carries chords and `Ray`'s units-of-`|dir|` contract (PR 6665), step-import stores unnormalized line carriers, and geom's crate docs keep carrier direction fields bare.
- If a band reading ever needs the narrower width, file it as a measured follow-up.

**The six sites.** Traced to where each direction is minted, none of them reads a computed direction that could be zero. So each comment names its construction and adds no arm:
- **The cone's apex-line pair** is two rulings of the cone's squared frame, `a·cos α + (u·cos φ + v·sin φ)·sin α` over orthonormal `a, u, v`, unit to rounding. Its readers are `join.rs` `line_of` (`ApexLines`), `offset_axial` `mint_carrier` (`ConeCapSection::Generators`) and `splitting/join.rs` `ruling_pairs`. The construction is stated once, at geom-brep's `ApexFan::ruling` and `PlaneConeSection::ApexLinePair`.
- **The plane×cylinder parallel and tangent rulings, and the parallel cylinders' tangent line**, run along the cylinder's stored axis, which tier 3's check 1 refuses zero at rest. Their readers are `ruling_pairs` and `rim_wedge`'s tangent locus (`boolean/mod.rs` `verify_tangency`, which normalizes it).
- **`Curve3::clearance`'s Line arm** carries the carrier's precondition, as the conic arms carry their axis's. Its doc names it.
- **`end_face` `line_clearance`'s chord** is `dir·(tb − ta)` over a certified edge's window.
  - The window is decided forward at certification.
  - The one hole was certification itself: `EdgeCurve::certify` decided a Line's span forward in parameter, not in metres, so a line of zero `dir` certified.
  - Where `dir` is the zero vector, it now also decides the span in metres, at the line's speed `|dir|` (`Margin::metered` with `carrier_speed::speed_range`), as the Nurbs arm does. The margin is an exact zero, and the line refuses as `IntervalNotForward`, so no certified line edge carries a zero `dir`. Every other line is decided once, in parameter, as before, so neither its K stream nor its symbolic forms move.
  - Pinned by `certify::tests::a_lines_span_is_forward_in_metres`, which is red on main.
