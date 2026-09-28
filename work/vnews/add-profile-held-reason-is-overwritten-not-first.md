---
id: add-profile-held-reason-is-overwritten-not-first
kind: issue
title: The add-profile form's bore and path arms overwrite blocked, so the frame prompt its comment says comes first is lost
status: closed
opened: 2026-09-25
priority: P3
cost: E
branch: vnews/the-frame-prompt-comes-first
closed: 2026-09-28
---

`ViewerBehavior::add_profile_ui` (`crates/viewer/src/pane/create.rs:986-989`)
sets `blocked = Some(Held::Waiting("pick a frame to draw on"))` first, with
the comment *"Stated before the shape check so the FIRST thing a person is
told is the thing they have to do first"*. The shape-`None` arm (`:995`)
honours that with `blocked.or(..)`, but the bored circle's arm (`:1028`,
`blocked = Some(held)`) and the path arm (`:1071`, `blocked =
Some(Held::Waiting("add a step to the chain"))`) overwrite it. With no
frame picked and an empty chain the reader is told *"add a step to the
chain"* and not *"pick a frame to draw on"*; with no frame and an
over-wide bore, the bore refusal.

Found by #3230's final review, and pre-existing: the overwrites predate
it. #3230 made the bore line loud (`Held::Refused`), which makes the
lost frame prompt more visible, not the ordering different. The fix is
to decide the precedence once (`or` everywhere, or a stated rule that a
refused input outranks a missing one), since the comment and the code
disagree today.

## Closed

The precedence is decided once, by how the reason is built rather than
by each arm: `ViewerBehavior::add_profile_ui` collects every held
reason into a list in the form's top-to-bottom order, and `held_for`
(`crates/viewer/src/pane/create.rs`) picks the one said. The rule it
states: **a refused input outranks the form waiting for one**, and
between two of one kind the earlier in the form wins. A missing frame,
shape or first step shows in the form itself (an empty picker, no shape
chosen, an empty chain); a refused bore looks like any other number in
its field and has no other voice, and it is the one drawn loud
(`Held::Refused`, `Tone::Actionable`). So no frame and an empty chain
says "pick a frame to draw on"; no frame and an over-wide bore says the
bore's refusal, as does a picked frame with one.

Pinned by `tone_tests::a_refused_input_outranks_a_missing_one_and_the_form_order_breaks_ties`
(the helper) and
`properties_pane_tests::the_add_profile_form_says_a_refusal_before_a_missing_input_and_the_frame_first`
(`crates/viewer/src/app.rs`, the whole app with the three planted
combinations plus the frame-picked empty chain).

Sweep for siblings (a held or refused reason assigned rather than
composed) across `crates/viewer/src`: none. The pattern and its blind
spot are in the branch's PR body.
