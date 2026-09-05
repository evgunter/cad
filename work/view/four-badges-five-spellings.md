---
id: four-badges-five-spellings
kind: issue
title: The toolbar's badge family is one concept in prose and five spellings in code
status: closed
opened: 2026-09-04
closed: 2026-09-05
refs: [camera-fold-clears-status-line]
---

## What this is

`crates/viewer/src/frame.rs`'s header names a family: the at-rest
verdict, the advisory checks, the δ the display budget chose, and the
product fault are all STANDING FACTS, badged rather than written to the
status line. The concept is real and the header is right about it. The
implementations agree on nothing else.

Taken from the toolbar in `crates/viewer/src/app.rs`, in draw order:

| Badge | `None` handled | Colour | "product: " / prefix | Ownership |
|---|---|---|---|---|
| at rest | folded into `Option<AtRestBadge>`, matched | `weak` when certified, `unresolved` when refused | composed at the site (`"at rest: {message}"`) | borrows, formats in place |
| checks | two conditions at the site (`Some(report)` and `!findings.is_empty()`) | `unresolved`, on a frameless `Button` | composed at the site | borrows, formats in place |
| δ chosen | two conditions at the site (`Some(fitted)` and `wording()`) | `weak` | composed at the site | borrows, formats in place |
| product | folded into a policy function (`frame::product_badge`) | `unresolved` | taken from `ProductError`'s `Display` | returns an owned `String` |

`crate::tree::RowStatus::badge()` is a fifth member and the crate's own
precedent for the shape the others do not use: a typed state that
renders its own badge label, with the payload kept separate.

## Why it matters

The differences are not cosmetic. Where the `None` decision lives
decides whether a row can assert it: `product_badge`'s carve-out for
the arms another channel carries is testable because it is a function,
and the checks badge's `!findings.is_empty()` rule is not, because it
is an `&&` inside a `ui` closure. Where the prefix comes from decides
whether the chrome writes prose about another value's failure — the
error micro-decision — and three of the four do. And the colour split
between `weak` and `unresolved` currently encodes "actionable or not",
which is a real rule (`pane/features.rs` argues it explicitly for
poisoned rows) that no value states.

## Not a sweep to run blind

The right shape is probably a small badge vocabulary: a typed value per
standing fact, its own rendering, its own `None`, and one draw at the
toolbar — with `RowStatus::badge()` as the model. But four call sites
in `app.rs` is CHROME-adjacent territory, and the at-rest and checks
badges each carry ratified arguments (the checks badge is a button
BECAUSE a tooltip is the wrong home for text a reader acts on) that a
uniformity pass must not flatten. Whoever takes this reads those first.

## Provenance

Found by the style review of `camera-fold-clears-status-line`, whose
own change added the fifth member. Filing rather than fixing was that
review's recommendation: asserting the family exists and implementing
one member differently is the finding.

## Put to Ev (VIEW orchestrator, 2026-09-04)

**On this PR because it is the second half of one decision, not
because it is hard on its own.** `status-line-writers-bypass-the-
ranking` sorts nineteen writers into *news* (which wants the news
vocabulary — the first question on this PR) and *standing facts*
(which want badges — this one). Four of the nineteen are standing
facts and each would add a fifth, sixth, seventh and eighth member to
a family that already implements four members four different ways.
Answering the two questions in two units means designing the badge
family twice.

**The question is narrow: is a badge vocabulary worth having?** The
model already exists in the crate — `tree::RowStatus::badge()`
(`crates/viewer/src/tree.rs:107`) is a typed state that renders its own
label with the payload kept separate. The four toolbar badges
(`crates/viewer/src/app.rs`, the block around `:1101-:1180`) agree
with it on nothing: where `None` is decided, where the prefix comes
from, whether the value is owned or formatted in place, and whether the
colour rule is stated anywhere.

The differences are not cosmetic, and this is the argument for
answering yes: **where the `None` decision lives decides whether a row
can assert it.** `frame::product_badge`'s carve-out is testable
because it is a function; the checks badge's `!findings.is_empty()`
rule is not, because it is an `&&` inside a `ui` closure.

**The constraint on any answer, stated so a uniformity pass cannot
flatten it:** the at-rest and checks badges each carry ratified
arguments — the checks badge is a *button* BECAUSE a tooltip is the
wrong home for text a reader acts on — and the `weak`/`unresolved`
colour split encodes "actionable or not", a real rule that
`pane/features.rs` argues explicitly for poisoned rows and that no
value states. A badge vocabulary has to keep all three or say why not.

**The orchestrator's recommendation:** yes, and take it *with* the
news vocabulary as one unit, because the sweep needs both.
## A sixth member, and it is a second SHAPE (#1886's style review, 2026-09-05)

Recorded as evidence on this item rather than filed again: the review
of `view/prune-report` found the family has two implementations *of
the notice itself*, not just five spellings of the badge.

`frame::supersession_notice` (`frame.rs:232`) and the new
`frame::dropped_hide_notice` (`:269`) are free functions returning
`Option<String>` that compose prose. `tools::ToolNotice`
(`tools.rs:181`) and `prefs::Notice` (`prefs.rs:81`) are **typed
values with `Display`**, extended into the same `notices` vector via
`.map(ToString::to_string)` (`app.rs:605`, `app.rs:525`). So the crate
holds both shapes, feeding one channel.

#1886 added the second member of the *function* shape — the minority
one, and the one `tree::RowStatus::badge()` and this item's own
argument both say is wrong. The two functions are additionally
near-identical: `frame.rs:232-243` and `:269-280` differ only in two
format literals.

**The reviewer named where else to look**, which this item should
carry: `frame::product_badge` and `frame::dialog_status` are the same
`fn(...) -> Option<String>` / `StatusUpdate` shape over other people's
typed values. That makes four members of the function shape, not two.

## RULED (Ev, #1883, 2026-09-05): yes, and it rides the news vocabulary

> "sure"

Answering *is a badge vocabulary worth having* — a typed value per
standing fact, its own rendering, its own `None`, one draw at the
toolbar, with `tree::RowStatus::badge()` as the model — and the
recommendation it was put with: **take it WITH the news vocabulary as
one unit**, because `status-line-writers-bypass-the-ranking` sorts its
nineteen writers into news and standing facts and needs both
vocabularies to sort them into.

The constraints stated when the question was asked are part of what was
ratified and a uniformity pass must not flatten them: the checks badge
is a **button** because a tooltip is the wrong home for text a reader
acts on, and the `weak`/`unresolved` split encodes "actionable or not",
which `pane/features.rs` argues explicitly for poisoned rows and no
value states.

The 2026-09-05 section above is part of the job: the family has two
implementations *of the notice itself* as well as five spellings of the
badge, and four members of the function shape rather than two.

## Closed by `view/news-and-badges` (PR #1933)

`frame::Badge` — label, `Tone`, detail, `Affordance` — with
`at_rest_badge`, `checks_badge`, `product_badge` and `delta_badge` as
the four members and `app::draw_badge` as the one draw. Every member's
`None` is a row, including the `!findings.is_empty()` rule that lived
in a `ui` closure. Both ratified constraints survive: the checks badge
is `Affordance::Opens` with the argument on the variant, and
`Tone::{Advisory, Actionable}` states the actionable-or-not split.
`Withdrawal` replaces the two free notice functions with one typed
value carrying `Display`.

**Residue with a file**: `tone-is-a-value-in-frame-and-a-comment-in-two-panes`
— the split is a value at the toolbar and still hand-picked at
`pane/features.rs` and `pane/create.rs`.
