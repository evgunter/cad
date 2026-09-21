---
id: interrogate-read-answers-a-tie-before-the-door-s-kind
kind: unit
title: interrogate::read refuses Ambiguous for a tied name of the wrong kind, where a unique one of that kind answers WrongKind
status: closed
opened: 2026-09-15
branch: wire/tie-before-kind
pr: 2681
closed: 2026-09-15
---


## Finding

Found by PORT-DOORS-1's §5 sweep (branch `port/doors-1-refusal-order`),
outside that unit's fence. It is the same defect PORT-DOORS-1 closed in
`crates/editor-core/src/assembly.rs`'s `resolve_face` — *a site that
asks how many entities answer to a name before it asks what the name
denotes* — standing at a second door.

`crates/editor-core/src/names/interrogate.rs`'s `read` is the one body
the five public read doors (`face_frame`, `edge_curve`,
`vertex_point`, and their kin) delegate to. It resolves first and asks
the kind second:

```rust
let (body, key) = entity_of(ev, node, name)?;
match K::of(key) {
    Some(k) => Ok(door(body, k)?),
    None => Err(kind_mismatch(K::KIND, key)),
}
```

`entity_of` refuses `InterrogateError::Ambiguous { candidates }` for
an `Entry::Tied` row **whatever the entities' kind**, so it returns
before `K::of` is ever reached. The consequence, at `face_frame`:

- a UNIQUE edge name answers `WrongKind { wanted: Face, found: Edge }`
  — the door tells the caller their reference names the wrong sort of
  thing;
- a TIED edge name answers `Ambiguous { candidates: 2 }` — the door
  tells them to narrow a reference that would not be readable however
  narrow they made it.

Both are refusals and both are true; only the first is actionable, and
which one a caller gets depends on whether the name happens to be
tied. `Ambiguous`'s own doc-comment at the top of the file says a door
"that must pick one refuses with `InterrogateError::Ambiguous`" — but
a face door handed an edge name is not a door that must pick one.

**The kind is answerable for a tie**, which is what makes this a
one-arm change rather than a design question: `NameTable::insert_ref`
and `insert_tied_ref` both refuse a row whose `name.kind` differs from
`ent.key.kind()`, so every candidate of a `Tied` entry carries the
name's kind, and `project`'s only other path to `Entry::Tied` is
`defer::narrow_into`, which goes through `insert_tied_ref` as well.
(PORT-DOORS-1 verified that premise against the tree for its own
change; it is the same table.)

## Why PORT-DOORS-1 did not take it

`crates/editor-core/src/names/interrogate.rs` is **WIRE's** territory,
and the fix is a public error-channel change of its own: the tag a
Python caller branches on moves from `ambiguous` to `wrong_kind` for
these names (`crates/pncad-py/src/py/readback.rs` projects
`InterrogateError`), and the read doors' `# Errors` sections and the
`.pyi` say what refuses when. That is the same shape of work
PORT-DOORS-1 was dispatched to do for `resolve_face`, and it deserves
the same treatment — a decision recorded, then landed with the rows
that pin it — rather than riding along in another program's PR.

There is a second question a lane taking this must answer, which
`resolve_face`'s did not have: `read`'s kind question is the DOOR's
wanted kind (`K::KIND`), not the name's, so the refusal to build is
`kind_mismatch(K::KIND, key)` and a tie has no single `key`. The
candidates all share the name's kind, so `WrongKind { wanted: K::KIND,
found: name.kind }` is the honest answer and `WholeBody` still has to
be reachable for a tied body name — but that is a decision about which
source the `found` word reads from, and
`work/wire/the-declared-pair-refusal-reads-the-authored-kind` is a
sibling row about exactly that choice at another site. The two should
be read together.

## Sites

- `crates/editor-core/src/names/interrogate.rs`, `read` and
  `entity_of` (the `Entry::Tied` arm) — the defect.
- `crates/editor-core/src/names/interrogate.rs`, `kind_mismatch` — the
  refusal the fix has to reach, and the `WholeBody` split it keeps.
- `crates/pncad-py/src/py/readback.rs`, the `E::Ambiguous` arm — what
  moves on the Python side.
- `crates/editor-core/src/assembly.rs`, `resolve_face` — the same rule
  already spelled the other way round, for the shape to copy.

## In review (PR 2681, `wire/tie-before-kind`)

`entity_of` takes the door's `wanted` kind and asks it between the
`NoSuchName` check and the `Entry::Tied` split, so a tied edge name at
`face_frame` answers `WrongKind { wanted: Face, found: Edge }` where a
unique one already did. The second question this row named — which
source the `found` word reads from — is settled with its sibling
`the-declared-pair-refusal-reads-the-authored-kind`: off the NAME,
because no key exists yet. `kind_mismatch` now takes an `EntityKind`,
which keeps the `Body → WholeBody` split in one place and reachable
from a name, so a tied body name still answers `WholeBody`. `read`'s
`K::of(key)` arm stays as the asserted invariant backstop.
