---
id: prune-kills-a-gesture-and-reports-nothing
kind: issue
title: prune's third clause kills an in-flight free-move gesture and reports nothing — the class's last member, in the function that fixed the other two
status: closed
opened: 2026-09-05
refs: [prune-discards-the-fault-that-explains-the-supersession, prune-drops-a-hidden-instance-silently, 1886]
closed: 2026-09-11
---


Disclosed by the `view/prune-report` lane (#1886) and filed at
adjudication, not left in the PR body: `work/README.md` is explicit
that a residue named only in prose is invisible to the re-homing sweep
and dies with this program's directory.

## What happens

`DisplayState::prune` reconciles **three** pieces of display state.
#1886 made two of them report and left the third silent. Verified at
that PR's head (`crates/viewer/src/display.rs`, `prune`):

```rust
let gesture_dies = self
    .free_move
    .as_ref()
    .is_some_and(|g| free_move_check(doc, g.instance).is_err());
if gesture_dies {
    self.free_move = None;
}
```

`is_err()` throws away the `DisplayFault` at the instant it is
computed — **the same `is_ok()`/`is_err()`-shaped discard, from the
same predicate, in the same function**, as the defect
`prune-discards-the-fault-that-explains-the-supersession` closed nine
lines above it. The gesture is taken, the revision is bumped, and
nothing is returned about it.

## Why it survived #1886, and why that is not a discharge

It was **defensible before that unit and is less so after**. While
`prune` returned a bare `Vec<RecipeNodeId>` there was nowhere for a
killed gesture to go — it is not an instance whose placement was
withdrawn. #1886 replaced that with `PruneReport`, which has **a field
per kind of withdrawal**, so the argument from "no home" is spent: the
report now has room and the third clause still declines to use it.

The behaviour is *recorded* (`crates/viewer/tests/review_gui4_r1.rs`
carries it as "the current behaviour, not endorsed", and
`OpOutcome::superseded`'s docs said the same), which is the discipline
working — a recorded silence is better than an unrecorded one. It is
not a decision that it should stay silent.

## What the user gets today

Mid-drag, the drag stops. The picture changes and nothing says why.
The only route to the reason is to attempt another free-move op and
read *its* refusal — which is the honesty rule inverted in the same
way `gesture-drags-have-no-cancel-door` describes: the state the user
is in is discoverable only by trying something else and failing.

## What a fix looks like

`PruneReport` grows a third field — the killed gesture and its cause —
and `frame` ranks it with the other two. The wording question is the
one #1886 already answered twice and is worth answering the same way:
a killed gesture is not a supersession either (nothing substituted for
it), so it is a third sentence rather than a third arm of an existing
one. Whether it is news or a standing fact is
`the-news-vocabulary-has-no-expiry`'s question, on `[ev]` PR #1883.

## Home

VIEW's: `crates/viewer/src/display.rs`, `crates/viewer/src/frame.rs`,
`crates/viewer/src/session/op.rs`.

## Closed

Closed by `view/silent-withdrawals`. `PruneReport` grew the third
field the item asked for — `killed_gesture: Option<Withdrawn>`
(`crates/viewer/src/display.rs:563-573`) — and `prune` carries the
fault instead of testing it
(`crates/viewer/src/display.rs:849-862`); the `is_err()` discard is
gone. It is an `Option` and not a `Vec` because a `DisplayState`
holds one free-move gesture, so the kind has no plural.

`OpOutcome::killed_gesture` (`crates/viewer/src/session/op.rs:755-771`)
carries it to the chrome, and `frame::WithdrawalKind::KilledGesture`
words it as a THIRD SENTENCE rather than a third arm of an existing
one, which is what #1886 answered twice: *"free move: the drag in
flight was ended — <fault>"*
(`crates/viewer/src/frame.rs:706-722`, `:736-748`). `app.rs` ranks it
beside the other two (`crates/viewer/src/app.rs:955-964`).

`OpOutcome::from_prune` (`crates/viewer/src/session/op.rs:782-803`)
destructures the report, so a FOURTH kind of withdrawal is E0027 at
the copy rather than silently dropped by the `..default()` spread the
two call sites used.

The recorded-not-endorsed assertion is now the positive one
(`crates/viewer/tests/review_gui4_r1.rs:848-864`): the kill is
reported, with the landing mate as its cause, and it is still not a
supersession.
