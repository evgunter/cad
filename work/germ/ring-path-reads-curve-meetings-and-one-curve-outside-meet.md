---
id: ring-path-reads-curve-meetings-and-one-curve-outside-meet
kind: issue
title: ring_path's misses_in_plane and segment_misses decide curve-pair meetings, and split_ring_path_same_circle decides one curve, outside geom_brep::curve_pair
status: open
opened: 2026-10-14
priority: P3
cost: M
refs: [5227]
---

Found by the review of PR 5154 (stage (c) of `the-curve-pair-reader` (closed, PRs 5154, 5162 and 5227)): its sweep for curve-pair meetings and one-curve readings outside `geom_brep::curve_pair` missed these.

In `crates/topo/src/ring_path.rs`:
- **`misses_in_plane`** decides whether a pierce ring's circle arc and a conic arc in one plane miss each other.
  - It reads `split_ring_path_coplanar_apart` (a ring gap) first.
  - It then decides one curve by `split_ring_path_same_circle`, on the margin `max(|Δc|, |r − a|, |r − b|)`. That is a second one-curve reading beside `geom_brep::curve_pair::pose::one_curve` (the Procrustes margin, which bounds the Hausdorff distance, tilt included). The two can decide one pair differently in the band (D3, "One reading per question").
  - Last come the four span-end readings.
- **`segment_misses`** decides whether a segment misses a conic by `split_ring_path_segment_apart`: the conic's distance floor at the segment's midpoint, less half its length.

Both are curve-pair meeting questions that `geom_brep::curve_pair::meet` reads through its one `(CurveKind, CurveKind)` table (C5).

A route:
- Read the arc pair and the segment against the conic with `meet`, keeping the ring path's `charge(…, slack)` for the resolution, as `work/germ/ring-path-readings-decide-past-their-resolution-at-far-frames.md` describes.
- The one-curve decision then becomes `meet`'s `shared` answer.
