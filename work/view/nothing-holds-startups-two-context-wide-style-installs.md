---
id: nothing-holds-startups-two-context-wide-style-installs
kind: issue
title: nothing holds that startup performs its two context-wide style installs, and the blocker that argued the gap has gone
status: closed
opened: 2026-09-14
closed: 2026-09-15
branch: view/style-installs
pr: 2692
---


Disclosed by the numeric-field door unit (2026-09-13, its "What is
still not held" section and `work/view/log.md`), whose own row closed
without it — so it has no file of its own until this one.

## What is not held

Startup writes two context-wide styles, and nothing checks that it
does:

- `apply_polarity(egui_ctx, theme.polarity)` — the resolved palette,
  applied before anything is drawn so the window does not open at the
  wrong polarity and flash.
- `crate::widgets::install_number_formatter(egui_ctx)` — the numeric
  rule onto both of the context's styles, two lines below it.

What the crate's rows cover is the RULE each install carries: that a
bare field round-trips its text, that it survives a theme switch. What
no row covers is that **startup performs the install** — either call
line could be deleted and every existing row would stay green.

## Why the old argument for leaving it does not hold any more

The disclosure's reason was the cost of building the subject: *"one
line in `ViewerApp::new`. It takes an `eframe::CreationContext` and no
test in this crate constructs one"*, so a guard would have meant a
whole `scripts/gates/` member with its `ci.yml` pair and a planted
fixture — not proportionate to two call lines.

**That blocker is gone.** `the-toolbar-row-does-not-wrap` (2026-09-14)
split `ViewerApp::new` into `ViewerApp::assemble`, which is every part
of startup that needs no graphics device — both of these installs among
them — and the device half that installs the viewport pipeline.
`assemble` takes an `&egui::Context`, and `app.rs`'s own test module
already builds one and runs the chrome headlessly. A row here is now an
ordinary `--lib` test, not a gate.

## What is NOT settled

**What the row asserts.** The context's `number_formatter` is a
function value, and comparing it is not the reading anyone wants; the
honest read is behavioural — format a number through the context's
style and check the text is the door's, and read the polarity back off
the applied visuals. Which reads are the right ones, and whether one
row covers both installs or each gets its own, is the open part.

## Home

VIEW's: `crates/viewer/src/app.rs`, `crates/viewer/src/widgets.rs`.

## Closed

Two `--lib` rows in `app.rs`'s own test module, one per install:
`startup_states_the_resolved_polarity_on_the_context` and
`startup_installs_the_number_rule_onto_both_of_the_contexts_styles`.
Both installs are on the `assemble` side, so neither needed a gate.

The premise was measured before it was built on: each call line
deleted in turn on a committed tree left the whole viewer suite
unchanged, so no existing row covered either install. With the rows in
the tree each deletion reds its own row and only its own row. Receipts
are in the PR and summarised in `work/view/log.md` (2026-09-15).

The open part is decided as two rows, both reading behaviourally, and
the reasoning is in the README clause beside the app-driver section.
