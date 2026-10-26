---
id: link-reads-refuses-a-point-in-a-planes-sliver-outside-its-sector
kind: issue
title: Link::reads refuses a point in a plane's sliver outside its sector, so Gap's Interval lane refuses edge pairs at exact contact
status: open
priority: P3
cost: M
opened: 2026-10-26
refs: [material-and-convexity-are-read-three-ways]
---

`Link::reads` (`crates/topo/src/link.rs`, `reads_over`) takes each sector's side of `q` first, through `Link::side`, and returns `None` as soon as one reads in the band's sliver. It never asks whether `q` lies inside that sector (`Sector::place`), so a point in the sliver of a sector's plane refuses even where it lies decidedly outside the sector, far from the cone's boundary. `link::tests::reads_refuses_a_point_in_band_of_a_plane_outside_its_sector` pins it: over a block's corner, `(0.5, 0.5, z)` with `z` in the top face's sliver reads `None`, and `Link::offset` refuses `InBand` with a magnitude of about 0.5.

**Where it costs.** `Gap`'s edge pairs at the Interval scalar. `Link::lowest_on`'s candidates include roots of two pieces' squared distances equated; at a double root the discriminant is about zero, and its square root widens the root's enclosure to about 1e-8. At the axis-aligned contacts the real-body rows build, such a candidate lands on a sector's plane but outside the sector, in the sliver, and the whole measure refuses. `gap_reads_vertex_and_edge_cells::real_bodies::interval_lane_encloses_the_f64_gap` admits a typed refusal and logs three: strut/ridge at `h = ±0.01` and ridge/strut at `h = 0`. f64 reads every row.

**What a fix looks like (not decided).** Read a sliver side as off the plane where `Sector::place` decides `q` outside the sector, as `reads` already does for a Zero side, with the arc stretch `Sector::misses` checks widened from the band's zero to its escalation. That moves the census's reader too, so it is checked against the census goldens at three ε before it lands.
