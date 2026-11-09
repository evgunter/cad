---
id: a-circle-arcs-distance-floor-reads-above-a-box-named-a-turn-away
kind: issue
title: A circle arc's distance floor reads above a point of a box whose azimuth is named a turn away
status: closed
opened: 2026-11-09
priority: P1
cost: E
closed: 2026-11-09
pr: 6677
---


Found while routing `topo::ring_path::ConicArc::distance_floor` through `geom::Curve3::distance_within` (the PR 6645 follow-ups). Both read a box's distance from a circle arc at `Interval`, and both read **above** the true distance of some point of the box: a wrong answer, not a refusal.

At `Interval`, `geom::Azimuth::near` (`crates/geom/src/azimuth.rs` ~`:350`) encloses each direction of a box by one name of its angle. Where the box's directions reach the antipode of the arc's middle, it renames them all by one whole turn, so some are named a turn away from their copy about the middle. Two shapes hit this:
- a box astride the antipode;
- a box near a small circle's axis, whose directions span much of a turn.

The readings:
- `ConicArc::distance_floor` (`crates/topo/src/ring_path.rs` ~`:1571`) held the azimuth to the span and read the held point. It reached one end while part of the box was nearer the other end, or nearer its own foot inside the arc.
- `Curve3::distance_within`'s circle arm (`crates/geom/src/curves/on_carrier.rs` ~`:567`) chose between the clearance and the ends by `select_le_zero` on the azimuth's offset. It decided "outside" and read the ends while part of the box lay inside the arc.

Witnesses, pinned in `crates/topo/src/ring_path/arc_floor_rows.rs`:
- astride, on `ConicArc`: reads 1.978 against a box point 1.894 from the arc;
- near the axis, on both: reads 0.17403 against a corner 0.17339 from the arc.

A randomized sweep found gaps up to 0.130 against 0.056.

Readers:
- `ConicArc::distance_floor`:
  - the boolean's touch gate (`boolean/carrier_touch.rs` ~`:396`) and the clearance cell door (`cell_region.rs` ~`:242`, ~`:336`), both through `splitting::containment::arc_distance_floor`;
  - a segment's miss of a loop arc (`ring_path::segment_misses`).
- `distance_within`'s circle arm: `boolean/sphere_region.rs`'s clearance (~`:362`) and `boolean/solid_contain.rs`'s edge distance (~`:3800`).

No production pose is known to hit it. The hole is in a box's reading only: `f64` and `Probe` read a point, where the held azimuth is the nearest. A ladder's `Interval` rung reads boxes of about rounding width, where the overshoot is of the box's width. Only a box astride the antipode within that width, or one reaching the axis, reads it at all.

Fix: `distance_within` holds the azimuth and the azimuth a turn either side of it to the span, and takes the least of the three held distances, then `max` with the whole circle's clearance. Each point's copy about the middle lies in one of the three. At `f64` the turned two hold to the ends, which are no nearer. `ConicArc::distance_floor` reads it.

## Since (PR 6666)

The three translates were replaced by a test that reads no azimuth: the foot lies in the arc where `|v|·|m|·cos(min(half, π)) − v·m ≤ 0`. Here `v` is `q`'s offset in the circle's plane and `m` is the radius to the arc's middle. The test chooses by `select_le_zero` between the whole circle's clearance and the nearer end.
- **Why.** PR 6645 had moved `solid_contain::edge_clearance`, which `side_reader`'s `loop_aim` reads, onto `distance_within`. A circle foot there broke `side_reader_aims_ask_nothing::the_aims_at_cylinder_walls_take_no_circle_foot`, because an aim must take no foot (`work/flush/the-side-readers-aims-decide-exact-ties-at-a-documents-nominal.md`). The cosine test takes none.
- **What it removes.** A cosine has no cut, so the turn-renamed class this row closed cannot arise in this reader.
- **What pins it.** `ring_path::arc_floor_rows` holds:
  - the pins for a near-full arc's gap and for arcs of more than a turn;
  - a sweep that also draws near-full arcs (gaps down to 1e-7), arcs of a turn or more, near-axis boxes and reversed spans.
