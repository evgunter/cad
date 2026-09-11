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
