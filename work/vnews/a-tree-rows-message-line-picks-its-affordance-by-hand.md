---
id: a-tree-rows-message-line-picks-its-affordance-by-hand
kind: issue
title: The feature row's message line re-spells frame::Affordance as a link-or-weak choice
status: closed
opened: 2026-09-20
branch: vnews/salience-read-from-the-value
pr: 3230
closed: 2026-09-25
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

`frame::Affordance::{Read, Opens}` is this program's own value for
exactly that question — *what a reader can do with this beyond reading
it* — and `app::draw_badge` reads it **from the badge** rather than
deciding per widget. So the feature row now reads one `frame` value
from the status and re-spells a second one beside it. (The value is
documented in `crates/viewer/README.md`, the implementation record,
and ratified nowhere: the census in
`ratified-is-asserted-across-viewer-src-and-some-was-never-ratified`.)

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

## Decided (2026-09-25): shape (1), and the general rule it sets

**The tree as found.** The line had moved into
`pane::features::failure_lines`, a free function, and the match there
produced `(words_to, then_to)`: the affordance already followed from an
`Option` (`words_to.is_some()` picked `widgets::message_link` over
`widgets::message_toned`), but *a poisoned row's line points at
`through`* was still stated by the pane, a second time after
`RowStatus::Poisoned::through`'s own doc, and the `Failed`-only gate on
`TreeRow::repair_at` restated `tree::rows`, which already sets it on
`Failed` rows only.

**The fix.** `RowStatus::jump() -> Option<RecipeNodeId>`, exhaustive
over the status. `failure_lines` reads `message()`, `jump()` and
`repair_at`, and no longer matches on the status at all.

**Why (2)'s defence does not hold here.** It holds for a payload the
affordance does not depend on. Here the payload the pane needs IS the
affordance: a line with a target is a link, one without is read, and
one `Option<RecipeNodeId>` answers both. So there is no second value
to owe, and no `frame::Affordance` beside it — only the pane's copy of
the mapping, which the move deletes.

**The general rule, then:** a value owes the *classification* a
surface draws from (a tone, an affordance) wherever a caller would
otherwise produce it per arm. A caller that destructures for payload
still may, provided what it draws from is read off the value rather
than implied by which arm it stands in; and where the classification
is the *presence* of a payload, the value owes the payload as an
`Option` and nothing more. `resolution-and-standing-pick-their-tone-
by-hand` is the same rule's other half: `pane::properties::
entity_verdict` still composes its words per arm, and reads its tone
once off `Standing::tone`.

**The one tone left at the site says why.** The words under a failed
row stay `Tone::Advisory` although the status is `Actionable`: the
row's loudness is its badge's, and a failed row drawn loud twice would
be the one loud row twice.

**Receipts.** `a_poisoned_rows_pointer_selects_the_row_it_names` is
`jump`'s guard: planting `Self::Poisoned { .. } => None` in `jump`
reds it. `a_failed_rows_words_are_weak_under_its_loud_badge` reads the
paint's colour (`pane::headless::Landed::ink`) against egui's own weak
colour: passing `row.status.tone()` at the site reds it.
