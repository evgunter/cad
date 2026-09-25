---
id: add-profile-held-reason-is-overwritten-not-first
kind: issue
title: The add-profile form's bore and path arms overwrite blocked, so the frame prompt its comment says comes first is lost
status: open
opened: 2026-09-25
priority: P3
cost: E
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
