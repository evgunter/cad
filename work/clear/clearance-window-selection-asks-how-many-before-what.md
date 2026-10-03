---
id: clearance-window-selection-asks-how-many-before-what
kind: issue
title: clearance::windows_of refuses Unresolved for a tied face name before it can answer NotAFace
status: open
opened: 2026-09-15
refs: [named-face-scope-refuses-not-a-face-without-naming-what-it-found]
priority: P1
cost: D
---


## Finding

Found by the caller-side sweep of WIRE's `wire/tie-before-kind` unit,
outside that unit's fence (`scripts/work.py territory` names
`crates/editor-core/src/clearance.rs` as SHELL's). It is the class
PORT-DOORS-1 closed in `assembly.rs`'s `resolve_face` — *a site that
asks how many entities answer to a name before it asks what the name
denotes* — with a second fault stacked on it.

`windows_of`'s `FaceScope::Named` arm reads:

```rust
let Some(Entry::Unique(ent)) = value.name_table.lookup(name) else {
    return Err(refuse(SelectionRefusal::Unresolved { name: rendered() }));
};
match ent.key {
    EntityKey::Face(k) if ent.body == sel.body => out.push(k),
    _ => return Err(refuse(SelectionRefusal::NotAFace { name: rendered() })),
}
```

Two consequences:

1. **Order.** A tied EDGE name refuses `Unresolved`, where the same
   edge name unique refuses `NotAFace`. A clearance window over a
   non-face is not readable however narrow the reference is made, so
   `NotAFace` is the whole fault and is answerable without resolving:
   every candidate of an `Entry::Tied` carries the name's kind
   (`NameTable::insert_ref` and `insert_tied_ref` are the only two
   writers of a row and both refuse a kind disagreement; the `forward`
   map is private, so this holds by construction).
2. **Collapse.** The `else` arm answers `Unresolved` for a name that
   IS in the table — a tie is a naming success, and the refusal says
   the name resolved to nothing. `SelectionRefusal` has no ambiguity
   word at all, so a caller cannot tell a stale selection from a tie
   it could record a choice for. The two repairs are different and a
   taker should decide both.

## Shape to copy

`crates/editor-core/src/assembly.rs`'s `resolve_face` (PORT-DOORS-1,
PR 2635) and `crates/editor-core/src/names/interrogate.rs`'s
`entity_of` (WIRE, `wire/tie-before-kind`): look the name up, refuse
the absent name, ask the KIND off the name, and only then split on
`Unique`/`Tied`.

## Sites

- `crates/editor-core/src/clearance.rs`, `windows_of`'s
  `FaceScope::Named` arm — the defect.
- `crates/editor-core/src/clearance.rs`, `SelectionRefusal` — where a
  tie has no word.

## A third fault, already on this slate

The style review of PR 2681 read the quoted arm and named a third
fact: `EntityKey::Face(k) if ent.body == sel.body` falls through to
`NotAFace` for a name that IS a face, in a DIFFERENT output body of
the same value. That is **not restated here** —
`work/shell/named-face-scope-refuses-not-a-face-without-naming-what-it-found`
is its row and has carried it since 2026-09-13, as its finding 2,
alongside the fact that `NotAFace` names no found kind at all.

The two rows are one site and two questions, so they are cross-linked
rather than merged: that row is about what the refusal SAYS, this one
is about the ORDER in which the arm above it decides anything. A taker
should read both, because the repair for this row (ask the kind before
splitting on `Unique`/`Tied`) changes which arm the wrong-body case
reaches, and the repair for that one decides what it says when it gets
there.
