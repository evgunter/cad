---
id: assembly-mint-spells-the-entity-kind-refusal-a-seventh-way
kind: issue
title: The mate mint's NotAFace is a seventh spelling of the entity-kind refusal, invisible to the entity door's census
status: open
opened: 2026-09-13
---


## Finding

Found by the sweep of `work/wire/the-entity-kind-door-has-six-spellings`,
which gave `eval::wire`'s three copies of *read a name, test its
`EntityKey` kind, refuse* one door. That row's census listed six
spellings; this is a **seventh**, and it is on DOCM's ground.

`crates/editor-core/src/assembly.rs`'s `mint_face_ref` (the arm around
`names.lookup(name)`) is the same five lines:

```rust
Some(Entry::Unique(ent)) => match ent.key {
    EntityKey::Face(f) => Ok(f),
    other => Err(refuse(RefusedRef::NotAFace { kind: other.kind() })),
},
```

**What is already right, and should be said first**: it computes the
found kind ITSELF, off the key it was handed, which is the rule the
entity door exists to enforce — a caller never writes the word. Its
sentence renders through `EntityKind::article`/`noun`
(`RefusedRef::NotAFace`'s `Display` arm), so the article agrees with the
kind. There is no correctness defect here.

**What is left** is vocabulary, and it is the thing the WIRE row was
counting: this is a THIRD field name for the same answer — `found` in
`NodeErrorKind`'s four entity-kind refusals, `wanted`/`found` in
`names::interrogate`'s `kind_mismatch`, `kind` here. One question,
three words for its answer.

## Why it was invisible, and what a taker owes

`crates/editor-core/tests/wire_entity_door.rs`'s `source_rules` derives
its subject from the `NodeErrorKind` variants declaring
`found: crate::names::EntityKind`. `RefusedRef` is a different error
type on a different road (mint time, not evaluation), so the census
cannot see this site and says so in its "what these rows cannot see"
paragraph. A seventh spelling added on this road would be caught by
nothing.

Two things a taker could do, and the WIRE unit deliberately did
neither because both land on DOCM's fence:

- rename the field to `found`, which makes the crate say one word for
  one answer and costs one `Display` arm plus the `pncad-py` tag rows
  that pattern it; or
- put the test itself through `eval::wire`'s `entity` door, which would
  have to be made generic over the error type and moved somewhere both
  roads can reach. That is a design change, not a rename.

The first is a rename with no user-visible text change (`Display`
renders the kind, not the field name). The second is a conversation.
