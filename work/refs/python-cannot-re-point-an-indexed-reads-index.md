---
id: python-cannot-re-point-an-indexed-reads-index
kind: issue
title: Python has no slot word for an indexed read's index slot, so an index cannot be re-pointed after insert
status: open
opened: 2026-10-10
---


An indexed read's index is a `Count` slot of its node, `SlotId::Index { seat,
k }` (`crates/editor-core/src/node.rs`), written by the ordinary slot door
(`SetStructuralParam`, D3). Python names a slot by one word
(`crates/pncad-py/src/slot_word.rs:48`, `slot_from_word`), and a word cannot
say which seat's index it means, so `DocEdit.set_structural_param(node,
"index", …)` has no spelling: a Python author re-points an index by
re-authoring the read. A spelling naming the seat and the position (e.g.
`"index:target:0"`, or a structured slot argument) closes it; the Rust door
already takes the edit.

Found building FORK-DM4 unit 1's indexed read (`intent/dm4-names-keyed-by-read`).
