---
id: an-edges-extent-has-four-dispatchers
kind: issue
title: an edge's extent has four per-Curve3 dispatchers and no home, so each per-site fold of a face's extent picks a different one
status: open
opened: 2026-10-06
priority: P1
cost: M
---


Found by PR 4115's review (the SHELL `planar-gate-misses` lane, Style Q1
and claim C6). Filed on flux because `geom::curves::boxes`, the most
exact of the four, is flux ground (`work.py territory`).

**The question with no home**: *what region can an edge's arc occupy?*,
an upper bound to fold into a face's extent. It is answered by four
per-`Curve3`-variant dispatchers, each with its own bound and tightness:

1. `geom::curves::boxes::{circle,ellipse,spiric}_arc_aabb` /
   `conic_arc_aabb` / `nurbs_curve_aabb`: exact arc boxes, outward
   rounded, but behind `T: Bounds`, which most `topo` callers do not
   carry (`bounds-allowlist.sh` ratifies compound bounds per file).
2. `topo::splitting::containment::carrier_ball`: a ball under `Decide`.
   A conic gets its whole carrier's ball, a spiric a speed-bound ball
   about the arc's midpoint, a spline its control box's ball. The shell
   clearance gate (`shell::carrier_box`) now reads this one; it was
   made `pub(crate)` for that in PR 4115.
3. `topo::boolean::boxes::{conic_extent, arc_extent}`: full-turn
   amplitude and certified subdivision over `Span`s at `Real`.
4. Until PR 4115's fix pass, a fourth one in `shell.rs` (`arc_extent`).
   It was deleted for `carrier_ball`.

The torus's own ball, `major_radius + minor_radius`, is respelled as a
literal at several sites: `boolean/solid_contain.rs` (two in the
spiric box reads, one `ext`), `splitting/containment.rs` (a tilt lever),
`boolean/carrier_eq.rs`.

These do not ask the question, and are not this class:
`geom_brep::certify::edge_extent` and `readback::edge_extent` are
LOWER bounds on an edge's diameter (a lever's certified direction).
`merge_faces.rs` `boundary_points` are on-face witness points.

**Three per-site folds that forgot the arcs wait on this**. Each would
otherwise pick a different dispatcher:
`chart-region-carrier-gates-read-vertices-not-arcs` (chart),
`splitting-face-extent-lever-reads-vertices-not-arcs` (hone) and
`tangent-witness-span-reads-vertices-not-arcs` (tang).

**Final state**: one recommended door for "an edge's extent in a frame
or about a point" at the `Decide` bound, which the three sites and the
shell gate read. The exact `Bounds` boxes stay as its tight twin, and
the literal torus ball becomes a named helper.

## Re-homed from FLUX to FLUXTAIL (2026-10-10)

(FLUX orchestrator) FLUX measured 125.5 budget points against 30 and was cut on its priority seam: FLUX kept the curved closed-form arms. FLUXTAIL collects the curved arms' numeric honesty rows: rim levels, chord lengths, span folds and bounds that cancel, drop a refusal or depend on the frame, under FLUX's closed forms. The id and the body above are unchanged; the move may have set `priority`, `cost` or `status` in the header.

## Evidence: the maximal-faces gate's ball (PR 4581, 2026-10-10)

`readback::edge_extent` is gone: its one caller, `boolean/reduce.rs`
`gate_maximal_faces`, levered its ladder at it, and a lower bound on the
diameter does not hold an arc past a half turn. The gate now reads a
ball about the edge's start that holds the edge. That ball is a
per-`Curve3` match of its own: a line's chord, a circle's farthest
point from its start (`2r·sin(min(span, π)/2)`), a NURBS carrier's
control hull, and any other carrier's box (`census::edge_reach_in`).
So it is a fifth dispatcher, and a client of the home this row owes.

## Evidence: the box lanes now share one edge map (RESTREAD, 2026-10-18)

`topo::boolean::boxes::edge_rule_box` is now the one `EdgeBoxRule → box`
map inside `topo`'s two box lanes (`edge_box` and the census's
`edge_reach_in`), and the census's copy of the spiric arm is gone. A
spiric's box has one construction everywhere: `geom::spiric_period_box`,
which charges for its own rounding. `topo`'s two lanes read it,
`geom::curves::boxes::spiric_arc_aabb` reads it, and so
`sweep::blend::surgery::boxed_reach` reads it through that door. The
sweep for that change found these other dispatchers for this row's
question, outside that map:

- `census::Candidates::build`'s `curved_box` (`crates/topo/src/census.rs`)
  takes a NURBS edge's control hull (`nurbs_curve_aabb`) for pass 2′,
  where `EdgeBoxRule` says `NoSoundBox`: a second NURBS-edge rule beside
  the rule's own NURBS bullet.
- `sweep::blend::reach`'s boundary-edge box (`crates/sweep/src/blend/reach.rs`,
  beside its `circle_arc_aabb` read) and `sweep::blend::surgery::boxed_reach`
  each match `Curve3` and read `geom`'s arc doors directly. Their arms
  differ: one refuses a spiric, the other a circle.
- `editor-core`'s `eval::measure::curve_reach` bounds a reach per
  `Curve3` kind (whole-carrier balls).

## The edge extent's new home (OFFSET, 2026-10-27)

`geom_brep::edge_extent` no longer exists (OFFSET, PR 5875). Its home is `geom_brep::extent`, beside the reach family, as three functions: `edge_extent_inf` (≤ the edge's diameter `D`: the chord, the minor-axis folds and the sampled pairs; the lever of a reading whose Zero only refuses), `edge_extent_sup` (≥ `D`; a reading whose Zero is acted on and whose definite signs only refuse) and `edge_reach_from` (the edge's farthest distance from a point of it; a reading that acts on every sign, read from that point). The premise above that the edge extent "is not this class" no longer holds: `edge_extent_sup` and `edge_reach_from` answer this row's question (an upper bound on an edge's region, about a point or as a diameter) at the `SpanLocate` bound, built on `extent.rs`'s reach family: a circle exactly, an ellipse through `conic_arc_reach` over its span, a spiric through `geom::curves::spiric_rate_bounds`, a spline by the lesser of its control net and a Lipschitz bound over its own window (the samples plus the speed bound). PR 5875 consolidates `certify::edge_extent`, `certify::farthest_pair` and `edge_nurbs::carrier_diameter` into that home, so these three now share one diameter helper (`extent::farthest_pair`). The census's `edge_reach_in` box, `containment::carrier_ball`, `boxes::{conic_extent, arc_extent}` and the maximal-faces ball are untouched and remain this row's.
