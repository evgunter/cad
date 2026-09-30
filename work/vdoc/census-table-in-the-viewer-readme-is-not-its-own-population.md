---
id: census-table-in-the-viewer-readme-is-not-its-own-population
kind: issue
title: The README's non-dump census table says nine and PruneReport::is_empty is a tenth it does not carry
status: closed
opened: 2026-09-11
priority: P4
cost: E
closed: 2026-09-21
branch: vdoc/readme-counts
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

## Closed — the population re-derived, nine rows to thirteen (#vdoc/readme-counts)

**Not one row added: the population re-derived**, which is what this
row asked for. `PruneReport::is_empty` is a member and so are three
more the table never carried.

**The enumeration rule, now written above the table**, and the printed
command IS the answer rather than something to read an answer off:

    rg -n --no-heading 'let\s+&?([a-z_]\w*::)*[A-Z]\w*\s*\{' crates/viewer/src | wc -l

**24** at `f45df59dc5`, run exactly as printed. No hit is in a comment.

This is the second rule this row has been given and the first one did
not reproduce. The version that first landed on this branch printed a
`-U -o` command and told the reader to *"read for the lines that begin
`let`"* — without `-n`, `--no-heading` prefixes every match with
`path:`, so nothing in that output begins with `let` and a reader
following the page literally counts **0** against a stated 22. That is
this row's own class re-minted inside its repair, and the tell is the
shape of the sentence: a rule written as a DESCRIPTION OF THE OUTPUT
rather than as the command that produces the number. Every command this
branch prints is now run verbatim out of the file and quoted with its
exit code.

**Widening it to qualified paths moved the population from 22 to 24.**
`let egui::Modifiers { … }` and `let egui::Vec2 { … }`
(`pane::viewport`'s `viewer_modifiers` and `scroll_event`) were
invisible to a pattern that demanded `[A-Z]` immediately after `let`,
and both carry a completeness argument of their own. They are a stated
bucket and not rows: the declaration they are held to is the toolkit's,
so a field arriving there is a version bump's news rather than one of
this crate's values falling behind its own account.

This row's own suggested regex (`'let (&)?Self \{|\} = (self|report);'`)
is a third reading again: it reports *24 hits* for a different reason
— it counts the opening AND the closing line of eight multi-line
patterns and cannot see a bind over any named type but `Self` — so its
agreement with the number above is a coincidence of two errors.

**The reading half is stated with every member named**, so the sort can
be argued with: four are the `Debug` dumps (`Derived`, `LandedRun`,
`DocSession`, `PickCache`); four bind a parameter or a returned
vocabulary struct and claim nothing about completeness
(`widgets::drag_gesture_ops`, `widgets::value_field_ops`, the two
`ProbeOps` unpacks); two destructure a toolkit type at the chrome
boundary (`viewer_modifiers`, `scroll_event`); the remaining fourteen
binds are **thirteen censuses** — `PartialEq for Camera` spends two,
the second over `Point3`.

**The four that were missing**, each carrying the census argument in its
own doc comment already:

- `PruneReport::is_empty` (`display.rs`) — this row's tenth.
- `PickCache::forget` (`pickcache.rs`) — *"Exhaustive by destructuring,
  like the walk above"*. The README described it in prose immediately
  above the table and left it out of the table.
- `Display for Unusable` and `Unusable::refusal` (`prefs.rs`) — the
  single-field pair this row flagged as the arguable judgement. Both
  are members: the type's doc makes the census argument for both in as
  many words (*"a second field breaks BOTH renderings rather than
  one"*), which is the correctness argument the table's own definition
  asks for.

**Two counts in the same paragraphs moved with it**, because a citation
fix is class-wide over the file or it makes the file worse:

- *"Two of the eight name a field the walk deliberately does not
  spend"* — **three of the thirteen**. It said *eight* over a table
  that already had nine rows, and the third member is
  `PickCache::forget`'s `seam: _`.
- *"The five `Display`s above are the struct half of a population of 36
  `Display` impls under `src/`; the other 31 are over enums"* — **six**,
  **41** and **35** (`rg 'impl.*fmt::Display for ' crates/viewer/src | wc -l`
  prints 41 at `f45df59dc5`, no hit in a comment; six of the 41
  subjects are structs, the rest enums). The sweep's own finding was re-run and is unchanged: no
  catch-all over a subject enum, no tuple-arity drop, exactly two `..`
  (`CameraOp::Frame` drops `bounds`, `MateToolEvent::PickLost` drops
  `resolution`).
- The `Debug` walk sweep above it said *"every `.field(…)` call — 22"*;
  it is **21** (5 + 6 + 6 + 4 across the four walks). *Nine calls,
  seven fields* re-derives exactly.
- The `PickCache::forget` paragraph said it *"clears the four fields
  that describe a picture and must not miss a fifth, since a missed
  `attempted`…"*. `PickCache` has **four** fields and `forget` clears
  **three** of them (`index`, `attempt`, `error`); `seam` is the `_`
  arm, and no field is called `attempted`. Both corrected.

**What the rule cannot see**, stated at the sentence rather than left: a
census destructuring in a `match` arm or a parameter pattern instead of
a `let`, and one over a value reached through an accessor. Neither
exists under `src/` today.
