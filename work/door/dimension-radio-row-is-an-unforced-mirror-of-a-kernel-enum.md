---
id: dimension-radio-row-is-an-unforced-mirror-of-a-kernel-enum
kind: issue
title: the new-parameter radio row mirrors editor-core's Dimension completely and nothing forces it
status: open
opened: 2026-09-06
---


Found by the style review of PR 2046 (`view/const-all`).

## The instance

`crates/viewer/src/pane/properties.rs:159-163` builds the new-parameter
radio row from an inline array:

    for (dimension, label) in [
        (Dimension::Length, "Length"),
        (Dimension::Angle, "Angle"),
        (Dimension::Count, "Count"),
        (Dimension::Scalar, "Scalar"),
    ] {

`Dimension` (`crates/editor-core/src/expr.rs:33`) has exactly those
four variants, so this is a COMPLETE hand-written mirror of another
crate's enum, in production code, driving what the user can pick. A
fifth dimension in `editor-core` leaves this row four buttons wide with
no compile error and no red row — the same defect, with the same
consequence, as `forms::BOOLEAN_OPS` had before PR 2387 retired it.

## Why it is filed separately

It is an instance of the class
`work/door/hand-maintained-mirrors-of-a-kernel-enum-are-unforced.md`
stated — "a viewer table that mirrors a vocabulary owned by another
crate" — but that item named only `forms::BOOLEAN_OPS` and
`forms::MATE_PRIMITIVES`, so this row was filed rather than edited into
it. **That item is now closed** (PR 2387), which is what makes this
row the class's live one rather than its sibling.

It also carries a second defect the other two do not: the list is
INLINE in a chrome function rather than a named `const` beside its
neighbours, so neither `grep "const ALL"` nor a table-name scan sees
it. PR 2046's census pass 2 ("an array literal holding two or more
`Type::Variant` entries, anywhere in `crates/viewer/src`") should have
returned it and its disposition is not recorded anywhere in that unit.
`scripts/gates/viewer-vocab-declared-once.sh` does not see it either,
for the same reason and by its own declared blind spot: the gate reads
`const` and `static` ITEMS, and this list is neither.

## The labels are a second copy too

`"Length"`, `"Angle"`, `"Count"`, `"Scalar"` are capitalised variant
identifiers rendered to a user, which
`crates/editor-core/src/expr.rs:46-58` — "the one home of the
dimension-in-prose rule for the crate" — says a dimension reaching a
user is not to be. Worth reading against that clause when this is
taken; it may be a separate finding.

## Claimed by DOOR (2026-09-11)

Moved from `work/view/` by the DOOR orchestrator, with Ev's direction
in-chat and VIEW told, alongside the class head this row names
(`hand-maintained-mirrors-of-a-kernel-enum-are-unforced`, closed by
PR 2387) and on this row's own instruction that whoever takes either
takes both. The
directory is the claim (`work/README.md`); the id is unchanged, and
the body above is unchanged but for the one path citation the move made
stale — its pointer at the class head, repointed from `work/view/` to
`work/door/`.

Its class at the claim is **E**: `Dimension`
(`crates/editor-core/src/expr.rs:33`) is four variants and the fix is
the same publication the class head's is. The second defect this row
carries — the list is INLINE in a chrome function rather than a named
`const`, so no table-name scan sees it — is what makes it worth landing
beside the head rather than after: the sweep that finds the next
instance has to be shaped for inline arrays, not for `const ALL`.

## What PR 2387 settled for this row (2026-09-11)

The class head landed as `topo::BooleanOp::ALL`, so the shape this row
inherits is decided and the remaining work is `Dimension`'s own:

- **`editor-core` publishes `Dimension::ALL`** beside the declaration
  at `crates/editor-core/src/expr.rs:33`, and `pane/properties.rs`
  draws one radio per entry. The viewer writes no list.
- **The labels become an exhaustive `match`**, not a second table — a
  label table would be the same defect one field over. That match is
  also where the capitalised-identifier question above gets answered,
  because it is the one place the words are written.
- **The order becomes the declaration's**, unless there is an argument
  for another that is written down; the boolean row's user-visible
  order changed on exactly that reasoning and it is argued in PR 2387.
- **A census row beside the new `ALL`**, per the idiom — with its
  known hole disclosed rather than overclaimed. See
  `all-census-idiom-forces-the-visit-not-the-update`, which is the
  measurement PR 2387's review made of that instrument.

`editor-core` has no ratified roster of hand-written lists to amend
(that machinery is `crates/viewer/README.md`'s and this list is not in
`crates/viewer/src` as a `const`), so this row does not carry the
README-and-gate half PR 2387 did.
