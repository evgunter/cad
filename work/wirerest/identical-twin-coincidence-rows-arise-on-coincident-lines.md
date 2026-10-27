---
id: identical-twin-coincidence-rows-arise-on-coincident-lines
kind: issue
title: A boolean over edges coincident along a line publishes identical twin rows, which the replay tells apart only by ordinal
status: open
opened: 2026-10-19
priority: P3
cost: E
---


Filed by WIRE's `wire-citerows` lane (PR 5352's fix pass), on the
review's question whether identical twins still arise once the spelling
door (`boolean/ops.rs` `spell`) holds misstated rows.

## What

They do. I counted rows of one boolean's output that agree in cells,
relation and site (a probe in `cite_rows`, removed after the run) over
`cargo nextest run -p topo -p editor-core` on the PR's head. Every
twin is `VertexFusion OnCarrier [B Edge, A Edge]`, two edges coincident
along a line decided at two points:
- `three_solids_on_one_line::four_prisms_touching_along_one_line_build_one_body_in_every_member_order`
  (84 twin pairs over its orders);
- `three_solids_on_one_line::the_carried_fold_passes_three_prime_in_every_member_order`
  (168);
- `union_pinch_member_order::solids_touching_along_one_line_build_one_body_in_every_member_order`
  (120).

The cut-in path (`respell_through_cuts`) was not probed.

## Why it matters

`editor-core` `coincide/replay.rs` matches a row to its replay by site
and ordinal, then by cells and relation. Twins pass that match whatever
order the two decisions arrive in, so nothing checks that each twin was
discharged by its own decision's margin. Either spell the two
decisions apart (the meeting's own cells, WIRE unit 2's k-ary row) or
pin the replay per twin.

Signed (WIRE implementer, wire-citerows).

## Slate triage (2026-10-26, CLEAVE)

- Moved to WIRE (`work/wire/`) by `git mv`, id and body unchanged: whose lane filed it: the rows a node publishes and the lineage they are carried by. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).

## Moved to WIREREST (2026-10-27)

WIRE measured 34 budget points against 30, so this P3 row moved by `git mv` (id and body unchanged) to WIRE's second tail. No code cites its path. Signed (WIRE orchestrator).
