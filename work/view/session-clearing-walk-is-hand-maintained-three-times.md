---
id: session-clearing-walk-is-hand-maintained-three-times
kind: issue
title: the constructor, open, new_document and land hand-maintain the same clearing walk, so a new landed_* field is silently missed
status: closed
opened: 2026-09-04
closed: 2026-09-04
refs: [viewer-session-god-module-split]
---


Found by the #1846 style review reading the whole file rather than the
diff. Pre-existing and not that unit's doing; it is VIEW's ground and
the finding is about `session.rs` as a whole, so it gets a file rather
than dying in a review transcript.

## The duplication

`DocSession` carries six `landed_*` fields
(`crates/viewer/src/session.rs:184-207`) plus `selection`, `hover`,
`scratch`, `path`, `resolver`, `display` and `bounds`. The set of them
that must be reset when the document underneath changes is written out
by hand in **four** places:

- the constructor (`session.rs:297-302`),
- `open` (`session.rs:1082-1101`) — twelve statements,
- `new_document` (`session.rs:1142-1153`) — the same twelve, with
  `path` and `resolver` going the other way,
- `land` (`session.rs:596-625`), which sets six of the same fields.

`new_document`'s doc-comment says outright that it is *"the same
clearing walk as `DocSession::open`, with the two file-shaped fields
going the other way"* — and then the two bodies repeat it statement for
statement. The comment is the tell: the code knows there is one walk
and has no way to say so once.

A seventh `landed_*` field, or any new derived-from-the-document field,
compiles cleanly while one of the four sites forgets it. Nothing in the
tree fails when they diverge; the symptom is a stale badge or a stale
report answering about the previous document, which is exactly what
`open`'s own comment says the walk exists to prevent.

## Two fields already sit outside the walk

- **`bounds`** is in neither clearing walk. It is discarded in
  `request_eval` (`session.rs:1441`) instead, which both walks reach at
  their end — so it is correct today, by a route a reader of either
  walk cannot see.
- **`gesture`** is cleared by neither, and is correct only because
  `SessionOp::permitted_during_value_gesture` refuses `Open` and
  `NewDocument` while a value gesture is open
  (`crates/viewer/src/session/op.rs:612-613`). A field kept consistent
  by a policy table in another module is the sharpest version of the
  hazard: relaxing that table — which
  `save-is-not-gesture-guarded` already asks about the neighbouring
  row — silently breaks a walk that never mentions the field.

## What resolving it looks like

One value for the derived-from-the-document state, reset by
construction rather than by twelve assignments — so a new field joins
it by being declared and `land` writes the same value it clears. The
constructor becoming the one spelling of "nothing has landed yet" is
the check: if the four sites cannot be collapsed to one, the reason is
worth writing down where the walks are.

Rides `viewer-session-god-module-split`'s ground; it is not that
unit's fix, because moving code between modules does not merge two
copies of a walk.

## Closed

`DocSession` holds one `Derived` — `selection`, `hover`, `scratch`,
`landed` and `bounds` — reset by `Derived::none()` at the constructor
and at both doors, and the six `landed_*` fields are one `LandedRun`
written in one place by `land`. The twelve-statement walk is now
`clear_for_new_document`, called by `open` and by `new_document`, and
the constructor's `Derived::none()` is the one spelling of "nothing
has landed yet".

The two fields outside the walk kept their positions, with the reasons
written where the walk is (`Derived`'s docs, `crates/viewer/README.md`):
`bounds` joined the value — so a reader of either door sees it go —
while `request_eval` keeps the stricter per-submit discard that is
what actually makes it correct; `gesture` stayed out, because dissolving
a drag under the pointer is the behaviour `permitted_during_value_gesture`
refuses `Open` and `NewDocument` to prevent, and the reset now ASSERTS
that guarantee (with `scratch.is_none()` beside it) rather than
depending on it silently.

`display` could not join: `DisplayState::clear` deliberately keeps its
revision counter across the reset, so reconstructing it would send the
chrome's rebuild key backwards. It stays a `clear()` call beside the
one assignment, and its own `clear` closes the same hazard for fields
inside it.
