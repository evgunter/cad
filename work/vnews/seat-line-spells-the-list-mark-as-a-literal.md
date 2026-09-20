---
id: seat-line-spells-the-list-mark-as-a-literal
kind: issue
title: seat_line joins its items with a bare "; " rather than frame::LIST_SEPARATOR
status: spec
opened: 2026-09-15
refs: [withdrawal-causes-join-on-a-mark-a-fault-may-contain]
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
