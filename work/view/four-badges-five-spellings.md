---
id: four-badges-five-spellings
kind: issue
title: The toolbar's badge family is one concept in prose and five spellings in code
status: open
opened: 2026-09-04
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
