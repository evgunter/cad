---
id: properties-do-not-say-a-node-is-read-by-the-product-or-a-definition
kind: issue
title: viewer: Properties' read-by lists nodes only, never the product slot or a variable's definition
status: open
opened: 2026-11-22
---

Found in review of PR 7356 (the relations unit). A node's "read by" section (`crates/viewer/src/relations.rs`, `relations`) is `ReadIndex::read_by` (`crates/editor-core/src/doc.rs`, `Doc::read_index`). It inverts nodes' operand rows only.

Two readers the kernel already enumerates in `Doc::read_sites` never show:
- **The product slot** (`ReadSite::Product`). A body the product reads does not say so in its read-by. The Properties pane's product control (`pane/properties.rs`, `product_ui`) says it in other words, and not as a relation row.
- **A variable's definition** (`ReadSite::Definition`). A named source `bracket = B`, a selection or a pose reading a body is not listed. The nodes reading through the variable are listed, at their own seats, with the variable after "through".

A row for each needs a seat that is no `OperandSlot` (`relations::Seat` would gain `Product` and `Definition(var)`). The definition's row also needs a link target: `Selection::Variable`, which a named definition has and an anonymous one does not.

Done when a body the product reads, and a body a named variable is defined over, each list that reader in read-by, linked where it can be selected. The corpus inverse row (`tree_badges::read_by_is_the_inverse_of_reads_over_the_corpus`) must still hold.
