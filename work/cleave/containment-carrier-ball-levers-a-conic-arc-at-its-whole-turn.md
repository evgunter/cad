---
id: containment-carrier-ball-levers-a-conic-arc-at-its-whole-turn
kind: issue
title: the containment walk's carrier ball levers a conic arc round its whole turn
status: open
opened: 2026-10-08
priority: P4
cost: E
---

Found by TANG's span-bounded conic reach (the sweep of Euclidean conic-reach sites).

## What

`splitting::containment::carrier_ball` bounds a circle or an ellipse edge by its carrier's whole ball, `(centre, radius)` or `(centre, major.max(minor))`, whatever span the edge holds. `extent_from` folds that into the point-in-loop extent as `|centre − from| + radius`, the `point_in_arc_loop_arm` schedule gate's lever.

On a short arc of a large rim the lever is the rim's size, not the loop's. It only over-states, so the gate escalates more than it needs to, never less.

The ellipse arm also takes `major.max(minor)` rather than the larger magnitude. The mint certifies an ellipse stored with a negative `major`; see `Reach::lever_from`.

## The shape of a fix

`geom_brep::Reach::span_reach_from` of the edge's `Reach::Span` gives a span-bounded reach from a point, past the arc by at most a quarter of its bulge. Where a ball is needed, the arc's quarter-chords and bulges bound one too.
