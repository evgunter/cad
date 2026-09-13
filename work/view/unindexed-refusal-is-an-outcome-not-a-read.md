---
id: unindexed-refusal-is-an-outcome-not-a-read
kind: issue
title: The ruling's worked example puts unindexed_refusal on the badge channel and the ruling's own rule puts it on the line
status: closed
pr: 1957
closed: 2026-09-06
refs: [1945, 1957, news-and-standing-facts-are-orthogonal-axes, status-line-writers-bypass-the-ranking]
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

## RULED (Ev, 2026-09-06): the rule governs, not the example

> "your recommendation seems fine here"

`unindexed_refusal` **stays on the line**. Recorded here rather than
only as an answer to this door, because **this is the sweep's sorting
rule**.

### The two candidate tests, and why provenance wins

They differ on *whose event decides*:

- the **rule** asks **what caused this sentence to exist** → an act;
- the **example** asks **what this sentence is about** → a seam.

Every refusal is about something and caused by something, so both are
coherent axes and neither is refuted by a definition. **Provenance
wins because it is the only one a user can see.** It decides whether
the sentence exists when nobody acted: `crates/viewer/src/pick.rs`'s
`unindexed` gates on `any(|a| matches!(a, PickAction::Select(_)))` and
`.then_some(...)`, so today the sentence exists only if the frame
carried a click, while as a badge it would be lit for the whole
2–13 second index window regardless. That is the observable
difference, and there is no other.

### Three consequences, recorded with it

1. **It would put one sentence in two places.**
   `crates/viewer/src/app.rs` already hangs
   `NotIndexed::Building.to_string()` on the spinner as its hover text,
   and `frame::Progress`'s header forbids two indicators lit for one
   wait with no rule saying which a reader should believe.
2. **It would undo half of #1843.** That ruling's deliverable was the
   indicator **and** the pick path distinguishing "not indexed yet"
   from "nothing under the cursor" — the indicator is held state (a
   badge), the refusal is the answer to a click (the line). Merging
   them lands back on a spinner over inert picks, which is the
   fail-quiet #1843 refused.
3. **The example's test would cost frame state and the rule's does
   not.** "Held" is author-chosen — #1957 CREATED `scene_fault` and
   `projection_fault` so those facts would qualify — so badging seam
   refusals means minting a held field per refusal, and each is a new
   entry in `ViewerApp`'s frame-state inventory. That inventory is what
   GQ6's toolkit decision rests on and what #1843 was careful not to
   grow.

### What it settles beyond this door

The sort test for `status-line-writers-bypass-the-ranking` is
provenance, not subject matter. Noted on that item; re-sorting its list
is that unit's work and not this one's.
