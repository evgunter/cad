---
id: assembly-mint-spells-the-entity-kind-refusal-a-seventh-way
kind: issue
title: The mate mint's NotAFace is a seventh spelling of the entity-kind refusal, invisible to the entity door's census
status: closed
opened: 2026-09-13
branch: edit/error-prose
pr: 2719
closed: 2026-09-16
---


## Finding

Found by the sweep of `work/wire/the-entity-kind-door-has-six-spellings`,
which gave `eval::wire`'s three copies of *read a name, test its
`EntityKey` kind, refuse* one door. That row's census listed six
spellings; this is a **seventh**, and it is on this program's ground (it was filed
against DOCM, which closed while the PR was in review; `assembly.rs`
is EDIT's territory now).

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
neither because both land on this program's fence:

- rename the field to `found`, which makes the crate say one word for
  one answer and costs one `Display` arm plus the `pncad-py` tag rows
  that pattern it; or
- put the test itself through `eval::wire`'s `entity` door, which would
  have to be made generic over the error type and moved somewhere both
  roads can reach. That is a design change, not a rename.

The first is a rename with no user-visible text change (`Display`
renders the kind, not the field name). The second is a conversation.

## Built (2026-09-16)

The first option, as the row scopes it: `RefusedRef::NotAFace`'s field
is renamed `kind` -> `found`, so the crate says one word for one answer
at every entity-kind refusal. The second option — a generic entity door
across both roads — is a conversation and was not built.

**Cost, as predicted, and nothing else.** The `Display` arm; three
construction sites in `assembly.rs` (`mint_face_ref`'s kind gate, its
non-face key arm, and the diagnosis in `refused_ref`); one `pncad-py`
pattern, `crates/pncad-py/src/py/assembly.rs`'s `kind` getter, which is
LIB's file and a mechanical follow-through; and five test patterns plus
three doc-comment spellings in
`crates/editor-core/tests/msolve5_read_below_a_root.rs` and
`display_contract.rs`.

`crates/pncad-py/src/tags.rs`'s `ref_not_a_face` row and
`test_binding_census.py`'s `"RefusedRef::NotAFace"` entry are keyed on
the VARIANT, not on its field, so neither moved. **No user-visible text
changed** — the sentence renders `found.article()` and `found.noun()`,
which is what it rendered before under the other name — and no variant
was reshaped, so the tags contract is untouched.

**What is still invisible.** `crates/editor-core/tests/wire_entity_door.rs`'s
`source_rules` derives its subject from `NodeErrorKind` variants
declaring `found: crate::names::EntityKind`. `RefusedRef` is a different
type on a different road, so the census still cannot see this site and
still says so in its "what these rows cannot see" paragraph. The rename
makes the two roads agree on the WORD; it does not put this road under
the guard, which is what the row's second option was for.

One more `NotAFace` sits on a third road and is deliberately untouched:
`clearance.rs`'s `SelectionRefusal::NotAFace` carries `name: String` — a
rendered name, not a kind — so it answers a different question and the
word `found` would not fit it. It is not an eighth spelling of this
answer; it is a different answer.

## Closed (2026-09-16, EDIT orchestrator)

Merged as PR #2719 on green CI (run 35047419997, full matrix) and the
orchestrator's read. Residue is in its own files, named in the Built
section above.
