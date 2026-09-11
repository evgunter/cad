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
consequence, as `forms::BOOLEAN_OPS`.

## Why it is filed separately

It is an instance of the class
`work/door/hand-maintained-mirrors-of-a-kernel-enum-are-unforced.md`
states — "a viewer table that mirrors a vocabulary owned by another
crate" — but that item names only `forms::BOOLEAN_OPS` and
`forms::MATE_PRIMITIVES` and is being landed by PR 2046, so this row
is filed rather than edited into it. Whoever takes either should take
both; three instances is what the class actually has inside
`crates/viewer/src`.

It also carries a second defect the other two do not: the list is
INLINE in a chrome function rather than a named `const` beside its
neighbours, so neither `grep "const ALL"` nor a table-name scan sees
it. PR 2046's census pass 2 ("an array literal holding two or more
`Type::Variant` entries, anywhere in `crates/viewer/src`") should have
returned it and its disposition is not recorded anywhere in that unit.

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
(`hand-maintained-mirrors-of-a-kernel-enum-are-unforced`) and on this
row's own instruction that whoever takes either takes both. The
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
