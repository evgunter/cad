---
id: how-a-person-sees-a-node
kind: ruling
title: When a kernel message mentions a node, who says which node: the kernel (a hex tag) or the surface holding the document (an address like Extrude 2)?
status: open
opened: 2026-09-30
priority: P1
needs_ev: true
---


## The question

Node ids are moving off the document's counter onto a digest mint
(`sibling-branches-mint-one-node-id-for-different-nodes`, under Ev's
#3262 ruling), so today's "feature 3" becomes a 20-digit number. The
readable number was never a working handle: the feature tree draws no
number on its rows, so a refusal's "feature 3" names a row nobody can
find by reading. About 190 kernel sentences in `editor-core` spell a
node by its raw integer; the viewer and Python show them word for word.

When a kernel message mentions a node, who says which node?

- **The kernel.** Every node and step id gets one context-free
  `Display`, a 12-hex prefix of the digest — the rule documents already
  follow (`DocRef`'s pin prefix, `ident.rs`), where the tree shows the
  file name beside the kernel's line and the kernel never learns it.
  The tree draws the tag on every row so an in-sentence mention can be
  found; friendlier words a surface wants are added beside the
  sentence, never into it.
- **The surface holding the document.** A person reads a node's
  address: its kind and ordinal among live nodes of that kind in
  `Doc::order` (`Extrude 2`). Kernel sentences that mention nodes take
  a namer (id → words); the viewer and Python pass the address namer,
  plain `Display` passes full hex. Every mention reads in the tree's
  own words; the kernel's sentences stop being finished for their node
  references.

Under either answer: the id carries nothing for display, machine
channels print the full 16 hex digits, profile steps stay addressed by
loop and step position, and `eval::schedule`'s id tie-break becomes
the node's position in `Doc::order` so the tree does not draw in hash
order.

Fork-log row 23.
