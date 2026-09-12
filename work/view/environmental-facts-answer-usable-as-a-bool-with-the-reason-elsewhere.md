---
id: environmental-facts-answer-usable-as-a-bool-with-the-reason-elsewhere
kind: issue
title: one environmental fact now says WHY it is unusable and its sibling still answers a bare bool with the reason kept elsewhere
status: open
opened: 2026-09-10
refs: [wasm-theme-choice-is-offered-and-silently-not-kept, hover-route-for-an-absent-chooser-has-no-test, 2293]
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
- `frame::ChooserBackend::usable(self) -> bool` — true when it can,
  and no words at all.

A reader of `app.rs` meets both: `chooser.usable()` gating the two
dialog buttons, and `self.store.unusable()` at the guard in
`remember_theme` and at the badge beside the palette picker. Two
adjacent environmental facts, read three lines of scrolling apart,
answering inverted questions.

## The argument for the change applies to the sibling verbatim

`prefs.rs`'s own reason for dropping the bool is *"with a bool the only
party able to say WHY was the store, and the only party with a person
to say it to was the caller, so the sentence a reader saw had to be
composed somewhere that did not know it."*

That is `ChooserBackend`'s situation. Its reason lives in
`frame::NO_CHOOSER_BACKEND`, a `const` composed away from the value and
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

- `frame::ChooserBackend` — the instance above.
- `frame::Zenity` and `frame::SessionBus`, the two probe readings
  `chooser_backend_of` folds; they feed a value rather than a reader,
  so they may well owe nothing.
- `frame::prefs_path() -> Option<PathBuf>`, whose `None` is the native
  half of the store case; its reason is already prose in its doc and
  reaches no reader.
- the WSL probe.

Enumerating them is part of the work, not a preamble to it: the rule
above is what makes the list falsifiable, and it has not been run past
the four named here.
