---
id: index-request-and-index-inputs-are-one-concept-twice
kind: issue
title: IndexRequest and IndexInputs carry the same five fields, owned and borrowed, and nothing says they are one concept
refs: [pick-and-parts-name-the-session-driver, 1953]
status: open
opened: 2026-09-06
---


Disclosed by the #1953 style review, which asked whether
`IndexInputs` is a concept or an argument bundle sized to one caller.

## The duplication

`evalseam::IndexRequest` and `pick::IndexInputs` carry the same five
things: generation, document, evaluation, δ, ε. One owns them (the
worker's copy, `Doc` by value and `Arc<Evaluation>` cloned); the other
borrows them (`&Doc`, `&Arc<Evaluation>`) at the one door that mints
it, `DocSession::index_inputs`. `PickCache::sync` takes the borrowed
one and builds the owned one from it, field by field, plus δ.

Three call sites spell the same trio out by hand rather than taking
either: `crates/viewer/tests/select_pick.rs:45-51`, and
`crates/viewer/src/app.rs` and `crates/viewer/src/pane/create.rs` at
their `PickIndex::build` and index reads. So the shape recurs whether
or not a type exists for it.

## Why `build` must NOT take `IndexInputs` — the part that is hard to
## rediscover

`PickIndex::build` is the PURE function, and the worker runs it off an
`IndexRequest` **that owns its copies**. That ownership is the seam's
contract, stated at `IndexRequest`'s own docs: the document is cloned
into the request and the evaluation is shared *so the worker owns
everything it reads and the session goes on being edited*.

`IndexInputs` is a **borrow of a landing**. Handing it to `build`
would put a session-borrowed value on the far side of a contract whose
whole point is that nothing over there borrows from a session — the
lifetime would either fail to compile at the seam or force the seam to
hold a borrow across a thread boundary it exists to avoid.

So the two types are not redundant today, and collapsing them into one
by giving `build` the borrowed shape is the "simplification" this item
exists to stop. Anyone reading `IndexInputs::of` beside
`IndexRequest`'s five fields will see the duplication before they see
the seam.

## The open question

Whether they should be ONE type parameterised over ownership — a
`Landed<'a>` and a `Landed<'static>`, or a pair generated from one
declaration — or whether two types with a stated relationship is the
right answer and only the relationship is missing.

The cheap answer is the second: say at each type that the other is its
counterpart and which side of the seam it lives on, so the next reader
meets the seam before the duplication. The expensive answer is worth
costing only if a third spelling appears; two is the number at which a
relationship is worth writing down and not yet the number at which a
mechanism pays for itself.
