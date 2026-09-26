---
id: hover-route-for-an-absent-chooser-has-no-test
kind: issue
title: the absent-chooser hover route is the only surface and nothing tests it, at any level
status: open
opened: 2026-09-10
refs: [2278]
priority: P4
cost: E
---


Found while building Ev's ruling (c) on
`was-the-status-route-supposed-to-fire-for-an-absent-chooser` (#2278).
That ruling makes the disabled control's hover text the **whole**
surface for "this machine has no file chooser". Nothing in this crate
holds it, at any level, and #2278 removed the one row that came
closest without replacing it.

## The promise, and what carries it

`crates/viewer/README.md:35-46` tells a reader what they will see: the
dialogs are disabled and the reason names **three** remedies — install
`zenity`, install `xdg-desktop-portal`, or pass a document path on the
command line. `platform::NO_CHOOSER_BACKEND` (`crates/viewer/src/platform.rs`) is
that sentence, and its two readers are
`.on_disabled_hover_text(platform::NO_CHOOSER_BACKEND)` at `app.rs:1197`
and `:1227`.

**Nothing asserts either half.** No test in `crates/viewer/tests`
mentions `on_disabled_hover_text`, drives the toolbar, or reads the
const. `chrome_labels.rs`'s own header says why the second half is out
of reach: *"The rest of the chrome's wording lives inside widget calls
and is not testable without a window; this suite claims only what it
can see."* So the gap is not an oversight of one suite — it is where
this crate's test boundary falls.

## What #2278 deleted, and why it was not replaced

`an_empty_dialog_is_loud_only_under_a_confidently_absent_backend` held
four things about `frame::dialog_status`. Three were
`NO_CHOOSER_BACKEND.contains("zenity" / "xdg-desktop-portal" /
"command line")` reached through the deleted `Show` arm's `Message`.
The fourth, `assert_eq!(message.text(), NO_CHOOSER_BACKEND)`, was a
tautology and losing it is a gain.

The three were **not** restored, and that is a judgement rather than an
omission. A `const &str` is fixed at compile time, so no runtime value
can make `contains("zenity")` false — the implementer discipline calls
such a predicate documentation and says deleting it is the repair. The
sharper reason is that restoring them would read as coverage for the
promise above while covering none of it: the untested step is *the
const reaching a tooltip a person sees*, and an assertion over the
const's own bytes cannot reach that step. A test that looks like it
holds a claim it does not hold is worse than a stated gap (#2148).

## Two candidate shapes, neither costed

1. **A gate over the README.** The promise at `README.md:35-46`
   enumerates three remedies and the const is supposed to carry all
   three; a gate that parses the list and greps the const would hold
   the README against the code, which is the half that has a
   mechanical answer. It would not touch the tooltip route.
2. **A chrome-level test that renders the toolbar.** This is the half
   that matters and the half `chrome_labels.rs` says it cannot do.
   Whether `egui`'s test harness can build a context, draw the toolbar
   with `chooser == ChooserBackend::Absent` and read a disabled
   control's hover text is unmeasured here — the crate has no such
   test to copy, so this is a real question and not a step.

**Not a reachability question.** `ChooserBackend::Absent` is
reachable on any machine with no `zenity` and no session bus, and is
what `chooser_backend()` answers unconditionally on wasm. This is a
route a user meets, not a latent arm.

## Evidence from `environmental-facts-answer-usable-as-a-bool-with-the-reason-elsewhere` (2026-09-25)

Shape 2 now exists. `platform::NO_CHOOSER_BACKEND` is gone — the words
are `platform::ChooserBackend::unusable`'s `Some` — and `app.rs`'s
`a_dialog_with_no_backend_to_open_it_says_why_on_its_own_control`
plants each backend on `ViewerApp::chooser`, rests the pointer on
Open… and Save As… in the real toolbar (the `Toolbar` harness beside
it) and asserts the painted hover equals `backend.unusable()`.
`frame_policy.rs`'s
`the_chooser_verdict_is_unusable_only_when_absent_and_says_why_itself`
checks the three remedies the README promises, now over a runtime
match arm rather than a const's bytes. What is left for this row is
shape 1 (the README-list gate), if anyone still wants it; the owner's
call whether it closes.
