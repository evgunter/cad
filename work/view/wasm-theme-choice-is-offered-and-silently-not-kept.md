---
id: wasm-theme-choice-is-offered-and-silently-not-kept
kind: issue
title: the theme picker is offered and its choice silently dropped wherever the store is unusable — two builds reach that, and the typed refusal written for the case is dead
status: open
opened: 2026-09-09
---


Found by `viewer-items-unreferenced-at-wasm32`'s lane while asking that
item's shape-(2) question — *does the browser build swallow a refusal it
ought to say?* The answer for `deliver_status` (since deleted) is no. The answer for the
preferences store is **yes**, and the browser is one instance of it
rather than its subject.

**The file name still says `wasm`** because ids are stable
(`work/README.md`); the subject below is not the target.

## The silent return, and the two builds that reach it

`ViewerApp::remember_theme` (`crates/viewer/src/app.rs:1014-1025`) opens

```rust
if !self.store.usable() {
    return;
}
```

(`app.rs:1015-1017`). Nothing is said on that path. Its reporting arm —
`self.notices.push(frame::store_refusal(&error))`, `app.rs:1023` — is
reachable only from the `Err` of `self.store.save(…)` at `app.rs:1022`,
which the early return has already skipped.

`usable()` is a property of the store's **state**, not of the target.
The `cfg` alias at `app.rs:92-95` decides only which store answers it,
and both answers can be `false`:

- **The browser.** `Store` is `prefs::Absent` (`app.rs:94-95`,
  built at `:104-107`), whose `usable` is `false` unconditionally
  (`crates/viewer/src/prefs.rs:338-340`).
- **The native build in an environment that names no config
  directory.** `Store` is `prefs::file::FileStore` (`app.rs:92-93`),
  constructed as `FileStore::new(frame::prefs_path())` (`app.rs:102`).
  `FileStore::usable` is `self.path.is_some()` (`prefs.rs:426-428`),
  and `frame::prefs_path` (`frame.rs:1678-1683`) returns `None` when
  neither `XDG_CONFIG_HOME` nor `HOME` is set — *"With neither, `None`
  — no path is invented. The caller's store is then unusable and says
  so, which is how a person finds out their preferences are not being
  kept rather than wondering later why nothing was remembered"*
  (`frame.rs:1703-1706`). A desktop viewer launched from a stripped
  environment takes the identical early return and says the identical
  nothing — against that clause, which is written in the imperative
  and describes a store that reports.

**This item's first framing named a target** — *"the one target where
the store is known unusable"* — and that is the half-fix waiting to
happen: anything keyed on `target_family` repairs the browser and
leaves the native instance exactly as it is.

## Second instance, one level down: a typed refusal written for this case, dead

`FileStore::save` opens with its own refusal for the pathless store
(`prefs.rs:407-413`):

```rust
let Some(path) = &self.path else {
    return Err(StoreError {
        doing: "save preferences",
        because: "no config directory in this environment".to_owned(),
    });
};
```

That sentence exists for exactly the environment above, and **nothing
can reach it.** `store.save` has one call site in `src/` —
`app.rs:1022` — and `remember_theme`'s `usable()` guard returns before
it whenever `path` is `None`, which is the only condition under which
this arm fires. Nor does any test reach it: the suite constructs
`FileStore` only through `FileStore::at` (`crates/viewer/tests/prefs.rs:189`,
`:202`), which cannot produce a pathless store, and the sole
`FileStore::new` call in the tree is `app.rs:102`.

`Absent::save` (`prefs.rs:331-336`) is the same shape with a test
holding it up: `an_absent_store_reports_rather_than_pretends`
(`tests/prefs.rs:178-184`) calls it directly, so its words are
asserted — but no *caller* reaches them either, for the same reason.

So the crate carries two written refusals for a condition it answers by
returning quietly, and a fix that only annotates the chrome leaves them
dead. Whatever shape the repair takes, the guard and these refusals are
one decision: either the guard goes and `save`'s refusal becomes the
report, or the refusals go and the guard becomes the only statement.

## Where the user meets it

`remember_theme`'s one caller is the palette picker: an ordinary
enabled `egui::ComboBox` over `Theme::ALL` (`app.rs:1419-1425`), whose
change arm applies the theme and calls `remember_theme`
(`app.rs:1433-1437`). So a user picks a theme, it applies, the session
ends, and the default comes back with nothing having said why.

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

The file chooser really does do this, and does it target-blind:
`chooser_backend()` answers `Absent` on wasm and delegates to
`chooser_backend_of` for the environment elsewhere
(`frame.rs:1601-1620`), `add_enabled(chooser.usable(), …)` disables
Open…/Save As…, and
`.on_disabled_hover_text(frame::NO_CHOOSER_BACKEND)` shows the reason
(`app.rs:1179-1180`, `:1198-1199`) — which since the ruling on
`was-the-status-route-supposed-to-fire-for-an-absent-chooser` is the
chooser's WHOLE surface, the status route beside it having been deleted
as misclassified. That is #1125's posture, and it is
stated over a *predicate*, not over a `cfg`. The theme picker is the
same shape with the braces missing, and `app.rs:88-91` names a *"Save
control"* that does not exist — nothing in the chrome is disabled by
`store.usable()`.

## What the fix is not

Disabling the picker would be wrong: the theme **does** apply on screen
and only its persistence is lost, so `remember_theme`'s own doc
(`app.rs:1003-1005`) — *"a failure here costs the next session's memory
of it, never this session's work"* — is the right trade and refusing
the switch is the worse one. The shape to argue is therefore an
annotation rather than a disabling: the picker says, once and not on
every switch, that this session keeps nothing.

**Whatever it is, it keys on `store.usable()` and not on the target**,
which is what makes the browser and the stripped desktop one fix; a
wasm build with a `web_sys::Storage` store would answer `usable()` true
and drop out of the class on its own.

Whichever shape is chosen, **the two doc claims above are false of the
tree today** and are the minimum this item owes.
