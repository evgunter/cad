---
id: wasm-theme-choice-is-offered-and-silently-not-kept
kind: issue
title: the theme picker is offered and its choice silently dropped wherever the store is unusable — two builds reach that, and the typed refusal written for the case is dead
status: closed
opened: 2026-09-09
closed: 2026-09-10
refs: [the-guard-that-decides-whether-a-preference-is-kept-has-no-test]
---


Found by `viewer-items-unreferenced-at-wasm32`'s lane while asking that
item's shape-(2) question — *does the browser build swallow a refusal it
ought to say?* The answer for `deliver_status` (since deleted) is no.
The answer for the preferences store is **yes**, and the browser is one
instance of it rather than its subject.

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
  and `frame::prefs_path` (`frame.rs:1681-1686`) returns `None` when
  neither `XDG_CONFIG_HOME` nor `HOME` is set — *"With neither, `None`
  — no path is invented. The caller's store is then unusable and says
  so, which is how a person finds out their preferences are not being
  kept rather than wondering later why nothing was remembered"*
  (`frame.rs:1706-1709`). A desktop viewer launched from a stripped
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
enabled `egui::ComboBox` over `Theme::ALL` (`app.rs:1353-1359`), whose
change arm applies the theme and calls `remember_theme`
(`app.rs:1367-1371`). So a user picks a theme, it applies, the session
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
(`frame.rs:1603-1622`), `add_enabled(chooser.usable(), …)` disables
Open…/Save As…, and
`.on_disabled_hover_text(frame::NO_CHOOSER_BACKEND)` shows the reason
(`app.rs:1174-1175`, `:1193-1194`) — which since the ruling on
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

## Closed (2026-09-10)

**The body above is a claim about the tree at `ceb6f6c41`** — the merge base
this close was built on — and its `file:line` citations are left as they
were written rather than half-repointed onto the tree that replaced
them. The fix moved `prefs.rs` by ~90 lines and deleted two of the
sites it names.

### The channel: a badge, and the rule decided it

`crates/viewer/README.md`'s provenance rule (Ev, 2026-09-06) sorts by
**what caused the sentence to exist**, and `store.usable()` is settled
when the store is built — `FileStore::new(frame::prefs_path())` or
`Absent` — and answers the same on every frame of the run. The sentence
therefore exists on a frame where nobody acted, which is the rule's own
visible test for a read of held state. Ev's ruling on
`was-the-status-route-supposed-to-fire-for-an-absent-chooser`
(2026-09-09) is the same fact one control over and settles the negative
half: *a whole-run environmental fact has no correct sentence on a line
that carries one frame's news*. So it is a `frame::Badge`, not a
`frame::Message` — `frame::prefs_badge`, `Subject::Preferences`,
`Tone::Advisory`, `Affordance::Read`, drawn beside the palette picker.

**Advisory and not Actionable** because nothing inside the session can
give the store somewhere to write and the theme applies anyway; it is
the family's purest report, weaker even than the budget's δ, which at
least has a field beside it a reader can answer with.

**Where it parts company with the chooser**, and this is the annotation
the item asked for rather than a disabling: a file dialog with no
backend can do nothing, so it is disabled with a reason; the picker
applies the theme on the frame it is chosen and loses only the memory,
so disabling it would cost the half that works to protect the half that
does not. The badge is drawn from the first frame rather than after a
switch, which is a better reading of *once and not on every switch*
than the item's own — a reader learns it BEFORE spending a choice.

### The two dead refusals: they went, and the read became the statement

The item's fork was *either the guard goes and `save`'s refusal becomes
the report, or the refusals go and the guard becomes the only
statement*. The second, and the guard's statement is the badge.

Neither `save` could simply drop its refusal — a store that keeps
nothing has no honest `Ok(())` — so what went is the two refusals as
INDEPENDENT PROSE. The condition is now worded once, by the party that
knows it: `prefs::Unusable`, handed back by `PrefsStore::unusable`
(which replaces `usable() -> bool`). The chrome renders those words;
`Unusable::refusal` renders the same words as a `StoreError` for a
caller that saves without asking. Two hand-written refusals for an
unspoken condition became zero, and one value the reader actually sees.

`ViewerApp::remember_theme` keeps its early return and it is now
load-bearing rather than silent: it exists so a whole-run fact does not
reach the outcome channel once per switch, and it says so. Nothing is
discarded there because nothing is attempted.

**What this crate's own call site no longer reaches** is either `save`
refusal, by construction — and that is the decision, not a residue.
`FileStore` and `Absent` are public with public constructors, so the
refusal is the public contract for a pathless construction this crate
never makes, and `tests/prefs.rs` now asserts BOTH (the item noted that
only `Absent`'s were asserted, and by a test no caller matched).

### The predicate, not the target

Every read keys on `store.unusable()`. The population is every read of
`app::ViewerApp::store`, a private field of the crate's only
`PrefsStore` value — four, all in `app.rs`: `load` at the constructor,
the guard, the badge, and the `save` the guard fronts. Complete because
the field is private and `prefs_store()` has one caller. `README.md`
carries that sweep rule beside the chooser's.

### The two false doc claims

Both gone. `prefs::Absent`'s header no longer says a control backed by
it is disabled — it says annotated, and why. `app::Store`'s header no
longer names a Save control that does not exist — it says the `cfg`
decides which store answers and never whether it can keep anything.

### Disclosed rather than fixed

**The door is asserted; the wiring is not.** Four mutations of
`frame::prefs_badge` and of the two stores each turn the suite red (the
PR lists them). Nothing reaches `app.rs`'s guard or the `draw_badge`
call beside the picker, because `ViewerApp` cannot be built in
`crates/viewer/tests` — the same boundary
`hover-route-for-an-absent-chooser-has-no-test` names for the chooser
tooltip, now with a second instance. Filed as
`the-guard-that-decides-whether-a-preference-is-kept-has-no-test`.

### One correction to the item

Its *"Where the user meets it"* says the picker's change arm is
`remember_theme`'s one caller, which was true and is the reason the
guard could hide. It also implies the annotation belongs to the switch.
It does not: the fact is true before any switch, so tying the sentence
to the change arm would have re-made the provenance error in a
quieter place.
