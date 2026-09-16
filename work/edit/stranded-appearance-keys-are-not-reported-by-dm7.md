---
id: stranded-appearance-keys-are-not-reported-by-dm7
kind: issue
title: A stranded appearance key is not in DM7's report: the walk is Node::payload_names and the appearance store is not a carrier
status: open
opened: 2026-09-16
refs: [2753]
needs_ev: true
---

(Found by the style review of PR 2753, DM7's build; measured by
`rv_a_stranded_appearance_key_is_not_in_the_report` in
`crates/editor-core/tests/rv_dm7_probes.rs`, adopted with that PR.)

## The finding

DM7's letter is `Node::payload_names`, and `stranded_names` walks
exactly that. An **appearance attachment** is keyed by a `StableName`
and is not a node payload — it lives in the document's appearance
store, reached through `DocEdit::SetAppearance` — so the walk does not
see it and the accepted delete says nothing about it.

It is the same shape one door over, by the edit vocabulary's own words:

- `DocEdit::SetAppearance`'s doc (`edit.rs`) states the identical
  carve-out — *"the name's NODE must be live at edit time … A later
  `DeleteNode` MAY strand the attachment (N5 dangling semantics, same
  as Declare)"*. Same check at the same door, same N5 aftermath.
- `DocEdit::Rebind` rewrites appearance keys exactly as it rewrites
  payload names, so the repair DM7 points a stranded name at is
  already the repair for a stranded attachment.
- Evaluation surfaces the loss as a typed
  `appearance::AppearanceLoss`, which is the `ResolveError::NodeGone`
  of this store — so the "loud at the next evaluation, silent at the
  edit" state DM7 exists to end is live here, unchanged.

The probe measures it: paint a face of node `v`, delete `v`, and the
attachment is still in `doc.appearance()` while `Applied.maintenance`
is empty.

## Why it is not built here

**The scope of a ratified clause is Ev's.** DM7 says
`Node::payload_names`; reporting appearance keys would widen what the
clause decides, not implement it. PR 2753 therefore measures the gap
and files it rather than closing it.

## The question for the next `[ev]` sitting

Should DM7's report cover every reference the document holds under N5
semantics — payload names AND appearance keys — or is
`Node::payload_names` the deliberate boundary? The cost of widening is
one more walk at the delete door and one more `Maintenance` arm (or the
same `Strand` arm with the carrier spelled as "the appearance store"
rather than a node id, which is the design question inside the design
question: a `Strand`'s `node` field is a `RecipeNodeId` today and an
attachment has no carrying node). The cost of not widening is that a
user who paints a face and deletes its body is told nothing until the
next evaluation, which is exactly what DM7 calls the limp-along shape.

## Recommendation (2026-09-16, EDIT orchestrator) — on the `[ev]` PR

**Widen DM7 to every reference the document holds under N5
semantics: payload names AND appearance keys.** DM7's reason — an edit
that strands a reference is loud at the edit, not at the next
evaluation — holds one door over by the vocabulary's own words:
`DocEdit::SetAppearance`'s doc gives an appearance key Declare's N5
semantics, `Rebind` already repairs both, and evaluation already
reports the loss typed (`AppearanceLoss`). The walk is one more pass
over the appearance store at the delete door.

**Shape: a second `Maintenance` arm, `StrandedAppearance { name }`,
not a wider `Strand`.** `Strand { node, name }` carries the CARRYING
node and an attachment has none — the store carries it. DM7's text
gains one sentence naming the appearance store as a carrier;
`Node::payload_names` stays the one list of NODE carriers.

**Alternative:** `Node::payload_names` is the deliberate boundary.
Then DM7 gains a sentence saying so and why, and this row closes as
ruled.
