---
id: seat-line-spells-the-list-mark-as-a-literal
kind: issue
title: seat_line joins its items with a bare "; " rather than frame::LIST_SEPARATOR
status: review
opened: 2026-09-15
refs: [withdrawal-causes-join-on-a-mark-a-fault-may-contain]
branch: vnews/seat-line-list-separator
---



Found by the same sweep as
`startup-notices-join-on-a-mark-a-prefs-notice-contains`, and much
smaller.

## The site

`seats::seat_line` (`crates/viewer/src/seats.rs`) renders one item per
seat and joins them with a bare `"; "`:

    .collect::<Vec<_>>()
    .join("; ")

That string is `frame::LIST_SEPARATOR`'s value, and what `seat_line`
produces is exactly what that constant is for — the items of a list
ONE notice carries (`crates/viewer/README.md`, "The line is composed
at two levels and they are two marks"). The line reaches the chrome as
a lost-pick notice's text.

**Not ambiguous today**, and for a stronger reason than most: both
halves of each item are built three lines above the join, from
`Seat::name()` (a vocabulary word) and a `RecipeNodeId`'s `u64`, and
neither can carry a `"; "`. What is wrong is only that the mark is
spelled twice — a change to `LIST_SEPARATOR` moves the withdrawal
join and the preferences join and leaves this one behind, which is
the two-copies-drift argument `seat_line`'s own doc comment makes
about the seat vocabulary one paragraph up.

## Fix

Use `frame::LIST_SEPARATOR`. The only question is the direction of the
dependency — whether `seats` should reach into `frame` for it, or the
constant belongs somewhere both can see — which is why this is a row
rather than a one-line edit made in passing.

## Home

VIEW's: `crates/viewer/src/seats.rs`.

## Fixed (`vnews/seat-line-list-separator`)

`seat_line` joins on `frame::LIST_SEPARATOR`, imported into `seats`.

**The dependency direction: `seats` reaches into `frame`.** Not a
shared third home, for three reasons. `frame` is where the crate's
news vocabulary is DEFINED, and `LIST_SEPARATOR` is defined in terms
of the other two constants beside it — it is *"a level in from
`NOTICE_SEPARATOR`"*, and `Message::new` rewrites a `NOTICE_MARK` into
it. A third module holding one of those three splits a definition that
is only meaningful whole. Second, the edge is acyclic and new in one
direction only: `frame` names nothing in `seats` today, so `seats` ->
`frame` adds no cycle and closes no door. Third, both modules declare
**Module kind: vocabulary** (`crates/viewer/README.md`, Module
boundaries), and a vocabulary naming a vocabulary is what that rule
permits; the rule's subject is a vocabulary naming a DRIVER.

### What this item claimed, checked

- `seat_line` joins with a bare `"; "` and that is `LIST_SEPARATOR`'s
  value: **true**, and it was the only `"; "` literal in
  `crates/viewer/src/` other than the constant's own definition.
- Neither half of an item can carry the mark: **true**, and stronger
  than the item says — `Seat::name()` is a `vocabulary!` word and
  `node.0` is a `u64`, so neither half is user text at all.
- *"The line reaches the chrome as a lost-pick notice's text"*:
  **false**. `seat_line`'s only production caller is `pane::create`,
  which renders it as a tool panel's `ui.weak` label; the lost-pick
  notice is `SeatEvent::PickLost`'s own `Display`, a separate sentence
  built from the same vocabulary (`Seat::name()`), which is what
  `seat_line`'s doc comment says. The item over-read the doc comment.
  Nothing about the fix depends on it.
- `withdrawal-causes-join-on-a-mark-a-fault-may-contain` and
  `startup-notices-join-on-a-mark-a-prefs-notice-contains` as live
  siblings: **both closed**, at #2693 and #2710. The second's fix
  moved `startup_notices` OFF `LIST_SEPARATOR` entirely, so the
  constant's live consumers after this change are `Display for
  Withdrawal` and this line.

### Sweep residue

`mate-panel-hand-rolls-the-seat-line` — the only other site in
`crates/viewer/src/` that spells the mark, filed rather than swept in
because the answer there is not a substitution.
