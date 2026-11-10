---
id: a-turn-renamed-parameter-is-placed-against-a-span-at-four-sites
kind: issue
title: Four sites place a parameter read about a span's middle against the span, where a turn-renamed parameter can read outside or at an end
status: open
opened: 2026-11-09
priority: P3
cost: M
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
