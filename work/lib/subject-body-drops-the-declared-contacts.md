---
id: subject-body-drops-the-declared-contacts
kind: issue
title: checks::subject_body drops the declared contacts sources_of produced beside the body
status: open
opened: 2026-09-03
refs: [LIB-B-VALIDATE4]
---

Measured at LIB-B-VALIDATE4 while auditing every Python `Body`
construction site for the declarations the new fourth validator rung
certifies against. An `editor-core` signature, so filed rather than
taken.

## The measurement

`crates/editor-core/src/checks.rs:828`:

```rust
pub fn subject_body<T: Decide>(
    ev: &Evaluation<T>, root: RecipeNodeId, output_ix: u32,
) -> Option<Arc<Body<T>>> {
    let sources = product::sources_of(ev.value(root)?)?;
    sources.into_iter().find(|(ix, _, _)| *ix == output_ix)
        .map(|(_, body, _)| body)
}
```

`sources_of` returns `(u32, Arc<Body<T>>, Arc<ContactRecords>)` — it
exists precisely to reconcile the two homes a record set can have
(`NodeValue::contacts` for an instantiate's carried D-1 declarations,
`BooleanValue::contacts` for a boolean's own). The third element is
then discarded by the `.map`.

## The consequence at the boundary

`pncad.subject_body` is the door from a check finding's attribution
back to its subject. Since the records do not survive the call, the
Python `Body` it answers with is plain — so a subject that IS a
declared boolean result reports its own certified seam as an
undeclared contact under `Body.validate_pseudomanifold`, while the
identical body read through `Value.body` passes.

It fails LOUD, never silently: an absent record set can only make the
tier-3′ gate refuse (`UndeclaredContact`), never bless anything. That
is why this is filed rather than worked around at the boundary — a
binding-side guess about which records belong to a subject would be
the invention the F1 contract forbids.

The narrowing is stated at the door
(`crates/pncad-py/src/py/checks.rs`), pointing here.

## The fix, when someone takes it

Widen `subject_body` to return the pair `sources_of` already built —
`Option<(Arc<Body<T>>, Arc<ContactRecords>)>` — and have the binding
call `Body::declared`. The information is not missing; it is dropped
one line after being computed. Callers of the Rust door are few and
the change is mechanical.
