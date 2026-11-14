---
id: a-carried-arcs-miss-is-charged-its-ends-enclosure-width-at-the-interval-scalar
kind: issue
title: A carried rim or parallel arc's whole-turn miss is charged about twice its ends' enclosure width at Interval, so far legal walls of a few arcs escalate props_chart_loop_whole at eps 1e-12
status: open
opened: 2026-11-14
priority: P3
cost: M
---


Found by BOXFINISH's edge-turns unit and its review (branch
`boxfinish/edge-turns`, PR 6946). That unit gave the cylinder's and the
torus's per-edge whole-turn misses the cone's constructors
(`EdgeTurn::short`, `EdgeTurn::carried`,
`crates/geom-brep/src/props/closure.rs`). What it left refusing is this.

## Measured

**Hand-built legal walls** through `curved_face_loops`. The frame is at
(30, −110, 200), axis (1, 1, 1)/√3; each rim or parallel is in 1 to 200
arcs. These are the pins citing this row in
`crates/geom-brep/tests/far_walls_of_many_arcs.rs`. At `Interval`,
ε = 1e-12, `props_chart_loop_whole` escalates on:

| wall | arcs a rim | margin |
|---|---|---|
| cylinder r 50 | 6 / 40 / 200 | `[0, 3.96e-12]` / `3.84e-11` / `2.05e-10` |
| cylinder r 213 | 6 / 40 / 200 | `[0, 1.62e-11]` / `1.10e-10` / `5.51e-10` |
| cylinder r 410 | 1 / 6 | `[0, 3.21e-12]` / `3.15e-11` |
| torus major 100 (minor 2.5) | 6 / 40 / 200 | `[0, 1.02e-11]` / `7.56e-11` / `3.83e-10` |
| torus major 300 | 1 | `[0, 1.70e-12]` |

At ε = 1e-9 and 1e-6 every one of them measures.

**Through a door.** The tilted hollow tube
(`sweep::tube_along_arc_hollow`, axis (1, 1, 1)/√3 through the origin,
outer 2.5, wall 0.001, full period) builds through major 750 at
ε = 1e-12. It refuses from 800 on:

| major | margin |
|---|---|
| 800 | `[0, 1.08e-12]` |
| 850 | `[0, 1.14e-12]` |
| 900 | `[0, 1.22e-12]` |
| 990 | `[0, 1.36e-12]` |
| 1100 | `[0, 1.88e-12]` |

The 990 and 1100 arms are pinned in
`crates/sweep/tests/far_walls_read_their_whole_turns.rs`.

**What it is made of.** A carried arc's miss is the principal angle
from its start, turned by its own turn, to its end, read about its
centre. At the interval scalar it is charged about twice the width of
its two ends' enclosures, whatever its radius:

- On the tube at 990, the own turn is a point (±τ_f64). The width is
  all in the own-frame arms: cross ±2.1e-9 m² over dot 9.85e5 m², so
  ±2.15e-15 rad at a lever of about 990 m, for each of the face's two
  parallels.
- A wall's threshold is therefore set by its count of carried arcs and
  by the ends' distance from the origin, not by any one arc's span. A
  rule that reads a span of exactly one period as agreeing with its ends
  would close only the whole-turn case.

**Behind it, in the tube coordinate.** A torus meridian's tube turn is
still read back about the core circle (`EdgeTurn::new` in
`torus_chart_loop`). On the tube at 990 each meridian's miss is
±3.66e-13 to ±4.71e-13 rad at lever 2.499, about 1e-12 m a meridian.
The face's two meridians take 1.83e-12 to 2.35e-12 of the 3e-12 slack.
The cost is the ends' width, not proportional to the minor radius, so a
third meridian arc on such a loop would pass the slack on this term
alone.

## The question this row leaves

A `LoopEdge`'s ends are `carrier.eval(t0)` and `carrier.eval(t1)`, so
through any public door a rim's stored span cannot disagree with its
ends. Every hand-built disagreement refuses earlier and typed:

- `props_cylinder_circle_tilt`;
- `props_rim_axis_parallel`;
- `props_rim_span_winding`;
- `ChartLoopOpen`.

These are pinned by
`far_walls_of_many_arcs::a_hand_built_far_wall_whose_rim_disagrees_refuses_before_its_turns`.
On rims, then, `props_chart_loop_whole` has no true positive reachable
from a door. Its live subject is the pairing of the parse's rim turns
with the loop's edges, which `props::curved::edge_turn_rows` exercises
at that seam.

Whoever takes this row decides between two readings:

1. a rim's miss is measured at all, with its ends' width charged as
   now;
2. it is true by construction, its pairing with its edge stated
   structurally instead.
