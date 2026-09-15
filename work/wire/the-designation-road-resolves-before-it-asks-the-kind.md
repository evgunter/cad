---
id: the-designation-road-resolves-before-it-asks-the-kind
kind: issue
title: named_entity and the measure reference resolve a name before asking what it denotes, so a tied name of the wrong kind refuses Ambiguous
status: open
opened: 2026-09-15
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

`named_entity` is written as resolve-then-ask:

```rust
let ent = ladder::resolve_in(name, doc, table, unresolved)?;
super::entity_door::entity(ent.key, read, |found| refuse(Box::new(name.clone()), found))
```

`ladder::resolve_in` ends in `ladder::resolve`, whose `Landing::Tied`
arm refuses `ResolveError::ambiguous`, so `entity_door::entity` is
never reached for a tied name. Its three callers — a fillet/blend
selection (`resolve_selection`), a shell designation, a derived
frame's face — therefore answer a UNIQUE name of another kind with
their own kind refusal, and the SAME name tied with "go narrow it",
where narrowing could not have helped.

The measure reference is the same pairing split across two functions:
the wiring loop resolves through `ladder::resolve_in` under
`NodeErrorKind::MeasureRefResolve`, and `Selected::faces` asks the
kind afterwards under `NodeErrorKind::MeasureSelectionKind`. A tied
edge name in a `min_clearance` leaf gets `MeasureRefResolve
{ Ambiguous }`; the unique one gets `MeasureSelectionKind`.

## Why it is one row and not four

All four refusals carry `eval::entity_door::Found`, whose field is
mintable only inside `entity_door` and is built from the resolved
`EntityKey`. Asking the kind first means the token has to be mintable
from a NAME's kind, which is one change in `entity_door` that all four
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

- `crates/editor-core/src/eval/wire.rs`, `named_entity` — the road.
- `crates/editor-core/src/eval/wire.rs`, `resolve_selection` and the
  shell/derived-frame callers of `named_entity`.
- `crates/editor-core/src/eval/wire.rs`, the measure wiring loop and
  `Selected::faces` — the same pairing across two functions.
- `crates/editor-core/src/eval/mod.rs`, `entity_door` — where `Found`
  is minted, and the only thing that has to change shape.
- `crates/editor-core/src/names/interrogate.rs`, `read`/`entity_of`,
  and `crates/editor-core/src/assembly.rs`, `resolve_face` — the same
  rule already landed, for the shape to copy.
- `crates/editor-core/tests/wire_entity_door.rs` — the census that
  walks these refusals and will want a tied row.
