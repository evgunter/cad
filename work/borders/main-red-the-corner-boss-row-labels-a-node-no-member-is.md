---
id: main-red-the-corner-boss-row-labels-a-node-no-member-is
kind: issue
title: Main red: emit_union_borders' corner-boss row panics labelling a name whose node is no member, since D′3
status: open
opened: 2026-10-26
---


Found by PR 5742's slow-set run (2026-10-26), and reproduced on a clean `origin/main` at `be2b7a139c`.

`emit_union_borders::the_corner_boss_logs_its_corner_alike_in_every_order_and_every_pair_boolean` is a slow-set row, so main's CI never runs it. It is red on main: `no entry found for key`, raised by `labels[&n.node]` in `edge_label`'s fallback arm (`crates/editor-core/tests/emit_union_borders.rs:677`).

The row came in with PR 5753 (`0077a3492f`). D′3 (#5636) merged after it, and made a copy carry its source's names. The likely cause, not yet shown: a cell's `node` can now be the member's source rather than its placement, and the row keys `labels` by the placements (`placed`, `:607`), so that name has no entry.

**Done when** the row labels a name by the member that carries it under D′3's naming, and passes on main.

## Slate triage

2026-11-01: moved from `work/cleave/` by CLEAVE's split on its semantic seams (Ev, in chat: CLEAVE measured 114 budget points against 30). it is BORDERS' ground (emit_union_borders' corner-boss row); id and body unchanged. (CLEAVE orchestrator)
