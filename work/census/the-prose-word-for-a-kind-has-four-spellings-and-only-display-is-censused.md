---
id: the-prose-word-for-a-kind-has-four-spellings-and-only-display-is-censused
kind: issue
title: the prose word for a kind has four spellings and prose_census can only see one of them
status: open
opened: 2026-09-06
refs: [2053]
---


Found by the style review of PR 2053 (`view/refusal-all`). Filed here
rather than on a program's slate because the owner is genuinely
undecided: the census is LIB's (`crates/pncad-py/*`), the rule's home
is DOCM's (`crates/editor-core`), and the instances are spread over
`viewer`, `editor-core`, `geom-brep`, `sweep`, `topo`, `profile` and
`geom-core`.

## The four spellings

The rule — *a kind reaches a user as the common noun a person would
say, never as its variant identifier* — has one written home,
`Dimension`'s `Display` (`crates/editor-core/src/expr.rs:46-64`). The
tree implements it four ways:

1. **`impl Display`** — `Dimension`. The only one `prose_census` reads.
2. **an inherent `fn label()` / `fn name()`** — 23 of them, e.g.
   `SlotId::label` (`crates/editor-core/src/node.rs:183`),
   `NodeKindWanted::name` (`crates/viewer/src/session/refuse.rs:85`),
   `SlotFamily::label` (`crates/editor-core/src/node.rs:433`),
   `Tool::label` (`crates/viewer/src/tools.rs:85`),
   `MateKind::name` (`crates/editor-core/src/mate.rs:91`).
3. **a free function** — `tip_state_words`
   (`crates/viewer/src/sketch.rs:826`), over `TipState`.
4. **a hand-written table at the render site** — the add-parameter
   radio row's `(Dimension::Length, "Length")` array
   (`crates/viewer/src/pane/properties.rs:158-161`), which
   `work/view/dimension-radio-row-is-an-unforced-mirror-of-a-kernel-enum.md`
   carries.

## Why this matters to the census

`prose_census` (`crates/pncad-py/src/prose_census.rs:963-969`) scans
`impl … Display for X { … }` bodies. Spellings 2, 3 and 4 are outside
it by construction, and so is a `Display` that DELEGATES to one of
them — which is how PR 2053's defect survived both instruments.

PR 2053 measures that gap as *"12 `{x:?}` sites tree-wide in inherent
impls of `Display`-carrying types"* (verified: 12 at its merge base, 11
after its fix). That figure counts only types that ALSO have a
`Display`. The larger set is types whose prose lives ONLY in an
inherent accessor and that have no `Display` at all — every one of the
23 above — where a `{x:?}` next to the accessor is a leak the census
can never see and no count exists for.

A live instance is `EditError`'s six `{slot:?}` renderings
(`crates/editor-core/src/edit.rs:860-931`), disclosed in that module's
own header at `edit.rs:798-802` — *"What still renders through `Debug`
here is the SLOT id ({slot:?}), which has a prose spelling
(`SlotId::label`) it does not use … it is filed rather than taken
here"* — and the file it says it is filed as is not named. `SlotId` is
`Display`-less, so the census's verdict is `Prose` and will stay
`Prose` however the scan set is widened; only a "the site bypassed a
prose renderer that exists" verdict reaches it.

## What this is not

Not a proposal that every kind get a `Display`. Naming the four
spellings is the finding; which one is the home, and whether the census
can be taught to see past a delegation, are the two decisions.

## Adjacent rows

`work/fix/error-types-with-no-display-class.md` (closed, #1111) is the
same rule for ERROR types with no `Display`; its residue is
`work/fix/verb-and-dimension-render-through-debug.md` (open), which
holds two instances. Neither asks the question this row asks, which is
about the SPELLING of a kind's prose rather than about a missing
`Display` on an error, and neither names the census's structural
blindness to spellings 2-4. Whoever routes this should decide whether
it merges into the FIX row or stands apart.

## Two facts from PR 2347 (2026-09-11, placed by the FIX orchestrator)

**1. `profile::path::Verb` joined spelling 1, and took a form this row
should note.** Its `Display` is declared **on the macro row** that
declares the variant — `verb LineTo(Target<T>) = "line_to" …` — so the
word cannot drift from the variant it names, and deleting the row takes
the word with it. That is strictly stronger than an `impl Display`
written beside the enum, and it is the repo's own spelling:
`crates/viewer/src/vocab.rs`'s `vocabulary!` spends `=` on exactly
this. Where a vocabulary is macro-generated, this is the shape that
makes spelling 1 unforgeable rather than merely conventional.

**2. A fifth instance of spelling 2 that this row does not list.**
`tip_state_words` (`crates/viewer/src/sketch.rs:826`) is a
viewer-local prose vocabulary for a kernel enum, silent under a rename
— and unlike `PathVerb::label` it is **not** macro-generated, so there
is no table to hang a word on and no exhaustiveness forcing it. It is
the same shape as the 23 `label()`/`name()` sites the row counts, in
the position where the fix is least obvious.

Also worth recording against this row's census half: `prose_census.rs`
was the wrong instrument for PR 2347's defect and could not have found
it. The census judges **brace-shapedness** — the struct-dump class — so
a fieldless enum with no `Display` reads as `Prose` by its verdict; it
does not expand `macro_rules!` bodies; and none of the `Dimension`
sites sat inside a `Display` impl at all. The row's title is right that
only spelling 1 is censused, and the sharper statement is that the
census cannot see this defect **even for spelling 1's own types**,
because brace-shapedness is not the property at issue.

## Re-homed to CENSUS (2026-09-11, the cut in `docs/WORK-TRACKS-2026-09.md` addendum 3)

CENSUS collects the rows of one class: a vocabulary spelled by hand in
several places, and the census or instrument that cannot see one of the
spellings. This row is a member of that class.

Its class at the cut was **H** — two decisions owed, owner undecided,
23+ sites over seven crates and three programs. The class is a dispatch
estimate made by reading the row against the tree on 2026-09-11, not a
verdict on the finding, and a lane that finds it wrong says so in its
PR. The id, the `track:` letter where the row carries one, and the body
above are unchanged by the move.
