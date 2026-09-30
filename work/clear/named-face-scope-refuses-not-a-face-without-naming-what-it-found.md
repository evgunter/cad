---
id: named-face-scope-refuses-not-a-face-without-naming-what-it-found
kind: issue
title: clearance.rs's named face scope refuses NotAFace without the kind, and conflates a wrong kind with a wrong body
status: open
opened: 2026-09-13
priority: P1
cost: E
---


## Finding

Found by the full review of PR 2517 (S2), which gave `eval::wire`'s
entity-kind test and its refusal one home. This is the same question on
the same road, outside WIRE's fence: `min_clearance`'s `Selected::faces`
(converted by that PR) and this site are reached by the same verb.

`crates/editor-core/src/clearance.rs`, in the `FaceScope::Named` arm of
the face-key gather:

```rust
match ent.key {
    EntityKey::Face(k) if ent.body == sel.body => out.push(k),
    _ => return Err(refuse(SelectionRefusal::NotAFace { name: rendered() })),
}
```

Two findings, both about what the refusal does not say.

**1. It names no found kind.** `SelectionRefusal::NotAFace` carries the
name and nothing else, so an author who designated an edge is told
"not a face" and never told what it is. That is the negation-of-expected
shape `work/wire/wire-refusals-answer-found-with-a-negation-of-expected.md`
closed for the value door and PR 2517 closed for the entity doors:
`EntityKey::kind` is one call away and `EntityKind::article`/`noun`
render it.

**2. The wildcard conflates two different faults.** The guard is
`EntityKey::Face(k) if ent.body == sel.body`, so a name that resolves to
**a face of the wrong body** takes the same arm as a name that resolves
to an edge, and gets the same sentence — "not a face" — about a face.
Those are different recipe faults with different repairs.

## Not this unit

`clearance.rs` is SHELL's; PR 2517 read it and did not edit it. The
fix is SHELL's call, including whether `NotAFace` grows a `found` field
(it would be a message change, so it owes an assertion sweep) or gains
a sibling for the wrong-body case.

## Its sibling on this slate

`work/shell/clearance-window-selection-asks-how-many-before-what` (WIRE
filed it 2026-09-15) is the same four lines, asked the other way: the
`let Some(Entry::Unique(ent))` ABOVE this match refuses `Unresolved`
for a TIED name, so neither of this row's two words is reached for one,
and a tie is reported as a name that resolved to nothing. Its repair —
ask the kind before splitting on `Unique`/`Tied` — changes which arm
the wrong-body case here reaches, so the two want deciding together.
