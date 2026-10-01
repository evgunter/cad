---
id: how-a-person-sees-a-node
kind: ruling
title: How a person sees a node once node ids are digests, and whether a node's human label is document data the kernel speaks
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

## Narrowed by Ev (2026-10-01, PR 3565)

Ev asked whether the kernel should have a concept of names: "if it
does make sense in the kernel then 1 but with the name if available;
if not, then 2 seems acceptable, with that also using gui-side names
after those are built for things that have a name". A second designer
pair weighed that (fork-log row 24) and converged:

- A node's **label** is document data. It lives in a per-document map
  from node id to `Label`, beside `appearance`, `placements` and
  `witnesses`, and outside `Node`, so it is in neither the id's mint
  preimage nor any content key. A rename recomputes nothing.
- One edit, `SetLabel { node, label: Option<Label> }`. `InsertNode`
  carries no label: a creation that labels is two edits in one undo
  step. Deleting a node drops its label. A label is in the save file
  and the pin.
- Not unique and never identity: references, selections, names and
  Python's `NodeId` hold ids, and nothing resolves a label.
- Kernel sentences spell a node `Extrude "base plate" (3fa9c1d2a0b1)`,
  or `Extrude 3fa9c1d2a0b1` when it has no label. The label is read
  off the document when the sentence is made and is never stored in a
  value the evaluation memo reuses. No namer, and no derived address.
- The GUI's create forms propose an editable "Kind N", stored only
  when the person commits it. The kernel mints no default.
- The term is "label", the word `Attr::Label` already uses for faces
  and bodies, and the two share one validated `Label` type.
