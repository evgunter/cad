---
id: cone-v-window-reads-a-boundary-arc-as-its-whole-conic
kind: issue
title: replace_face's cone v-window reads each boundary arc's v-range over its whole conic, so a sub-arc's face window is wider than the face, while its doc says the hull IS the face's window
status: open
opened: 2026-10-25
priority: P3
cost: E
---


Found by the sweep of PRED's `D292` (the directional extremum of a
conic arc; PR body has the hit list).

`crates/topo/src/replace_face.rs`, `cone_v_range` (read by
`group_cone_v_window`): a `Circle` or `Ellipse` boundary carrier's
`v`-range is `centre ± amplitude` over the FULL period, whatever its
certified span `(t0, t1)`. `group_cone_v_window`'s doc says the hull of
the boundary's ranges "IS the face's window" and that "nothing is
padded"; for a face bounded by a sub-arc it is the window of the
arc's whole conic, so the window is wider than the face by up to the
arc's sagitta along the cone axis, and the comment beside the arm
("conservative on a sub-arc and exact on a closed rim") contradicts the
function doc.

The exact range now has a door: `v = (p − apex)·axis / cos α` is an
affine functional, so the arc's range is
`geom::curves::boxes::conic_arc_extent_along(carrier, t0, t1, apex, axis)`
divided by `cos α` (comparison-free, runs at the body's scalar). The
fix is that swap; what it changes is the replacement cone chart's
`v`-window on faces whose rims are sub-arcs, so it owes a look at what
reads `v_min`/`v_max` downstream (`replace_face.rs`, the cone arm of the
offset re-chart) and a row with a sub-arc rim.
