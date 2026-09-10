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

## What one harness would and would not close

`hover-route-for-an-absent-chooser-has-no-test` names the same boundary
for `frame::NO_CHOOSER_BACKEND`'s tooltip, and its shape (2) — *whether
`egui`'s test harness can build a context, draw the toolbar and read a
control* — is the same uncosted question.

**The badge draw is that row's shape exactly**, and a harness that
closes the chooser row closes this half with it: a value reaching a
toolbar widget, unreachable only because `ViewerApp` cannot be built.
Nothing here argues otherwise, and an earlier draft of this row did.

**The guard is genuinely different, but not because it is "not a
widget".** The obstacle is that `remember_theme` is a PRIVATE method:
even a test that could build a `ViewerApp` could not call it, and
would have to drive `update()` and simulate a picker change to reach
it — or the method's visibility would have to change, which is a
design question rather than a harness one. So the honest statement is
*a toolbar harness closes the chooser row and half of this one; the
guard needs a driven `update()` or a visibility change*, and that is
why this is two files rather than one and rather than three.

**Not a reachability question.** Both builds that reach it are ordinary:
the browser, and a desktop viewer launched with neither
`XDG_CONFIG_HOME` nor `HOME`.

## What would settle it

Costing shape (2) once, for both rows, is the whole of the work — the
crate has no test that builds an `egui` context, so whether one can
exist here is unmeasured rather than known to be hard. If it cannot,
the honest outcome is a written reason at both call sites rather than a
test that looks like coverage, which is `#2148`'s rule.
