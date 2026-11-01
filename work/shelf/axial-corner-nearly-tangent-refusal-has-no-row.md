---
id: axial-corner-nearly-tangent-refusal-has-no-row
kind: issue
title: the axial corner solve's no-resolvable-pair refusal (nearly tangent outside the band, parallel, or missing) has no door-built row
status: open
opened: 2026-10-08
priority: P3
cost: M
refs: [shell-of-a-tangent-dome-refuses-at-the-axial-corner]
---


`topo::offset_axial::solve_corner` refuses `TogetherAxialCorner` with
"no pair of the surfaces here meets transversally enough to resolve
this corner against the edges that end at it — they are nearly tangent,
parallel, or they miss" when no profile pair passes the conditioning
meter and none is tangent inside the band (`tangent_foot`). No test
reaches it.

The tangent bullet (`sf2b_axial`) was its one row: a line–circle pair
too ill-conditioned to resolve. Since the tangency fallback, that pair
takes its foot and the bullet hollows. What could still reach it:

- a line–circle pair nearly tangent OUTSIDE the band, yet with
  `det·arm` inside it — `det ≈ √(2·gap/r)` makes that need a gap of
  about `r·band²/(2·arm²)`, below the band, so for line–circle it may be
  unreachable at sane arms;
- two parallel line profiles (two coaxial walls, or two stations, at
  one corner);
- a pair that misses.

The module doc's ledger names it as having no row. The item is a row
that reaches it, or a demonstration that one of the arms above cannot be
reached and its refusal text narrowed to what can.
