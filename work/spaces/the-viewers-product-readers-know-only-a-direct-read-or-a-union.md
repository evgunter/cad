---
id: the-viewers-product-readers-know-only-a-direct-read-or-a-union
kind: issue
title: the viewer's product readers (product_reads, is_seen, Takeover::of_var) read the slot by hand and know only a direct read or a ⊔, not a pick or a source
status: open
opened: 2026-11-17
priority: P3
cost: M
---


## What

Found by DOCTAIL's product-seat unit (PR 7087, `work/doctail/split-widens-a-product-reading-one-of-several-cut-copies.md`). The product slot is now a read site like any other (`Doc::read_sites`). It admits a pick and a source written in place (`DocEdit::SetProduct`, `edit.rs` `product_read`), and `split` writes a pick of the instance's copy wherever the product read one of several cut world copies. The editor-core census (`Doc::reader_counts`, `Doc::read_by_nothing_but_the_product`) counts that read.

The viewer's own answers to "what does the product read" are spelled by hand, and each knows only a direct read or a `⊔`:

- `world::product_reads` (`crates/viewer/src/world.rs`) answers the `⊔`'s operands, or the one read otherwise. For a pick it answers the pick variable, and for a source the source variable. Neither is the output of any node.
- `world::is_seen` asks whether one of a node's outputs is among `product_reads`. So a copy the product reads through a pick or a source is "not seen" by the product.
- `session.rs` `Takeover::of_var` sets `product: doc.product() == Some(var)`. A product reading a pick of the taken body, or a source of it, is not re-pointed with the `⊔` seats.
- The other callers of `product_reads` inherit this: `app.rs`, `pane/properties.rs`, `frame.rs` (`ProductOutsideWorld`), and `session.rs` (×2).

## Owed

One home for "the bodies the product reads" that resolves a pick to the piece it names and a source to the body it reads, used by those callers. Add a row in `crates/viewer/tests/` with a product set to a pick of one piece of a copy of a two-piece `⊔`, asserting that the picked piece is seen as the product and the other piece is not. The picture's own case is `a-pick-read-takes-the-whole-copy`.
