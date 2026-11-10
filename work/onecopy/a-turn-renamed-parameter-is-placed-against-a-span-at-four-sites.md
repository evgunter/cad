---
id: a-turn-renamed-parameter-is-placed-against-a-span-at-four-sites
kind: issue
title: ring_path's span_readings places a parameter named about a span's middle against the span, where a box wider than a near-full arc's gap can be renamed a turn away (narrowed from four sites)
status: closed
opened: 2026-11-09
priority: P3
cost: M
closed: 2026-11-10
pr: 6727
---


Found by the PR 6666 review, beside the circle-arc distance fix (PR 6677, `a-circle-arcs-distance-floor-reads-above-a-box-named-a-turn-away`).

Four sites read a point's parameter about a span's middle (`Curve3::param_near`, or `Azimuth::near` against `mid_param`) and then decide it against the span's ends:
- `crates/topo/src/boolean/torus_region.rs` `arc_place` (~`:282`), through `geom_brep::curve_face::plane_crossing::span_place`: inside, at an end, or past one;
- `crates/topo/src/shell.rs` `strictly_inside` (~`:3532`), the `shell_seam_corner_inside` row: strictly inside, or not;
- `crates/topo/src/ring_path.rs` the oval's `param` (~`:791`), read by `LoopArc`'s in-span readers (~`:770`);
- `crates/sweep/src/blend/surgery.rs` the split's window test (~`:3523`), a BOUNDS-lane read of the parameter against the stored window.

At `Interval`, `Azimuth::near` names every direction of a box by one name of its angle, renamed by one whole turn, so a box reaching the antipode of the middle can be named a turn from its copy about the middle (`geom::Curve3::distance_within`'s doc). Placed against the span, such a parameter can read "outside" or "at an end" where some of the box is inside, rather than refusing. This happens:
- near a near-full arc's gap;
- at the ends of an arc;
- on an arc of a turn or more, where a reading about the middle can lie past both ends.

At `f64` a point's parameter is one name, so the hole is in a box's reading only. **Unprobed:** no fixture or sweep has been run against these four.

**The question for this row.** Should all four read through one span-placement door that handles the renaming once? That door would place a parameter against a span by its distance round the turn from the middle (as `azimuth::separation` reads it), or by testing the parameter and its copies a turn either side. `span_place` is the nearest home. The surgery's BOUNDS-lane test and the shell's strictly-inside test ask the same question on other rows.

**What closes it:**
- a probe at `Interval` of each site, with boxes about the middle's antipode, near-full arcs and spans of a turn or more;
- then either one door the four read, or a reason, cited at each site, why its spans cannot reach the antipode.

## Probed (`onecopy/span-turns`)

Each site's reading of `near` / `param_near` was logged against its span over the topo, sweep, editor-core and step-import `ci` suites at both ε, using temporary logging that is not kept. There were about 105k readings at default ε and 102k at 1e-12.

**Production: no wrong placement.**
- **Readings by site:**
  - The shell site read 45 points.
  - The oval read 1,367 to 1,397 points.
  - Surgery read about 590 Interval readings and 12.6k points.
  - The torus `arc_place` read 3.5k to 4.2k Interval readings and about 80k points.
- **Spans:** every span is under a turn with a real gap, or exactly one period.
- **Reach:** only 4 Interval readings reach past mid ± π, all at `arc_place`. They are the antipode of a short arc (span (1.17, 1.97), θ = 3π/2), which is past under either copy.
- **Widths:** the widest Interval reading is 2.6e-5 rad, and none is wider than its span's gap.

**What can read wrong.** A box angularly wider than a near-full arc's gap, read against the span by an off-carrier point's foot. `Period::near` renames the whole box onto one copy past the stop end, while its points beyond the gap have feet inside past the start.

The pin is `geom-brep curve_pair::span_rows::a_box_astride_a_near_full_arcs_gap_is_not_read_past`. On the base, `Span::place` reads `Ok(Past)` for such a box.

**By site:**
- **`Span` (the home, from PR 6687).** Each end's side, on a closed carrier, is the greater of the parameter's own margin and its copy's a turn away against the other end. At `f64` a name within half a turn of the middle is no nearer the other end a turn away, so the side is its own to rounding. The pin is green.
- **torus `arc_place`.** It reads through `Span::place_param`, whose chords are read from the carrier's own point at the parameter. A named enclosure straddling the gap covers the start's point, so the chord either decides End or escalates. It is safe as it stands, and also gets the side fix.
- **shell `strictly_inside`.** It now reads through `Span::place_param` (Inside, at no end), in place of its own `shell_seam_corner_inside` row. Its meter moves from the chord over the span to the carrier's greatest speed.
- **surgery's window test.** This is a bounds-lane read, `lo() > 0` on both sides. A one-sided name reads outside on its copy and refuses typed, so it never answers wrong; it is cited at the site. The window is also under one period, from the `Within::Short` guard above it.

## Narrowed: what remains

ring_path's `span_readings` (`split_ring_path_in_span`, ~`:1668`), read on `LoopArc::param_at`, the oval's `param` and the conic's `param_in`, and on path pieces' roots.
- **Why it was not folded.** It also reads line spans, and its path roots are not named about the span's middle. The `Span` copy reading would rename those roots and move parities at `f64`.
- **Evidence.** The oval and conic readings in the suites are points only, and the parity fuzz rows hold against their oracles.
- **What closes it.** A probe of `span_readings` at `Interval`, with boxes wider than a near-full arc's gap. Then either the copy reading there, keyed on the carrier being periodic and the parameter being named about the middle, or `Span::place_param` in its place.

## Closed (2026-11-10, `onecopy/turn-place`)

**Probed.** `ring_path::span_turn_rows` reads span_readings at `Interval` on a near-full arc's gap. A box holding the circle's points from just past the stop end, across the gap, to past the start a turn away is named wholly past the stop end, `[6.093, 6.385]` on `(0, τ − 0.2)`. The base read it `[Positive, Negative]`: decided outside, though the box holds an inside point. So the path parity passed over a crossing the box may hold.

**One door for the copy rule.** `geom_core::period::Period::span_margins(t, (lo, hi))` reads each end's margin on `t`'s own copy and on its copy a period away, against the other end. `curve_pair::Span::ends_place` reads its ends through it on a closed carrier, and so does span_readings. The pin is green.
- **Every periodic reading** span_readings takes is named about its span's middle: `plane_roots`' roots (`Azimuth::near(middle)`), `CircleArc::param`, the crest's `param_in`, `Conic::param_in` and `Oval::param`. The narrowing above said path roots were not; on main they are.
- **The line arm's parameter** has no copy (`None`).
- **The meters stay each site's own.** span_readings levers a root's side by the slope its piece crosses at, which keeps near-graze roots honest. `Span` reads its side at the carrier's sup speed. So `Span::place_param` is not span_readings' door, and only the copy rule is shared.

**Moves, measured** over topo, sweep, editor-core, step-import and mesh at ε 1e-9 and 1e-12, about 300k readings each, with temporary logging that is not kept:
- **f64:** 496 and 448 point readings moved bits. Every one is the exact tie, a parameter at the middle's antipode, which is past on either copy. No sign moved. The door's doc states the tie.
- **Interval:** 2 production box readings moved at 1e-9, one box `[2.48, 3.81]` against `(−0.59, 0.59)`, which is past on both copies, so no sign moved. The rest are the probe's own.
