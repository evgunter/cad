---
id: census-table-in-the-viewer-readme-is-not-its-own-population
kind: issue
title: The README's non-dump census table says nine and PruneReport::is_empty is a tenth it does not carry
status: open
opened: 2026-09-11
---


Found by `view/gesture-doors` while adding a row to the table for its
own new census (`frame::Withdrawal::all`).

## The claim and what contradicts it

`crates/viewer/README.md` introduces the table with *"Every census in
this crate destructures the value instead of listing its fields by
hand … Nine of these are not dumps:"* and then lists nine rows. The
population it certifies is *a destructuring census under
`crates/viewer/src` that is not a `Debug` dump*, and the table is not
that population.

`PruneReport::is_empty` (`crates/viewer/src/display.rs:602-609`)
destructures `PruneReport` — its own doc says so, in the table's own
words: *"**Destructured rather than field-read**, so a fourth kind of
withdrawal is E0027 here rather than a withdrawal that leaves the
revision where it was"*. It is not a dump and it is not a row.

It is not a pre-existing miss of the sweep that built the table
(#2103): the destructure was written by #2348 (`961b2a64ae`,
`view/silent-withdrawals`), four days after the table (`2a20fb74fc`).
The same PR added a second one, `OpOutcome::from_prune`, which
`view/gesture-doors` has since deleted along with the copy it guarded.
So the shape is a diff adding a member of a tabulated class and not
tabulating it — the census-owes-a-tracker-pass rule one level in, for
a roster that lives in prose.

## What resolving it looks like

Re-derive the population rather than adding one row: the sweep is
every destructuring bind under `crates/viewer/src` read against
whether its correctness argument is *this list IS the value's fields*,
and `rg -n 'let (&)?Self \{|\} = (self|report);'` returns 24 binds
today, of which the table carries nine and four more are the `Debug`
dumps the sentence excludes. The rest are unclassified here
deliberately — several are single-field `Display` impls where the
census reading is arguable, and that judgement is the work.

## Home

VIEW's: `crates/viewer/README.md`, `crates/viewer/src/display.rs`.
