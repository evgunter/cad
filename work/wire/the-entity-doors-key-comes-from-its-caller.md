---
id: the-entity-doors-key-comes-from-its-caller
kind: issue
title: The entity door's kind is unforgeable but the key it is read off is the caller's, so a road can still refuse about the wrong entity
status: closed
opened: 2026-09-13
priority: P1
cost: M
closed: 2026-10-18
pr: 5264
branch: wire/selection-kind-at-the-door
---


## Finding

Found by the delta review of PR 2517, against that PR's own claim. It
is a correction to the reasoning the unit was built on, not a defect
the unit introduced — the same gap existed in every earlier shape of
this code, and the row records it because the PR's doc briefly said it
did not.

`crate::eval::entity_door::entity` reads the kind off the key it is
HANDED:

```rust
read(key).ok_or_else(|| refuse(Found(key.kind())))
```

`Found`'s field is private, so no road can write a kind. **But
`EntityKey`'s variants are `pub` and its payloads are slotmap keys with
`Default`**, so a road can hand the door a synthetic key of whatever
kind it likes. The door then truthfully describes THAT key and falsely
describes the entity the road was talking about.

The reviewer compiled the sharp form: `resolve_open_faces` resolving
the real key through `ladder::resolve_in`, projecting through
`ent.key.face()` so the success path stays byte-identical, and handing
the door a forged `EntityKey::Vertex`. It compiled, and the source
census passed on it. Only the byte-exact document rows caught it.

## What PR 2517 did about it, and what is left

**Closed**: the door's `read` is a `fn` pointer rather than a closure,
so it cannot capture a second key. The value the door returns and the
kind it reports now come off the SAME key. That is what made the
attack above invisible — a road that substitutes a key now substitutes
it for its own success path too and stops working, rather than
succeeding on one entity while refusing about another.

**Not closed**: a road can still pass the wrong key. The realistic
shape is not forgery but a mistake — a road that resolved one entity
and handed the door another — and the result is a confidently wrong
refusal. What covers it today is
`crates/editor-core/tests/wire_entity_door.rs`'s byte-exact rows,
which is a textual guard of the kind the token was adopted to reduce
reliance on.

## Why it was not closed structurally

Measured rather than assumed. Closing it means the door taking
something a road cannot substitute — a resolved entity rather than a
bare key. The candidates are `names::EntityKey` and
`names::EntityRef`, and **the crate constructs them in about 150 and
30 places respectively**, nearly all in the naming layer that mints
them legitimately. Making either unforgeable is a naming-layer
redesign with its own design conversation, not a change this unit
could carry.

A narrower option a taker might prefer: give the [`ladder`] a
`Resolved` token minted only by `ladder::resolve_in`, have `Selected`
carry it instead of a bare key, and have the door take that. It fails
today on module privacy — the token's constructor must be private to
the minting module, `entity` must be able to read it, and `ladder`
lives inside `eval::wire` while `entity` cannot. Moving one to reach
the other is the change to weigh.

## Decided (2026-10-17, FORK-ENTITYDOOR, designers converged)

One Opus and one Fable designer, briefed with the problem only, reached
the same final state independently in round 1. The two rows are one
symptom: `NameTable`'s seating doors (`insert_ref`, `insert_tied_ref`)
refuse a row whose name's kind differs from its key's, so the kind a
resolved key answers is always `name.kind` — authored data, readable
with no resolution. The question belongs at the doors that read what
the author wrote, not at evaluation.

**The build:** `Select::fault` gains a per-name kind arm (the name at
`at` is not of the selection's kind), so the edit door, the load door
(`persist/check.rs` `first_selection_fault`) and `Rebind` hold "a
selection of kind K holds only names of kind K"; `SlotKind::selection_kind`'s
`Is(kind)` arm stops ignoring the names' kind, and the measure/pose
lowering checks against the minted `VarKind` rather than `names.first()`.
Evaluation resolves into typed keys through `names::interrogate::key_in::<K>`
(`Selected<K: Denoted>`); `eval::entity_door`, `Found` (and its `lib.rs`
re-export), `NodeErrorKind::SelectKind`, `class.rs` `found_body`, the
`Found` census in `tests/wire_entity_door.rs`, and the blend/shell
`filter_map(EntityKey::edge/face)` silent drops all go. A wrong-kind key
out of resolution is the table's broken invariant: loud, never an
author-facing refusal. The Python `select_kind` tag folds into
`selection_shape`.

Not Ev's: the text it contradicts is agent-written (`Select::fault`'s
doc, `eval::entity_door`'s module doc, `docs/INTENT-STAGE2-SPEC.md`
§Select, kept with reasons by the stage 2 E implementer per DR-141);
no ratified page states the evaluation-time kind check. Stage 3 reach,
not designed here: a saved mate side naming a non-face would refuse at
load rather than resolve to `MateFault::SideUnresolved`
(`tests/edit_one_predicate.rs`), and `mate::member::head_of`'s kind
check becomes a backstop; announced on INTENT's log. One unit carries
both rows.

Signed (WIRE orchestrator).

## Closed (2026-10-18)

PR 5264 (merged `a3a5bf6d05`) built FORK-ENTITYDOOR's decision: a wrong-kind name is refused at the edit door (`EditError::SelectionNameKind`, speaking the node, position, name and both kinds) and the load door (`SelectionFault::Kind`); evaluation resolves typed keys through `key_in::<K>`, with `K` checked once against the selection variable; `eval::entity_door`, `Found` and `SelectKind` are deleted, and the blend/shell silent `filter_map` drops with them. Full review approved with fixes; fix pass taken. Residue: `a-blend-of-no-edges-refuses-at-evaluation`, `a-seating-door-refuses-a-kind-mismatch-as-a-duplicate-name` (WIRE), MSOLVE's `a-mate-sides-docs-name-a-retired-select-kind`.

Signed (WIRE orchestrator).
