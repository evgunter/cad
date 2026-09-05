---
id: unindexed-refusal-is-an-outcome-not-a-read
kind: issue
title: The ruling's worked example puts unindexed_refusal on the badge channel and the ruling's own rule puts it on the line
status: open
opened: 2026-09-05
---



## What this is

`#1945` ruled the two axes: **a badge is a read of held state a reader
consults; a line message is the outcome of something that just
happened**, and either can carry a subject. It then worked the rule
through three facts of one class and said all three are badges —
naming `scene_refusal`, `index_refusal` / `unindexed_refusal` and
`projection_refusal`.

**The rule and that example disagree about `unindexed_refusal`, and
the unit built the rule.** Three of the four doors moved to the badge
family; `frame::unindexed_refusal` stayed on the line.

## Why the rule puts it on the line

`crates/viewer/src/pick.rs:2551` — `pick::unindexed(actions, indexing)`
answers `Some` for a `Select` and `None` for a `Hover` or a
`ClearHover`, and its own doc says why: *"A click is an act — the user
asked for something and did not get it — and that is exactly what the
line carries."* Half its input is **this frame's pick stream**, not
held state, so the sentence exists because the user clicked. That is
an outcome under the rule, whatever the sentence reports.

Three consequences, each checkable:

- A badge is drawn from held state alone, so it would be lit whenever
  the index is absent, clicked or not — a behaviour change nobody
  asked for.
- The seam state it reports is already read by two other channels:
  `frame::index_badge` (this unit) for a build the cache is holding a
  refusal for, and `frame::Progress::Indexing` for one under way —
  whose hover text at `crates/viewer/src/app.rs` is literally
  `NotIndexed::Building.to_string()`, this same sentence.
- `frame::Progress`'s own header forbids the result: *"expressing that
  as a second `if` beside the first would have given the toolbar two
  indicators that can both be lit, for one wait, with no rule anywhere
  saying which the reader should believe."*

## What is at stake

Nothing about the mechanism. `Badge` carries a subject either way, and
moving this one door is a two-line change: `unindexed_refusal` becomes
`unindexed_badge`, reading `indexing` and the cache instead of the
frame's actions, and the pane stops writing the line.

What is at stake is **which of the two the twenty-writer sweep sorts
on**. If the example governs, "reports seam state" is the test and
most of `status-line-writers-bypass-the-ranking`'s news list re-sorts
with it; if the rule governs, "reads held state" is the test and the
frame's own events are the discriminator. The unit took the rule
because the rule is what Ev ratified in as many words and because the
example is refuted by the code it names.

## For Ev

Confirm the rule over the example, or say the example was the ruling
and this door moves too.
