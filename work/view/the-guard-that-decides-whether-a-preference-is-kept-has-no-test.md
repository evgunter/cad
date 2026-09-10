---
id: the-guard-that-decides-whether-a-preference-is-kept-has-no-test
kind: issue
title: the guard and the badge that decide whether a preference is kept are unreachable from the suite, at any level
status: open
opened: 2026-09-10
refs: [hover-route-for-an-absent-chooser-has-no-test, wasm-theme-choice-is-offered-and-silently-not-kept]
---

Disclosed by the close of
`wasm-theme-choice-is-offered-and-silently-not-kept`. That fix has two
ends. **The door end is held**: `frame::prefs_badge`,
`prefs::PrefsStore::unusable` and both stores' refusals are functions of
typed values, and four separate mutations of them each turn
`crates/viewer/tests/all.rs` red — the subject, the tone, the silence,
and the two stores' words drifting apart from what they refuse with.

**The wiring end is held by nothing.** Two sites decide whether a person
ever learns their theme is not being kept, and no test in
`crates/viewer/tests` reaches either:

- `ViewerApp::remember_theme`'s `if self.store.unusable().is_some()`
  guard — the statement that a store keeping nothing is not asked, and
  the site whose SILENCE was the original defect.
- the `frame::prefs_badge(self.store.unusable().as_ref())` draw beside
  the palette picker in `ViewerApp::update` — the sentence itself.

Both live in `crates/viewer/src/app.rs`, one inside an `egui` closure
and one on a `&mut ViewerApp`, and `ViewerApp` cannot be constructed in
the suite: it needs an `eframe` render state. So a lane could delete
either line and every gate this repo runs would stay green.

## Why this is not the chooser row over again

`hover-route-for-an-absent-chooser-has-no-test` names the same boundary
for `frame::NO_CHOOSER_BACKEND`'s tooltip, and its shape (2) — *whether
`egui`'s test harness can build a context, draw the toolbar and read a
control* — is the same uncosted question. **What is new is that the
population is now two and the two differ in kind.** The chooser's
untested step is a const reaching a tooltip. This one has a step the
chooser's does not: a `bool`-shaped CONTROL-FLOW decision in
`remember_theme` that is not a widget at all, and which nothing about a
toolbar harness would necessarily reach. A fix that answers the chooser
row could leave this half open, so the two are costed together or the
cheaper one is mistaken for both.

**Not a reachability question.** Both builds that reach it are ordinary:
the browser, and a desktop viewer launched with neither
`XDG_CONFIG_HOME` nor `HOME`.

## What would settle it

Costing shape (2) once, for both rows, is the whole of the work — the
crate has no test that builds an `egui` context, so whether one can
exist here is unmeasured rather than known to be hard. If it cannot,
the honest outcome is a written reason at both call sites rather than a
test that looks like coverage, which is `#2148`'s rule.
