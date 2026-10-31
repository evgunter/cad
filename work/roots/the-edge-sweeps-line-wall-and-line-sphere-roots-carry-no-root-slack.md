---
id: the-edge-sweeps-line-wall-and-line-sphere-roots-carry-no-root-slack
kind: issue
title: The edge sweep's line × wall and line × sphere roots carry no rounding slack, where its line × cone roots do
status: open
opened: 2026-10-31
---


Found by FLUSH's `flush/aimed-ray-reach`. That branch gives
`line_wall_roots` and `line_sphere_roots` (`boolean/solid_contain.rs`)
a first-order bound on each root's rounding, `WallRoots::Two(roots, slack)`,
and reads both quadratics about the line's closest approach to the axis
or centre (`Centred`).

The edge sweep's door (`boolean/reduce.rs` `line_wall_roots_of`, the
two `WallRoots::Two(ts, _) => two(ts)` arms) drops that slack: a line
edge's wall and sphere roots go on as exact. Its cone arm does meter its
roots (`line_cone_roots`, `bool_line_cone_root_slack`), and that arm
still reads the cone quadratic about the edge line's own origin
(`line_cone_quadratic`), so its `C` is `|w₀|²`-scaled.

Owed: carry the wall and sphere slack through the edge door, as the cone
arm carries its own. Settle whether the cone arm should read about the
closest approach (`line_cone_centred`), so that all three are one form.
