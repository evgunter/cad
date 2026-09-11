---
id: product-table-answers-a-tie-before-kind-the-operand-the-reverse
kind: issue
title: The gate answers Ambiguous for a tied non-face in the product's own rows but NotAFace for the same tie at the operand
status: open
opened: 2026-09-06
---



Left by MSOLVE-5 (PR 2090), pinned by value in
`crates/editor-core/tests/msolve5_read_below_a_root.rs` (the tied-face
and tied-edge rows assert their controls at the pattern root).

`resolve_face` (`crates/editor-core/src/assembly.rs`) matches the
PRODUCT's own rows tie-first: `Entry::Tied` is `Ambiguous { width }`
whatever the entities' kind, and only a `Unique` non-face is
`NotAFace { kind }`. `operand_answer`, the sibling MSOLVE-5 added for
the operand's table, was ruled kind-first: a non-face entry, unique or
tied, is `NotAFace` because a non-face never mints anywhere, so what
it is precedes where it is rooted. The same tied edge therefore
answers `Ambiguous` read at the root and `NotAFace { kind: Edge }`
read one node below it. Both are refusals and both are true; only the
first word differs. MSOLVE-5's spec put `Ambiguous`/`NotAFace`
semantics out of scope, so the product match was left verbatim.

What is owed: one order for both tables — kind-first in the product
match too (a one-arm change: the kind is the name's, which the table
enforces for every candidate), with the `display_contract` and the
two control assertions moved — or a stated reason the product's rows
answer differently.
