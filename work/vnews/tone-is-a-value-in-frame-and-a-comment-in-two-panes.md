---
id: tone-is-a-value-in-frame-and-a-comment-in-two-panes
kind: issue
title: The actionable-or-not rule is a value at the toolbar and a comment in two panes
status: closed
opened: 2026-09-05
branch: vnews/tone-row-badge
pr: 2915
closed: 2026-09-20
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
- `crates/viewer/src/pane/create.rs:582-586` — a third copy.

**Both citations above are as-of 2026-09-05 and neither names its
subject today** — the first because this row's lane removed it, the
second because the subject moved files before the lane ran. They stand
as the reading that filed the row; the section at the end says where
each subject is now.

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
`create.rs:582`'s copy is a separate read and wants looking at on its
own terms — it may not be the same rule.

## What the lane did, and what the third citation turned out to be
(2026-09-19)

**Taken as the row proposes.** `tree::RowStatus::tone()` returns
`frame::Tone` and `pane::features` reads it
(`crates/viewer/src/tree.rs`, `RowStatus::tone`, `:149`;
`crates/viewer/src/pane/features.rs`, `feature_row`'s badge block,
`:76-93`). The direction is `tree -> frame` because the rule is already
`tree`'s by its own module header — *"which row a failure sends the eye
to"* — and was merely spelled at the pane; `frame` names `tree` only in
doc links, so nothing is inverted and no cycle is made. Both modules
are vocabularies, so the module-kinds gate's *a vocabulary may name a
vocabulary* holds.

**`Ok` is `Advisory`, not absent.** `tone()` is total where `message()`
is not: an `Ok` row has a badge (`"ok"`), which
`crates/viewer/examples/r1_e2e.rs` prints. Whether a surface DRAWS a
healthy row's badge is the surface's decision, and the Features pane
keeps its own silence arm.

**The third citation was stale and its subject is one file over.**
`pane/create.rs:582-586` is the `ShapeKind::Path` notation block today.
At this row's filing sha (`8cceb9b6ce`) those lines were the
`PreviewError` weak-or-coloured arm; `49897008d` (2026-09-19) moved it
into `pane::profile::preview_verdict`, which no successor program
claims. **It is a real third copy and a genuinely different rule** —
unfinished-versus-blames-a-step, not own-refusal-versus-someone-else's
— and it is filed as
`work/issues/preview-error-picks-its-tone-by-hand-in-a-comment`.

**Residue**, each filed rather than disclosed here:
`work/vnews/tone-doc-argues-from-a-site-that-now-reads-the-value`
(`frame.rs` is fenced off this lane),
`work/vnews/tone-to-chrome-mapping-is-spelled-twice` (the paint, which
reaches `app.rs`), and
`work/vnews/resolution-and-standing-pick-their-tone-by-hand`.

**On closing this row**: `work/chrome/chrome-weight-is-outside-the-palette`
is `parked` on it and on nothing else, so the PR that closes this one
reds `work.py lint` until that row is re-parked, opened or deferred in
the same commit.

## What the fix pass changed, and the title's own count (2026-09-20)

**The pane's draw is an exhaustive `match` again.** The first cut
spelled the silence decision `if !matches!(row.status, RowStatus::Ok)`,
which bought `tone()`'s totality at the price of the property this
module states twice — *"one arm per variant so a new node type cannot
fall into a wildcard"*, *"Exhaustive on purpose"* — and the header
anticipates the very variant that would have walked through it (*"a
status saying 'the run, not this row'"*,
`work/chrome/band-refusal-still-badges-every-row`). `tone()` stays
total; the call site is a `match` with `Ok` drawing nothing as its own
arm. Receipt: a planted `RowStatus::RunFault` reds `pane/features.rs`
along with `badge()`, `tone()` and `message()` — four sites, where the
`matches!` spelling would have compiled and drawn.

**The tone-to-chrome mapping has one home**, `app::toned`, read by the
toolbar badge family and by this row — see
`tone-to-chrome-mapping-is-spelled-twice`, closed with this row rather
than left as residue.

**The title undercounts.** *"a comment in two panes"* was the reading
at filing; the prose census the fix pass ran found the rule attributed
to a SITE in **four** further homes — two in `frame.rs`, one in
`theme.rs`, one in `crates/viewer/README.md` — none of which any grep
for the tone CALL could reach. The population and its sweep rule are
`tone-doc-argues-from-a-site-that-now-reads-the-value` and VDOC's
`viewer-readme-attributes-the-tone-rule-to-the-pane`. A second
`frame` value re-spelled inside the repaired function is
`a-tree-rows-message-line-picks-its-affordance-by-hand`.
