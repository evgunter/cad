---
id: a-span-limited-distance-is-spelled-at-eight-sites
kind: issue
title: The distance from a point to a carrier's span is spelled at eight sites
status: closed
opened: 2026-11-09
priority: P3
cost: E
closed: 2026-11-09
pr: 6666
---


Found by the PR 6645 review. The distance from a point to the part of a carrier inside a parameter span had three homes for the circle:
- `geom::Curve3::distance_within` (`crates/geom/src/curves/on_carrier.rs`)
- `topo::splitting::containment::arc_distance_floor` (`crates/topo/src/splitting/containment.rs`)
- `topo::ring_path::ConicArc::distance_floor` (`crates/topo/src/ring_path.rs`)

The line arm was also spelled as a clamped foot followed by a norm at five sites, and at a sixth in the plane:
- `crates/sweep/src/blend/surgery.rs` (~`:2428`)
- `crates/topo/src/link.rs` `to_segment` (~`:1355`)
- `crates/step-import/src/adopt.rs` (~`:939`)
- `crates/topo/src/boolean/solid_contain.rs` (~`:3471`)
- `crates/topo/src/census.rs` (~`:1499`, via `Margin::norm3`)
- `crates/step-import/src/chart.rs` `boundary_distance` (~`:330`), in the plane on `geom_core::Line2`

`ConicArc::distance_floor`'s clamp form was unsound at `Interval` for a box astride the arc's far side.

Fix: one home on geom, `Line::distance_within` for the line (`geom_core::Line2::distance_within` in the plane) and `Curve3::distance_within` for every carrier, with the eight sites reading it. `scripts/gates/line-distance-reads-distance.sh` fires on a span foot's norm written by hand. It cannot see a foot bound to a name before its norm, or one behind a local helper (link.rs's `foot`).
