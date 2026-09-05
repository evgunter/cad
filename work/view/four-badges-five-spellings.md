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
