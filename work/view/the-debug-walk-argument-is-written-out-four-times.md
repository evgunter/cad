---
id: the-debug-walk-argument-is-written-out-four-times
kind: issue
title: the exhaustive-destructure argument is restated in the README, in three doc comments, in the item and in the log with no single home
status: open
opened: 2026-09-06
refs: [2093]
---



Found by the style review of #2093.

One argument — *the walk destructures exhaustively, a `_` arm is a
visible decision, and `finish_non_exhaustive` names exactly those arms*
— is written out five times in the change that introduces it:

- `crates/viewer/README.md:431-447`, a 17-line paragraph;
- `crates/viewer/src/session.rs:302-313` (`Derived`, 12 lines);
- `crates/viewer/src/session.rs:421-428` (`LandedRun`, 8 lines);
- `crates/viewer/src/session.rs:1941-1951` (`DocSession`, 11 lines);
- `crates/viewer/src/pickcache.rs:163-169` (`PickCache`, 7 lines).

Plus the item's `## Closed` section and `work/view/log.md`'s entry,
which are records and not code-adjacent, so they are not the problem.

The four in-code copies are not four different statements: three of
them open by pointing at another (*"as [`Derived`]'s is"*, *"as
[`Derived`]'s is: a field added to..."*, *"Exhaustive by
destructuring:"*) and then restate the rule anyway. `Derived`'s copy
is the general statement of the mechanism and is attached to the
smallest of the four values, for no reason a reader can see; `README`'s
copy is a fifth phrasing of the same thing rather than the home the
others defer to. Comment-style discipline asks for the invariant at the
site; what is at each site is the invariant *plus* the argument for it.

## The two "three"s

A reader of `session.rs` meets two different groupings of
`DocSession`'s fields, both counted to three, 1,700 lines apart:

- `session.rs:222-252` — *"Three neighbours are outside it"* —
  `display`, `gesture`, and `path`+`resolver` (three bullets, four
  fields), grouped by what `Open`/`NewDocument` do to them;
- `session.rs:1947-1951` — *"five `_` arms ... and they are five for
  three reasons"* — `eval`, `requested_doc`, and
  `tol`+`display`+`resolver`, grouped by what the dump does with them.

`display` and `resolver` are in both trios and `path` is in the first
and rendered by the walk; `tol` is in neither struct-level grouping.
Nothing is wrong — they answer different questions — but the same
count-word over the same struct twice is the shape that makes a reader
believe they have already read this.
