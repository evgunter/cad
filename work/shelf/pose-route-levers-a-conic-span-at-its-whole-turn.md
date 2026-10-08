---
id: pose-route-levers-a-conic-span-at-its-whole-turn
kind: issue
title: replace_face's C5 pose gate levers a conic edge round its whole turn, not the span it holds
status: open
opened: 2026-10-08
priority: P3
cost: M
---

Found by TANG's span-bounded conic reach (the sweep of Euclidean conic-reach sites).

## What

`replace_face::pose_route` hands `geom_brep::intersect::route_pose` the edge's carrier over `[t0, t1]` as a `Reach::Span`. `route_pose` levers the cone, sphere and torus arms at `Reach::lever_from` from their anchors. For a circle or an ellipse that is `|centre − anchor| + max(|a|, |b|)`, the whole turn, whatever span the edge holds:
- a short arc of a large rim is levered at the rim's size;
- a rim circle on a cone is levered at `h + r` from the apex, where every point of it stands `√(h² + r²)` off.

The span-bounded reach (`Reach::span_reach_from`) would shorten it, but that makes the C5 gate admit poses main escalated. On a declared path that is a widening under D10, which needs its own differential, with truths, over the cone, sphere and torus arms. TANG left `lever_from` unchanged.

## The shape of a fix

Read `Reach::span_reach_from` from each anchor, then run main against head over conic-edged faces on cones, spheres and tori, listing every pose admitted where main escalated, against its truth.
