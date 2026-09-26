---
id: sibling-branches-mint-one-step-id-for-different-steps
kind: issue
title: Two edits applied to one base mint the same step id for different steps
status: open
opened: 2026-09-25
priority: P2
---


## What

A step id is unique within one document (`names/README.md`, "N1, the
profile pieces"), but not across the documents that branch from one
value. `Doc::next_step` (`crates/editor-core/src/doc.rs`) is part of the
document value, and `apply` is pure: undo is keeping the prior value.
So after an undo followed by a new edit, or two applies from one base,
each branch mints from the same counter, and the same `StepId` names a
different step in each.

Within one document nothing aliases. A name only crosses between
branches when a caller carries it: a name held in the UI across an undo,
a name copied from one branch's document into the other's. Carried that
way, a name on a step the undone edit minted resolves in the other
branch to whatever step that branch minted under the same number, with
no report.

Node ids share the behaviour: `Doc::next_id` is branch-local in the same
way, and a `StableName` carried across branches can denote a different
node. The step id adds no new class, only a second counter of it.

## Where to start

- The minting sites: `ProfilePayload::mint_step_ids` (`program.rs`) and
  `settle_step_ids` (`edit.rs`); and `next_id` in `doc.rs`.
- The question for the owner is whether ids are unique per document (as
  now, with names not portable across branches by contract), or per
  history, which needs a counter that is not part of the value.

Found by review of #3223.

## Measured (2026-09-25)

`crates/editor-core/tests/asm_parent_held_names.rs`:

- `sibling_versions_mint_one_step_id_and_a_held_name_crosses_between_them`:
  - The part is the 2 × 2 square, extruded. From that one base,
    version A inserts a leg to `(3,1)` after `(2,0)` and version B
    inserts a leg to `(1,3)` after `(2,2)`, each by `SetProgram` with
    the new step's id left to the door. Both mint `StepId(5)`.
  - The parent pins A and paints `InPart { Lateral(Piece { 5, Leg }) }`,
    which denotes A's leg `(2,0)→(3,1)`. It then applies
    `UpdateReference` to B's pin.
  - `maintenance: []`, and the held spelling now denotes B's leg
    `(2,2)→(1,3)`, a step A never had. That is a silent rebind, the
    same outcome the P0 row
    `a-child-documents-rebind-leaves-the-parents-held-names-in-the-old-numbering`
    recorded for positions, now reached only across branches.
  - The row pins that behaviour and is the one that turns when this is
    fixed.
- `sibling_versions_mint_one_node_id_for_different_nodes`: two inserts
  applied to one base mint one `RecipeNodeId` for two different
  extrudes.

**This is reachable through the public doors, with no doctored state.**
The store holds one file per id, the current one
(`pncad::workspace`), and `update_to_store` moves a parent to it. So:
save the part at A, update the assembly to it and hold a name on A's
new step, undo in the part, make a different edit, save, and update
the assembly again. The same holds for two copies of one part file
edited apart, or a file restored from a backup and edited.

**Nothing at the pin update can see the collision today.**
- `UpdateReference` is storeless by design and reads neither version.
- Evaluation resolves only the new version, and N1's check (a name may
  spell only a step the document has minted, refused at or beyond the
  counter) passes, because B's counter is past A's id.
- Neither the parent's name nor its `DocRef` records anything about
  the minting beyond the id. A step's content cannot stand in for its
  identity: N1 keeps a name across a value edit, which changes the
  content.

## The fork

N1 says an id is "unique across the whole document" and that a pin
update "over a child document that kept the step" keeps a name. It
does not say what a document is across branches of its value, or what
a pin update between two branches does. Every fix needs a design
choice N1 does not make:

- **(b) Mint from a chain, not a counter.** A new id is a digest of
  the document's mint chain, which each mint extends with the minting
  edit's canonical bytes. The same edit sequence gives the same ids
  (D9); different edits from one base give different ids, and a held
  name on A's step resolves in B as a step B never minted, so it
  refuses or vanishes loudly. It needs no check at the pin update. It
  changes N1's text: ids are no longer "minted from the document's
  monotone step counter", and "at or beyond the counter" becomes
  membership in the document's mint log, which the document must then
  carry. Node ids would change the same way, and they are spelled
  across the whole tree.
- **(a) Record provenance with the pin.** Each document keeps a mint
  log, a digest per minted id, and a `DocRef` records the log's state
  when it pins. Evaluation, or the store-aware update, checks that the
  new version's log extends the recorded one and strands every held
  name spelled at or past the divergence. This keeps ids small, but it
  adds persisted state to the document and to the reference's wire.
- **(c) A counter outside the value.** This breaks `apply`'s purity,
  and it cannot tell two applies from one base apart. Rejected.
- **(d) Names are per-lineage by contract.** Keep the ids as they are,
  write into N1 that a pin update between versions that do not descend
  one from the other is unsupported, and leave it silent. This is the
  status quo written down.

Recommendation: (b). It makes "never reused" true of the history
rather than of one value, it is deterministic, and it removes the need
for a check rather than adding one. Its cost is id width and the node
ids' blast radius. Taking it for step ids alone, now, and node ids as a
second row, would bound that cost.

## Ruled (2026-09-25)

Ev chose (b) on #3262 ("(b) sounds good!"). The step id carries its own
lineage: it is minted from a digest chain that each minting edit
extends with its canonical bytes. The same edit sequence mints the same
ids (D9), and siblings never share one. A carried name from the other
branch resolves `Vanished` wherever it is read. The mint log refuses a
duplicate, and the doors refuse an id the log does not hold. The rule
is in N1's "The id." bullet.

This row is now the build for step ids. Node ids stay on their counter
for now; moving them too is a later row, because the blast radius is
large.

Ev asked whether this was a defect in `UpdateReference` instead. It is
not: a counter id cannot tell "step kept and edited" from "two steps
sharing an id", so any check at the reference update needs the lineage
the id should carry.
