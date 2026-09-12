---
id: verb-error-arity-renders-verbkind-through-debug
kind: issue
title: VerbError::Arity renders VerbKind through Debug and names a Boolean door that does not exist
status: closed
opened: 2026-09-11
branch: fix/verbkind-display
pr: 2368
closed: 2026-09-11
---


(FIX orchestrator) From the `verb-and-dimension-render-through-debug`
lane, PR 2347 — the sweep that closed that item's `profile` half found
this in a **seventh** crate, one none of the parent class's fences
covered.

`crates/verbs/src/run.rs:226` — `VerbError::Arity` renders `VerbKind`
and `Arity`, both fieldless and neither carrying a `Display`, through
`Debug` at a user surface. Same shape as the parent class.

**It is not an oversight, and that is what makes it worth a row rather
than a patch.** The site justifies itself in place and is pinned
byte-for-byte, on the argument that these identifiers are *the doors'
own names* — a user who called `extrude` should be told about
`Extrude`. That argument is sound and it is exactly the argument a
`Display` would make explicit: if the variant identifier IS the
user-facing word, then the type should say so once, in an impl, rather
than every consumer re-deciding that `Debug` happens to be right here.
`VerbKind::ALL` (`crates/verbs/src/verb.rs:151`) is already the
hand-written census that keeps that vocabulary honest, so the word has
a natural home beside it.

Note what PR 2347 established one crate over: `profile::path::Verb`
took its `Display` **on the macro row** (`= "line_to"`), so the word is
declared with the variant and cannot drift from it. `VerbKind` is
hand-written rather than macro-generated, so the same shape costs an
impl — which is the only real decision here.

`crates/verbs/*` is in no open program's `paths` (checked), which is
why this is homed on FIX's slate.
