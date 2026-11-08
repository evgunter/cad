---
id: conic-germ-pairs-rank-by-the-frames-turn-not-the-foot-parameter
kind: issue
title: The conic germ pairs still on the frame lane rank partners by the frame's turn, not the section value's foot parameter
status: open
opened: 2026-10-31
priority: P1
cost: H
---


Filed by the SECTARMS orchestrator (2026-10-31) from its survey of
`the-walk-order-is-spelled-twice`.

## What

The boolean's join orders a germ's partners along a section in two ways:

- **The carried lane** orders by foot parameter on the section value:
  `along_facing` → `Along { travel, speed }` → `nearer_along`'s first
  branch (`bool_join_foot_ahead`, `bool_join_foot_travel`). It serves
  only pairs that `geom_brep::pair_section` builds.
- **The frame lane** orders by the frame's turn: `Turn::facing`,
  `germ_arm`, `travel`, `turned_past`, `off_conic_slack`
  (`bool_join_arc_ahead` / `_travel` / `_clear` / `_slack`). It serves
  every conic germ pair still routed `Elsewhere`: plane × cylinder,
  plane × sphere, plane × cone, sphere × sphere, plane × torus, and the
  equal-radius cylinder bisector ellipses.

SECT's ratified design says foot parameters replace frames
(`work/sect/plan.md`, "The design"). The frame lane's spelling retires
only when those pairs move onto the value, and no row names that move.

## Done when

The conic pairs are on the section value, ordered by foot parameter, and
the frame lane's turn ordering is gone.

Landing this also retires SECTTAIL's
`the-travel-slack-could-be-read-away-near-the-site` and
`the-travel-slack-charges-a-partner-at-the-sites-cot-psi`, and FLUSH's
`conic-germs-pair-order-is-unargued`. The bisector ellipses depend on
the crossing-node question under
`skew-cylinder-germ-pair-has-no-section-frame`.
