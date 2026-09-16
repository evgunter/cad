---
id: the-canceled-label-names-a-cause-a-dead-worker-did-not-have
kind: issue
title: The chrome labels a picture nothing will ever refresh 'canceled', naming a cause a dead worker did not have
status: open
opened: 2026-09-16
---



## What

`a-dead-seam-worker-reads-as-an-ordinary-idle-state`'s residue, and the
one half of its bullet 2 that its ruling did not reach.

`frame::Progress::Canceled` is the state the toolbar draws when the
picture is older than the document and nothing is working on it. A
cancel produces it. **So does an evaluation worker that has died**, and
`crates/viewer/src/app.rs` draws the same literal label for both:

    ui.label("canceled — showing an older result");

Nobody canceled anything. The badge beside it now says what actually
happened (`frame::dead_seam_badge`, `Tone::Actionable`) and the
Re-evaluate button beside it is disabled carrying the seam's own words,
so the chrome is no longer silent — but it still asserts a cause, in
the one place a reader looks first, that is false in this arm.

## Why it was not taken with the fix

Two reasons, and the second is the one that matters.

**It is a vocabulary change, not a wording change.** The label is a
string literal inside an `app`-gated `ui` closure, where no headless
row can reach it — the shape `frame::Badge`'s own docs argue against
("where the `None` decision lives decides whether a row can assert
it"). Making it honest at the call site would be the `&&`-inside-a-`ui`-
closure anti-pattern this crate spent #1957 and #2026 removing. Making
it honest properly means a fourth `Progress` arm, which changes
`frame::progress`'s signature, eight call sites in
`tests/frame_policy.rs`, and `crates/viewer/README.md`'s paragraph on
why that function takes the folded value beside one `bool`.

**And it wants the vocabulary question answered first.**
`Progress::Canceled`'s doc says the arm is about *what the picture is
waiting on*, and a dead worker does not change either read it is
composed from — the picture IS older, nothing IS running. Whether
`Progress` should carry a cause at all, or whether the cause belongs
entirely to the badge channel (which is where the ruling put it), is
the decision this row is really about. A lane that adds a fourth arm
without answering it will have made the value carry two questions.

## What it would cost to measure

Nothing to measure: the label is a literal and both producers of the
state have rows already (`tests/frame_policy.rs`,
`a_panicked_evaluator_reaches_the_chrome_as_canceled_not_as_evaluating`
and `a_cancel_keeps_the_last_good_picture_and_reevaluate_recovers_it`
in `tests/eval_seam.rs`). What it costs is the vocabulary decision
above.
