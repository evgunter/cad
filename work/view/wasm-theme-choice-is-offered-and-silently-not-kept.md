---
id: wasm-theme-choice-is-offered-and-silently-not-kept
kind: issue
title: the browser build offers the theme picker and silently does not keep the choice, against two doc claims that it disables and reports
status: open
opened: 2026-09-09
---


Found by `viewer-items-unreferenced-at-wasm32`'s lane while asking that
item's shape-(2) question — *does the browser build swallow a refusal it
ought to say?* The answer for `deliver_status` is no. The answer for the
preferences store is **yes**, and this is where the refusal is.

## What

On wasm `Store` is `prefs::Absent` (`crates/viewer/src/app.rs:94-95`,
`:104-107`), whose `usable()` is `false`
(`crates/viewer/src/prefs.rs:338-340`) and whose `save` returns
`StoreError` (`prefs.rs:331-336`).

The only writer is `ViewerApp::remember_theme`
(`crates/viewer/src/app.rs:1014-1023`), and its first act is

```rust
if !self.store.usable() {
    return;
}
```

— a silent return. Its reporting arm (`self.notices.push(
frame::store_refusal(&error))`, `app.rs:1023`) is reachable only from
`save`'s `Err`, which the early return makes unreachable on the one
target where the store is known unusable.

Its one caller is the palette picker, `app.rs:1409-1415` and
`:1423-1427`: an ordinary enabled `egui::ComboBox` over `Theme::ALL`.
So on wasm a user picks a theme, it applies, the tab is reloaded, and
the default comes back with nothing having said why.

## Why it is a defect and not a trade

Two doc claims in this crate assert the opposite, in as many words:

- `crates/viewer/src/prefs.rs:318-321` — *"**Reports rather than
  pretends**, exactly as `frame::chooser_backend` does for a desktop
  with no portal and no `zenity`: a control backed by this is disabled
  with a reason, never offered and then silently ineffective."*
- `crates/viewer/src/app.rs:88-91` — *"The browser's arm is
  [`prefs::Absent`] until a `web_sys::Storage` store is written — it
  reports, and the Save control disables itself, exactly as the file
  chooser does where no portal exists."*

The file chooser really does do this: `chooser_backend()` answers
`Absent` on wasm (`frame.rs:1600-1619`), `add_enabled(chooser.usable(),
…)` disables Open…/Save As…, and `.on_disabled_hover_text(
frame::NO_CHOOSER_BACKEND)` shows the reason (`app.rs:1220-1221`,
`:1244-1245`). That is #1125's posture. The theme picker is the same
shape with the braces missing, and `app.rs:88-91` names a *"Save
control"* that does not exist — nothing in the chrome is disabled by
`store.usable()`.

## What the fix is not

Disabling the picker would be wrong: the theme **does** apply on screen
and only its persistence is lost, so `remember_theme`'s own doc
(`app.rs:1002-1005`) — *"a failure here costs the next session's memory
of it, never this session's work"* — is the right trade and refusing
the switch is the worse one. The shape to argue is therefore an
annotation rather than a disabling: the picker says, once and not on
every switch, that this build keeps nothing. Note also that a wasm
build with a `web_sys::Storage` store would answer `usable()` true and
the whole question would go away, so a fix should not hard-code the
target.

Whichever shape is chosen, **the two doc claims above are false of the
tree today** and are the minimum this item owes.
