---
id: op-rs-calls-the-panels-admission-test-its-own-copy
kind: issue
title: session/op.rs calls the Properties pane's admission test its own copy, and it is the shared door
status: closed
opened: 2026-09-20
priority: P1
cost: E
closed: 2026-09-21
branch: vseam/gesture-naming
---


Filed by the `is-instance-collapses-absent-and-wrong-kind` lane (VNEWS,
PR #2916), which changed the door the sentence is about. `session/op.rs`
is VSEAM's, so it is filed rather than fixed.

## The sentence

`crates/viewer/src/session/op.rs`, in `SessionOp`'s free-move safety
argument — the middle bullet of *"They meet in three places"* (`:738`
at this lane's merge base):

> the view the scene draws (`DocSession::display_view`) resolves
> against the PREVIEWED one, and so does **the Properties pane's own
> copy of the admission test**, which decides whether to DRAW the
> control the operation then decides whether to ACCEPT

**The pane holds no copy.** `PropertiesPane::instance_ui` calls
`display::instance_check`, which is the same function `drawn_targets`
runs, so the panel's test and the operation's test are one
implementation. It was already loose before #2916 — the call was
`display::is_instance`, also a function in `display.rs` rather than a
hand-written test — and #2916 makes it pointedly wrong by giving the
panel and `drawn_targets` literally the same call.

**The bullet's real claim survives and should be kept.** What it is
about is WHICH DOCUMENT each of the three asks: the operation admits
against the committed document, the pane and the scene against the
previewed one. That is still true — `DocSession::doc()` returns the
scratch document when there is one and `instance_ui` reads `doc()`.
Only *"own copy"* is false, and the repair is a word.

## Why the VNEWS lane did not just fix it

The same phrase in `display.rs` WAS that lane's — its diff invalidated
it, so it landed with the change that caused it. This one is one file
over, in another program's territory, and was already false at that
lane's merge base, which is the line the VNEWS orchestrator ruled:
prose your own diff falsifies is yours; prose that was already wrong is
filed across the fence.

## Closed

2026-09-21, on `vseam/gesture-naming`, as the one-word repair the item
asks for. The bullet now reads *"and so does the Properties pane, which
runs the SAME admission test the view does (`display::instance_check`,
which `display::drawn_targets` runs first)"*. Re-derived by subject:
`instance_check` is `crates/viewer/src/display.rs:375`,
`drawn_targets` calls it at `:486`, and `PropertiesPane::instance_ui`
calls it at `crates/viewer/src/pane/properties.rs:399`. The bullet's
real claim — which document each of the three asks — is untouched.
