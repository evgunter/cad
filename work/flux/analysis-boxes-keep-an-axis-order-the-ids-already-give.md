---
id: analysis-boxes-keep-an-axis-order-the-ids-already-give
kind: issue
title: ParamBox and AnalyzedBox store an axis order that is now always id order
status: open
opened: 2026-10-07
priority: P4
cost: M
refs: [name-order-was-insertion-order-under-the-counter]
---


## What

`AnalyzedBox` and `ParamBox` (`crates/editor-core/src/analysis.rs`)
each store an `order: Vec<VarId>` beside their `BTreeMap<VarId, _>` of
axes, read from `Doc::free_vars`, and `ParamBox::from_axes_in` takes an
order to lay the axes out in. The order was the document's stored
declaration order (`Doc::var_order`) while ids were bare digests. A
variable id now orders as minted (`MintId`, ordinal first), the stored
declaration order is gone, and `free_vars` reads the variable map in id
order — so every such `order` is the map's own key order, carried a
second time.

Readers: `analysis.rs` (`AnalyzedBox::order`, `in_order`,
`ParamBox::from_axes_in`, `ParamBox::order`, `split_axis`), `drive.rs`
(the midpoint box and the axis walk), `clearance.rs`, `stackup.rs`,
`mc.rs`.

## Fix

Drop the stored `order` fields and `from_axes_in`'s order argument, and
iterate the axis maps.
