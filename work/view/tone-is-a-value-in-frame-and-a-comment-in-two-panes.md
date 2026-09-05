---
id: tone-is-a-value-in-frame-and-a-comment-in-two-panes
kind: issue
title: The actionable-or-not rule is a value at the toolbar and a comment in two panes
status: open
opened: 2026-09-05
---


## What this is

`view/news-and-badges` made the actionable-or-not colour rule a value —
`frame::Tone`, with `Advisory` for a report and `Actionable` for a
verdict a reader may need to act on — and removed the four hand-picked
spellings at the toolbar. **`Tone`'s own doc argues from a site it did
not reach.**

- `crates/viewer/src/pane/features.rs:75-92` — the row badge still
  hand-picks `ui.weak` for `Unevaluated`/`Poisoned` and
  `ui.colored_label(chrome(self.theme.unresolved), …)` for `Failed`,
  with the rule stated in a comment. This is the site `Tone`'s doc
  cites as the rule's origin.
- `crates/viewer/src/pane/create.rs:592-594` — a third copy.

So four spellings became one at the toolbar and three remain elsewhere,
and the value that claims to state the rule is not what either of those
sites reads.

## Why it was not taken with the badge vocabulary

Ev's ruling was about the TOOLBAR badge family — "a typed value per
standing fact, its own rendering, its own `None`, one draw at the
toolbar" — and `tree::RowStatus::badge()` was named as the *model* for
that family, not as a member to convert. Reaching into the Features
pane's row drawing would have been a second uniformity pass inside a
diff already touching four badges and twelve writers.

## The shape of an answer

`tree::RowStatus` grows a `tone()` beside its existing `badge()`, and
`pane::features` reads it instead of matching the status a second time.
`create.rs:592`'s copy is a separate read and wants looking at on its
own terms — it may not be the same rule.
