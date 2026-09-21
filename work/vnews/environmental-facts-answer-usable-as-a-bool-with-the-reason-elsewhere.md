---
id: environmental-facts-answer-usable-as-a-bool-with-the-reason-elsewhere
kind: issue
title: one environmental fact now says WHY it is unusable and its sibling still answers a bare bool with the reason kept elsewhere
status: open
opened: 2026-09-10
refs: [wasm-theme-choice-is-offered-and-silently-not-kept, hover-route-for-an-absent-chooser-has-no-test, 2293]
priority: P3
cost: E
---

Disclosed by the close of
`wasm-theme-choice-is-offered-and-silently-not-kept` (#2293), which
changed one of the two and left the other. **Taken out of that PR
deliberately**: the chooser is a settled surface with a ruling of Ev's
on it, and re-shaping it is a decision rather than a detail.

## The two spellings, with opposite polarity

The crate now answers *"can this facility do anything?"* two ways:

- `prefs::PrefsStore::unusable(&self) -> Option<prefs::Unusable>` —
  `None` when it can, and otherwise the store's own words for why not.
- `platform::ChooserBackend::usable(self) -> bool` — true when it can,
  and no words at all.

A reader of `app.rs` meets both: `chooser.usable()` gating the two
dialog buttons, and `self.store.unusable()` at the guard in
`remember_prefs` and at the badge beside the palette picker. Two
adjacent environmental facts, read three lines of scrolling apart,
answering inverted questions.

## The argument for the change applies to the sibling verbatim

`prefs.rs`'s own reason for dropping the bool is *"with a bool the only
party able to say WHY was the store, and the only party with a person
to say it to was the caller, so the sentence a reader saw had to be
composed somewhere that did not know it."*

That is `ChooserBackend`'s situation. Its reason lives in
`platform::NO_CHOOSER_BACKEND`, a `const` composed away from the value and
handed to `on_disabled_hover_text` by the two call sites; the value
that KNOWS the environment carries none of it. `crates/viewer/README.md`
already records that the sweep for that fact has to range over reads of
the field rather than over readers of the string, precisely because the
string and the value can come apart.

**It is not a straight copy of the fix, and that is the question.**
`ChooserBackend` is a closed three-value vocabulary (`ZenityPresent`,
`PortalPossible`, `Absent`) with exactly one unusable arm, so its
"why" is genuinely a constant and `Copy` is worth something; the
store's reason is per-store text. So the candidate shapes are at least
two — `unusable(self) -> Option<&'static str>`, or keeping the bool and
moving the const onto the value as a method — and neither is obviously
right. That is what makes this a decision.

## Sweep, and what its rule ranges over

The population is **every `frame` value that reports what the
environment offers**, not every `-> bool` and not every use of the word
`usable` — the claim is about a fact having a reason a reader needs,
so the rule ranges over the facts:

- `platform::ChooserBackend` — the instance above.
- `platform::Zenity` and `platform::SessionBus`, the two probe readings
  `chooser_backend_of` folds; they feed a value rather than a reader,
  so they may well owe nothing.
- `platform::prefs_path() -> Option<PathBuf>`, whose `None` is the native
  half of the store case; its reason is already prose in its doc and
  reaches no reader.
- the WSL probe.

Enumerating them is part of the work, not a preamble to it: the rule
above is what makes the list falsifiable, and it has not been run past
the four named here.

## What the disabled-control census decides here, and what it does not (2026-09-19)

`a-disabled-control-says-why-in-four-shapes` was run at merge base
`2654cc111417da806d9786c40136106469096fec` and lands on the rule *a
control a reader cannot use owes the sentence a click would have been
answered with — when there is such a sentence.* **That rule does not
decide this row, and this row is untouched by it.** Recorded here
because that census's filing named `NO_CHOOSER_BACKEND` as one of its
own two likeliest genuine hits, and the census concludes it is not one.

**Why it is not.** The two dialog buttons (`crates/viewer/src/app.rs:
1464-1465` and `:1483-1484` — re-derived from the filing's `:1197` and
`:1227`) push no `SessionOp` when the backend is absent. There is no
door, so there is no post-click sentence, so `NO_CHOOSER_BACKEND` is a
**first** composition rather than a second one drifting from a refusal.
`crates/viewer/src/platform.rs` says exactly this at the const: *"A
missing backend is held state, so the disabled control carrying this as
its `on_disabled_hover_text` is the read and there is no status-line
route beside it."*

**The two complaints are disjoint at this site.** The census's class is
*two sentences for one condition*. This row's class is *a value that
knows a fact and carries none of its words* — `ChooserBackend::usable`
answering a bare `bool` with the reason parked in a `const` beside it.
Neither implies the other, and the argument recorded above (the one
`prefs.rs` makes verbatim) is the whole case for this row. It stands
on its own.

**One piece of evidence the census adds.** The one-home principle is
stated in the crate's own doc comments **nine times, in five files** —
`session/refuse.rs` at `Refusal::self_instance`, `Refusal::affordance`
and `Refusal::exists_wording`; `session/op.rs` at `CancelDoor` (one subject,
stated across its two doc comments); `pane/create.rs` at the parts-catalogue entry;
`parts.rs` at `PartEntry`'s `open_document` mint; and
`pane/properties.rs` at the free-move probe, `slot_notes_ui` and the
add-parameter form's already-exists notice. (An earlier version of this
paragraph said "three times, in `refuse.rs`"; that was the count before
the census re-derived it.) Each is the reason a helper or a field
exists, so that a value's words are composed once. That is the same principle this
row applies to `ChooserBackend`, at a value rather than at a sentence,
and it is precedent for the shape *a method on the value that knows*
over the shape *keep the bool and move the const*.

## Evidence added by the `is_instance` lane (2026-09-19)

That lane's sweep ranged over every `-> bool` door under
`crates/viewer/src` (66 of them), which is a different rule from this
row's and therefore a cross-check on it rather than a substitute. Two
things it settled, both narrowing rather than widening the population:

- **`platform::running_under_wsl` is not a member.** It is
  `std::env::var_os("WSL_DISTRO_NAME").is_some() ||
  std::env::var_os("WSL_INTEROP").is_some()` — two reads that cannot
  fail, so its `false` stands over one state and there is no unusable
  arm and no reason to carry. The sweep rule above lists *"the WSL
  probe"* as a candidate; this is the answer for it.
- **`platform::ChooserBackend::usable` is the only `-> bool` in the
  crate that hides an environmental reason**, on that 66-door pass.
  The other bool doors over a lookup either carry no reason at all
  (`is_hidden`, `holds`, `current_for`) or have their typed reason on a
  door beside them (`pickcache::indexing` next to `pickcache::error`,
  `session::select::Selection::live` next to the unresolved verdict) —
  which is the shape this row is asking `ChooserBackend` to take.

Neither touches the row's open decision, which is which of the two
shapes the fix takes.
