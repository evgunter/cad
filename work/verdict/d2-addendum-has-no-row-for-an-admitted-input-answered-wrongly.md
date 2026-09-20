---
id: d2-addendum-has-no-row-for-an-admitted-input-answered-wrongly
kind: issue
title: design: the D2 addendum has no row for an input that is ADMITTED and answered wrongly, which is what issue 1598 was for three months
status: open
opened: 2026-09-16
priority: P1
cost: D
---


Filed by the PROPS sphere-pole-side fix pass (PR 2741). **Design text,
so Ev's call**: the D2 addendum is ratified, and this row states the
gap rather than editing it.

## The gap

The addendum's rows classify inputs by how the kernel DECLINES them —
row 1, refused by cause; row 2, valid input with the lane not built;
rows 4/5, the `unreachable` classes. There is no row for an input that
is **admitted and answered wrongly**, and that is a distinct and worse
state than any of them: a refusal is visible at the door, a wrong
number is not visible anywhere.

Two live instances, both from this unit's ground:

- **Issue 1598**, for three months: a closed sphere split into two
  faces by the same two edges passed every premise and `mass_properties`
  answered `Ok { volume: 0.0 }` with `pad = 0`. Nothing in the
  addendum's vocabulary described that state while it lasted.
- **The rim-only fold's own first landing**, caught by this PR's dual
  review before merge: with the pole folded but the rim's CLOSURE never
  decided, a lone half rim answered half a cap's area through the
  public `curved_face` door. Four shapes that had been typed refusals
  became definite wrong numbers.

The second is the sharper argument for the row: a unit whose PR body
NAMED the missing class re-created an instance of it in the same diff.

## What the post-fix states are

Both are now typed refusals, and the classification of each is **row
2** — valid input, lane not built — which is what
`require_rims_at_extremes`' own doc already files `NotIsoRectangle`
under. PR 2741's body originally said issue 1598 "moves into row 1's
shape"; that is wrong and is corrected there.

## What the row would say

Reachable by input, valid, admitted, and ANSWERED — with an answer the
kernel cannot stand behind. Its recourse column is empty by
construction (there is nothing to fall back to; the wrong answer is
already out), so what the row buys is not a recourse but a NAME for the
state a reviewer is looking for, and the obligation that finding one is
a stop-the-line event rather than an issue to schedule.
