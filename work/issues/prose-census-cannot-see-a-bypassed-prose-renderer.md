---
id: prose-census-cannot-see-a-bypassed-prose-renderer
kind: issue
title: prose_census cannot see a Display that delegates, nor a Debug that spells an identifier
status: open
opened: 2026-09-06
refs: [the-prose-word-for-a-kind-has-four-spellings-and-only-display-is-censused, 2053]
---


Found by PR 2053 (`view/refusal-all`), which fixed a leak that was
green under both of the tree's prose instruments. Filed here rather
than on `work/lib/`'s slate because
`docs/prompts/implementer-discipline.md` §6 forbids a unit branch
filing into another program's directory, and `work/README.md` requires
a disclosed residue to get a file — `work/issues/` is where those two
meet. The census is LIB's (`crates/pncad-py/*`); the change is LIB's to
take.

## The defect that motivated it

`Refusal::exists_wording` (`crates/viewer/src/session/refuse.rs`)
rendered `({dimension:?})`, so the status line and the add-parameter
form both showed `Length` where the ratified rule
(`crates/editor-core/src/expr.rs`, `Dimension`'s `Display`, "the one
home of the dimension-in-prose rule") says a user reads `length`. It
was green under `prose_census` for TWO independent reasons, and that
is why this is one row rather than two.

## Gap 1 — the scan set is `impl Display` bodies

`census()` (`crates/pncad-py/src/prose_census.rs:963-969`) walks
`code.match_indices("Display for ")` and reads formatting calls in
those bodies only. But a vocabulary may compose its sentences in an
INHERENT impl on purpose: `Refusal::affordance`, `exists_wording` and
`offer_wording` are each documented as "its one home" precisely so a
pre-click surface and the status line cannot drift, and
`Display for Refusal` then delegates through a bare `{}`. **A
delegated wording is outside the census.**

Measured: **11** `{x:?}` sites tree-wide sit in inherent impls of
`Display`-carrying types (12 before PR 2053's fix removed one). The
other ten are `__repr__`-shaped, in `pncad-py`'s own `py/` modules and
`test-utils`.

## Gap 2 — the verdict asks about braces, not about identifiers

Had the census seen the site, it would still have passed it.
`declaration_verdict` answers `VariantShape::Unit => Verdict::Prose`,
so a fieldless enum is prose — correctly, for the question the module
asks, which is whether a `Debug` can carry the `" { "` fingerprint the
prose gate rejects. `Dimension` is fieldless. Its `Debug` carries no
brace and writes `Length`.

The verdict that reaches this is **"the site bypassed a prose renderer
that exists"** — a `{x:?}` whose resolved type has an `impl Display`
in the tree. Scale: **453** `{x:?}` placeholders sit inside `impl
Display` bodies tree-wide (a looser regex gives 455), against 224
types carrying a `Display`, so how many are real bypasses is unmeasured
and is the first thing a lane taking this should count.

## Why one row and not two

Neither half alone catches the founding defect. Widening the scan set
leaves `Dimension` a `Prose` verdict; adding the bypass verdict leaves
`exists_wording` unread. A lane that took one and shipped it would have
a guard that still passes over the case this row exists for, and would
reasonably believe the class was closed.

## Relation to the row beside it

`work/issues/the-prose-word-for-a-kind-has-four-spellings-and-only-display-is-censused.md`
(filed the same day, by the same review) names both of these facts as
EVIDENCE for a different thesis — that the rule "a kind reaches a user
as a common noun" has four implementations and the census reads one.
Its two decisions are which spelling is the home, and whether the
census can be taught to see past a delegation; **this row is that
second decision, with its measurements**, and it is deliberately
narrower: it asks nothing about where the rule should live, only what
the instrument must do to see a violation of it.

That row's `SlotId` instance (`crates/editor-core/src/edit.rs`'s six
`{slot:?}` renderings, disclosed in that module's own header) is
reached by Gap 2 and NOT by Gap 1 — `SlotId` has no `Display` at all,
only an inherent `label()`, so a bypass verdict keyed on `Display`
would still miss it. Whoever takes this should decide whether the
verdict keys on `Display` or on "a prose renderer, however spelled",
which is exactly where the two rows meet.
