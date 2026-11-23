---
id: a-construction-whose-centre-runs-off-still-stores-a-centre
kind: unit
title: a construction whose centre runs off still stores a centre: four mints hold a point of their arc
status: open
opened: 2026-11-22
priority: P1
cost: H
refs: [7258, 7203]
---


Owed by Ev's answer on PR 7258 (2026-11-22, "ok sounds good!";
`docs/DESIGN-FORK-LOG.md` row 296): "a construction whose centre runs
off at a pose it can reach holds the arc"
(`crates/geom-core/README.md`, "Float rounding").

Four kernel constructions whose conic's centre can run off to infinity
at a pose they can reach still mint a centre-held conic:

- the cylinder's tilted ellipse (a cylinder cut almost along its axis);
- `bisector_ellipses` (two equal cylinders almost parallel;
  `geom-brep/src/pair_section/cylinders.rs`);
- blend's oblique end face (`sweep/src/blend/open/end_face.rs`);
- `recognize_curve`'s circle in the STEP import.

Each holds a point of its arc instead, through the arc-held placement
PR 7203 lands for the ellipse (`Held::Arc { phase }`). **The wrong
choice is closed in code, not checked at run time** (Ev, PR 7258: "i
think rejecting ill-conditioned stuff is ok if we can do it as a panic,
i.e., it is a kernel bug if we choose the wrong one"): which constructor
such a mint may call is what stops a centre being stored, since there is
no number for a run-time check to test.

What the unit must find out first, per mint: which point of the arc the
construction holds exactly (PR 7203's cone section holds its near
vertex); whether the arc-held circle PR 7203 removed (no caller then)
comes back for the import's circle; and for the cylinder's shallow
off-axis cuts, whose in-box arc lies about half a radian from any
principal vertex, whether a general anchor is needed first
(`a-section-conic-is-anchored-at-a-general-point-of-its-arc`, filed on
PR 7203's branch).

Ground shared with `sect`, `germ`, `tang`, `importgate`, `exch` and
`band`; a note goes on each live program's log when this is dispatched.

Waits on PR 7203.
