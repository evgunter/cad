---
id: stranded-appearance-keys-are-not-reported-by-dm7
kind: issue
title: A stranded appearance key is not in DM7's report: the walk is Node::payload_names and the appearance store is not a carrier
status: review
opened: 2026-09-16
refs: [2753]
branch: edit/appearance-strands
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

## RULED (Ev, on the `[ev]` PR #2764, 2026-09-16): widen — DM7 covers appearance keys

"Widening seems reasonable." Recorded in DM7
(`crates/editor-core/REFERENCES.md`): the report covers every
reference the document holds under N5 semantics, the appearance store
is the second carrier, and the stranded attachment rides a second
`Maintenance` arm, `StrandedAppearance { name }`. This row is now the
unit that builds it; spec at the next sitting.

## Spec (2026-09-16, EDIT orchestrator) — middle tier: one opus style review with a correctness arm, no A/B row

Branch `edit/appearance-strands`. DM7 as amended
(`crates/editor-core/REFERENCES.md`) is the clause; this builds its
second carrier.

1. `Maintenance::StrandedAppearance { name: StableName }` — a second
   arm, not a `Strand` with a sentinel node (an attachment has no
   carrying node; the store carries it). Its own `Display` sentence in
   the shape of `Strand`'s; the F6 census gains the case.
2. `apply`'s `DeleteNode` arm, after `stranded_names`, walks the
   document's appearance store as it then stands and reports every key
   whose minting node the edit removed. The order contract on
   `Applied::maintenance` extends: payload strands, then appearance
   strands, then the cluster acts — stated on the field with the
   pinning row named. Cost stated at the site as `stranded_names`'s is.
3. Red first: the review's probe
   `rv_a_stranded_appearance_key_is_not_in_the_report`
   (`crates/editor-core/tests/rv_dm7_probes.rs`) asserts absence
   today; rewrite it to assert the report (red), then build. Rows: paint
   a face of `v`, delete `v`, the report names the key and the
   attachment is still in the store (DM7 reports, never repairs);
   `ClearAppearance` on the stranded key still works (the repair path);
   a cascade reports each strand at the step that made it, including a
   key whose carrier the cascade later removes (the transient shape the
   DM7 review named); a live node's key is never reported; the
   round-tripped document reports the same strands. Mutants: drop the
   walk (rows red); walk the document BEFORE the removal (the
   self-referential case reds, as DM7's did).
4. The façade and `pncad-py` (`Maintenance.variant` gains
   `"stranded_appearance"`, the `name` getter, the tag, the census)
   follow mechanically — LIB's, say so. CHROME's
   `cascade-delete-shows-the-strand-count` row defines its end-state
   count over payload names; it gains appearance keys by the same
   definition — append one paragraph to that row from this PR
   (outside the fence, disclosed in the body), do not build the chrome.

## Built (2026-09-16)

DM7's second carrier is at the door. `Maintenance::StrandedAppearance
{ name }` is the arm: no carrying node, because the appearance store
carries the attachment and no node does. It renders its own prose
sentence, naming the store where `Strand`'s names a node and offering
both repairs (`Rebind` moves the key, `ClearAppearance` retires it —
only the second works without a live node to move to). The F6 census
in `display_contract.rs` gains the case.

`apply`'s `DeleteNode` arm calls `stranded_appearance_keys` after
`stranded_names` and extends the same vector, so the order contract on
`Applied::maintenance` reads: payload strands, then appearance
strands, then the cluster acts. The contract is stated on the field
and names its two pinning rows —
`an_appearance_strand_follows_the_payload_strands_of_the_same_delete`
for the first boundary and
`a_mates_head_strands_and_its_read_site_does_not` for the second. The
walk is one pass over `doc.appearance()`'s keys, filtered on
`name.node`; its cost is stated at the site beside `stranded_names`'.
Rows come in the store's own `BTreeMap` order, which is `StableName`'s,
so nothing is sorted.

Rows, in `dm7_delete_strands.rs`: a delete reports the keys it
stranded and leaves every attachment where it is; a key minted by a
live node is never reported; the order row above; a cascade reports
each key at the step that removed its minting node, and the store
outlives the whole cascade; a reported key is still `ClearAppearance`-
able; the round-tripped document reports the same keys. The review
probe `rv_a_stranded_appearance_key_is_not_in_the_report` is rewritten
as `rv_a_stranded_appearance_key_is_in_the_report` — the gap it
measured is closed, and it now asserts the report and the untouched
store.

**Spec correction, measured not argued.** The spec's second mutant —
walk the document BEFORE the removal, expecting the self-referential
case to red — is INERT for this carrier, and the reason is a premise
worth stating: `DeleteNode` does not prune the appearance store (that
is what makes a key stranded rather than gone), so the store is
bit-identical before and after the removal and both walks give the
same rows. Measured: `stranded_appearance_keys(doc, *id)` in place of
`(&new, *id)` leaves all seventeen DM7 rows green. The walk still
takes `&new`, so the two passes read ONE document and cannot disagree
about which nodes are gone, and the function's doc says exactly that
rather than claiming an assertion guards it. The first mutant behaves
as specified: dropping the walk reds six rows.

`pncad-py` follows mechanically (LIB's ground, touched here):
`maintenance_tag` gains `"stranded_appearance"`, the `TAG_INVENTORY`
word, the `name` getter (the `node` getter answers `None`), the stub
and the binding census's variant map. `crates/pncad`'s re-export
comment gains the second carrier. Nothing persisted changed —
maintenance is derived.

CHROME's `cascade-delete-shows-the-strand-count` gained one paragraph
from this PR (outside EDIT's fence, disclosed in the body): its
end-state count gains the store's keys by the same definition, and for
this carrier the pre-click and post-click numbers coincide, because no
cascade deletes the store.
