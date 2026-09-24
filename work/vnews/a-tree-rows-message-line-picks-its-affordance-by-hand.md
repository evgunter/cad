---
id: a-tree-rows-message-line-picks-its-affordance-by-hand
kind: issue
title: The feature row's message line re-spells frame::Affordance as a link-or-weak choice
status: open
opened: 2026-09-20
priority: P1
cost: E
---



Found by the review of `tone-is-a-value-in-frame-and-a-comment-in-two-
panes`, fifteen lines below the badge that row repaired and inside the
same function — `crates/viewer/src/pane/features.rs`, `feature_row`'s
message line (~`:97-115`).

**The same defect one axis over.** That unit moved the LOUDNESS rule
onto the value (`RowStatus::tone()`, read as a `frame::Tone`). The line
under the row still picks its AFFORDANCE at the call:

- `RowStatus::Poisoned { through, .. }` draws `ui.link(message)` and
  pushes a `Select` on click;
- every other status draws `ui.weak(message)`.

`frame::Affordance::{Read, Opens}` is the ratified value for exactly
that question — *what a reader can do with this beyond reading it* —
and `app::draw_badge` reads it **from the badge** rather than deciding
per widget. So the feature row now reads one `frame` value from the
status and re-spells a second one beside it.

## Why it was not taken with the tone

It is not the same fix, and saying so is the row's content. The tone
match produced **nothing but the classification**, so moving it onto
`RowStatus` deleted the match outright. This match also produces the
click TARGET — the `through` id the `Select` op needs — which the pane
must obtain from the status whatever the affordance says. So the
honest shapes are:

1. `RowStatus` grows `fn jump(&self) -> Option<RecipeNodeId>`, the
   affordance follows from `Option::is_some`, and the pane matches
   once; or
2. the current match stands, because a site that must destructure for
   the payload is not re-deriving anything when it also learns the
   affordance from the same arm.

**(2) is a real defence and is why this is a row rather than a fix.**
It is also the argument that would NOT have saved the tone match, which
destructured nothing. Deciding it decides the general form: does a
value owe an affordance when the caller needs its payload anyway.
