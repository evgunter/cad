---
id: new-document-owes-the-reframe-open-gets
kind: issue
title: app.rs re-frames and drops the delta budget for Open only, though NewDocument replaces the document too
refs: [session-clearing-walk-is-hand-maintained-three-times, 1885]
status: open
opened: 2026-09-05
---


Found by the #1885 style review (S4). Pre-existing; #1885's own
declared blind spot — *state derived from the document, held outside
the session, kept correct only because another module happens not to
reach it* — found in a second module.

## What happens

`crates/viewer/src/app.rs:778` computes
`let opened = matches!(op, SessionOp::Open(_));` and uses it at
`:794-798`:

```
None if opened => {
    self.fit_on_scene = true;
    self.fit_delta_on_scene = true;
    self.budget_delta = None;
}
```

under the comment *"A replaced document owes a re-frame AND a fresh δ
— both taken when its scene actually lands, not on the outgoing
picture. The δ the last document was being read at says nothing about
this one."*

`SessionOp::NewDocument` replaces the document too — it is the other
half of the pair `DocSession::clear_for_new_document` exists for — and
it gets none of the three. The comment states the rule over "a
replaced document"; the code matches one op.

## Why it is the same class

`fit_on_scene`, `fit_delta_on_scene` and `budget_delta` are state
DERIVED from the document, held in `ViewerApp` rather than in
`DocSession`, so the session's one-value reset cannot reach them. They
stay plausible after a `NewDocument` only because an empty document is
usually framed acceptably by whatever the last one left — which is a
coincidence about the fixture, not a property.

## Where else to look

The same shape — a viewer-side value derived from the document and
reset (or not) by an op match rather than by the session — should be
swept at least at `crates/viewer/src/tools.rs:487`,
`crates/viewer/src/pick.rs:2270-2288` and
`crates/viewer/src/pane/create.rs:141,415`. Cite what the sweep could
not match when it is done.

## What resolving it looks like

Either the app asks the session whether the document was REPLACED
(one predicate, both ops) instead of matching one op's shape, or the
three fields move to where the replacement is already handled once.
The predicate is the smaller change and it puts the rule in the same
place as its sentence.
