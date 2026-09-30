---
id: the-entity-doors-key-comes-from-its-caller
kind: issue
title: The entity door's kind is unforgeable but the key it is read off is the caller's, so a road can still refuse about the wrong entity
status: open
opened: 2026-09-13
priority: P1
cost: D
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
