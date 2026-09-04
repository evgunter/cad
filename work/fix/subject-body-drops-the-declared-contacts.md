---
id: subject-body-drops-the-declared-contacts
kind: issue
title: checks::subject_body drops the declared contacts sources_of produced beside the body
status: closed
opened: 2026-09-03
refs: [LIB-B-VALIDATE4]
closed: 2026-09-03
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

## Closed

`editor_core::checks::subject_body` now returns the pair
`product::sources_of` already built —
`Option<(Arc<Body<T>>, Arc<ContactRecords>)>` — and the Python door
(`crates/pncad-py/src/py/checks.rs`) calls `Body::declared` instead of
`Body::plain`. The Python signature is unchanged (`Optional[Body]`):
the records are CAPTURED on the body, which is the same
carrier-projection `Value.body` uses, so the two doors cannot disagree
about what was declared over one body. The Rust callers were the four
the issue named; `dsc_checks.rs` destructures the pair and the two
re-exports needed nothing.

The prose that stated the narrowing is gone: the door's doc comment
now says what it answers with and why both halves travel together (the
two homes a record set has — `NodeValue::contacts` for an instantiate's
carried D-1 declarations, `BooleanValue::contacts` for a boolean's own
— which is what `sources_of` exists to reconcile), and the pointer at
this file is deleted. `pncad.pyi` gained the positive claim; it never
carried the narrowing.

**The regression pin** is
`test_checks.py::TestSubjectBodyCarriesItsDeclarations`, and its red
was seen before its green. The subject is an INSTANCE of the bench
corpus's mated stand — the carried-declaration home, which is the one
of the two homes observable from Python: a declared glue WELDS the
faces it declares, so its record set comes out empty and its 3′ pass
is vacuous (`test_validate`'s own measurement). Against the pre-fix
binding the row failed with

    pncad.ValidationError: validate_pseudomanifold reported 16
    failure(s): tier-3′ census: undeclared contact VertexOnEdge {
    vertex: VertexKey(5v1), edge: EdgeKey(16v1) } at Point3 { x: 0.0,
    y: 0.09, z: 0.5 } — touching must be backed by a declared-contact
    record, never blessed from discovery; …

on the same body that passed through `Value.body` — the issue's
symptom exactly. It passes now. A second row is the control that keeps
the first from being vacuous: the SAME geometry through `product`,
which gathers and declares nothing, still refuses with findings, so
the census really looks at these seats.
