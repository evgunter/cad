---
id: the-designation-road-resolves-before-it-asks-the-kind
kind: issue
title: a selection resolves a name before asking what it denotes, so a tied name of the wrong kind refuses Ambiguous
status: closed
opened: 2026-09-15
priority: P1
cost: E
rides_with: the-entity-doors-key-comes-from-its-caller
closed: 2026-10-18
pr: 5264
branch: wire/selection-kind-at-the-door
---


## Finding

Found by the caller-side sweep of WIRE's `wire/tie-before-kind` unit
(the three rows it closed:
`declare-door-refuses-a-tie-before-it-asks-the-pairs-kinds`,
`the-declared-pair-refusal-reads-the-authored-kind`,
`interrogate-read-answers-a-tie-before-the-door-s-kind`). It is the
same class PORT-DOORS-1 closed in `assembly.rs`'s `resolve_face` — *a
site that asks how many entities answer to a name before it asks what
the name denotes* — at the **designation road**, which is the one
place left in `crates/editor-core/src/eval/wire.rs` that still has it.

`select` — the evaluation of a selection definition, which every
designation reads through — is written as resolve-then-ask:

```rust
let ent = ladder::resolve_in(name, doc, table, |error| NodeErrorKind::SelectResolve { .. })?;
super::entity_door::entity(ent.key, read, |found| NodeErrorKind::SelectKind { .. })?;
```

`ladder::resolve_in` ends in `ladder::resolve`, whose `Landing::Tied`
arm refuses `ResolveError::ambiguous`, so `entity_door::entity` is
never reached for a tied name. Its readers — a fillet's or chamfer's
edges, a shell's open faces, a derived frame's face, a measure's
references — therefore answer a UNIQUE name of another kind with
`SelectKind`, and the SAME name tied with "go narrow it", where
narrowing could not have helped.

The measure reference has the same pairing a second time, across two
functions: its selection resolves through `select` (a `Measured` seat
admits a face, an edge or a vertex), and `Measured::faces` asks the
scope afterwards under `NodeErrorKind::MeasureSelectionKind`. A tied
edge name in a `min_clearance` leaf gets `SelectResolve { Ambiguous }`;
the unique one gets `MeasureSelectionKind`.

## Why it is one row and not two

Both kind refusals (`SelectKind`, `MeasureSelectionKind`) carry
`eval::entity_door::Found`, whose field is
mintable only inside `entity_door` and is built from the resolved
`EntityKey`. Asking the kind first means the token has to be mintable
from a NAME's kind, which is one change in `entity_door` that both
callers then inherit. `interrogate::read`'s `K::KIND` split (landed on
`wire/tie-before-kind`) is the shape: the guard answers off the name,
and the key projection stays as the invariant backstop that answers
what the key IS.

The premise the repair rests on is verified by construction:
`NameTable`'s `forward` map is private and has exactly two writers,
`insert_ref` and `insert_tied_ref`, and both refuse a row whose
`name.kind` disagrees with a candidate's `key.kind()`. So every
candidate of an `Entry::Tied` carries the name's kind and the kind is
answerable without resolving.

## Sites

- `crates/editor-core/src/eval/wire.rs`, `select` — the road, and the
  one place every designation reaches it.
- `crates/editor-core/src/eval/wire.rs`, `wire_measure` and
  `Measured::faces` — the measure's second pairing.
- `crates/editor-core/src/eval/mod.rs`, `entity_door` — where `Found`
  is minted, and the only thing that has to change shape.
- `crates/editor-core/src/names/interrogate.rs`, `read`/`entity_of`,
  and `crates/editor-core/src/assembly.rs`, `resolve_face` — the same
  rule already landed, for the shape to copy.
- `crates/editor-core/tests/wire_entity_door.rs` — the census that
  walks these refusals and will want a tied row.

## An interaction a taker must not be surprised by

`work/wire/the-entity-kind-door-has-six-spellings` already lists
`names::interrogate`'s `kind_mismatch` as a separate spelling of the
kind word. PR 2681 changed that function's signature from
`(EntityKind, EntityKey)` to `(EntityKind, EntityKind)` and calls it
with `name.kind`, so the `found:` word at that door is now one a
CALLER could write — which is the opposite of the rule
`eval::wire`'s `select` relies on (*a road cannot supply
the word for the kind it found; that word arrives as a token only the
door can mint*).

That was forced, not chosen: the reorder asks the kind BEFORE
resolution, and before resolution there is no key to mint a token
from. The repair this row proposes — making `entity_door::Found`
mintable from a NAME's kind — is the same move at the other door, so
the two rows agree in direction; what a taker owes is deciding whether
`Found`'s guarantee becomes "a door minted this" rather than "a
resolved key produced this", and saying so where the token is defined.
Both doors then state one rule instead of two.

## Triage (2026-10-17, WIRE orchestrator)

Narrowed: the measure half is gone (`MeasureSelectionKind` no longer exists; `wire_measure` goes through `select`, and the seat's kind is enforced at edit). What is left is `resolve_selection` calling `ladder::resolve_in`, whose `Landing::Tied` refuses `Ambiguous`, before `entity_door` asks the kind. The mate-member `resolve_in` call is stage 3 ground and stays as it is. Re-priced `E`; it shares `entity_door::Found` with `the-entity-doors-key-comes-from-its-caller`, so the two go as one unit.

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
