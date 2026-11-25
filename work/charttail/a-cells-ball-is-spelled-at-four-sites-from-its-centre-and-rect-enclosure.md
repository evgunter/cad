---
id: a-cells-ball-is-spelled-at-four-sites-from-its-centre-and-rect-enclosure
kind: issue
title: A cell's ball, its centre station and its chart rectangle's enclosure, is spelled at four sites with no one home beside Ball
status: open
opened: 2026-11-25
priority: P4
cost: E
---



## Finding

Found by the review of PR 7787 (lane `embedwalk-trimmed`).

`cell_region::Ball` is the ball holding a chart cell's patch. Its one
constructor is `Ball::holding(centre, patch)`. Every caller spells the
cell's ball the same way, as the centre station and the rectangle's
enclosure (`Surface::rect_enclosure`, which narrows with the cell on a
spline chart):

- `crates/topo/src/cell_region.rs` `cell_region_lifted` (now reads
  `Ball::holding(station, surface.rect_enclosure(u, v))` and
  `Ball::clear_as`, PR 7787);
- `crates/topo/src/clearance.rs` `ball_of`, over the cell's stored
  enclosure;
- `crates/topo/src/injective.rs`, two sites (EMBED's):
  `Ball::holding(surface.point_eval(m(x), m(y)), surface.rect_enclosure(x, y))`.

The centre is `eval_at` in one, `point_eval` in the others. A reader
spelling the patch with `Surface::eval` at the rectangle's intervals
instead is a ball that never narrows on a spline chart, which
`cell_region_lifted` did until PR 7787.

## What is needed

A `Ball` constructor for a cell (`Ball::of_cell(surface, u, v)`), read
at every site, so the ball has one home.
