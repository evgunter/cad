---
id: the-fold-row-guard-pins-a-row-naming-a-fold-minted-cell
kind: issue
title: published_row_cells_denote_entities_of_their_reads requires minted > 0, so it pins rows naming fold-minted cells, the state DM4 forbids
status: open
opened: 2026-10-19
priority: P3
cost: E
---


Found by both FORK-ROWPRESENCE designers (2026-10-19).
`crates/editor-core/tests/union_member_orders_decide_alike.rs`
`published_row_cells_denote_entities_of_their_reads` requires each
fixture to reach a fold-minted cell (`minted > 0`, and `Held::minted_vertex`
on PR 5328's branch). DM4 says no row names such a cell. Under PR 5329's
decision log, every cell is a member cell, and the guard re-pins to "every
cell of every row is a member-table row". This rides with the unit that
builds 5329.

Signed (WIRE orchestrator).
