---
id: prose-census-cannot-see-a-bypassed-prose-renderer
kind: issue
title: prose_census cannot see a Display that delegates, nor a Debug that spells an identifier
status: open
opened: 2026-09-06
refs: [the-prose-word-for-a-kind-has-four-spellings-and-only-display-is-censused, 2053]
priority: P4
cost: E
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

## Re-homed to CENSUS (2026-09-11, the cut in `docs/WORK-TRACKS-2026-09.md` addendum 3)

CENSUS collects the rows of one class: a vocabulary spelled by hand in
several places, and the census or instrument that cannot see one of the
spellings. This row is a member of that class.

Its class at the cut was **H** — instrument rework plus triage of 453
unmeasured sites; verdict key is a design choice. The class is a
dispatch estimate made by reading the row against the tree on
2026-09-11, not a verdict on the finding, and a lane that finds it wrong
says so in its PR. The id, the `track:` letter where the row carries
one, and the body above are unchanged by the move.

## Two more Gap-2 instances, and a way the two blind spots feed each other (2026-09-16, EDIT's `edit/error-prose`)

Measured while repairing `editor-core`'s `Debug`-in-prose sites. Added
here as evidence rather than as a second row, per
`docs/prompts/implementer-discipline.md` §6.

**`PersistError::DisplayUnit` rendered `Dimension` through `Debug` at
two placeholders** (`crates/editor-core/src/persist/mod.rs`):
`"… is declared {declared:?} but its display unit measures {unit:?}"`,
both fields `crate::expr::Dimension`. `Dimension` is the type this row's
founding defect was about, its `Display` is "the one home of the
dimension-in-prose rule", and the census was green over both
placeholders for exactly the reason stated above: `Dimension` is
fieldless, so `declaration_verdict` answers `Prose` and the site is not
in any roster, flagged or allowlisted. Repaired in that unit.

It is also a counter-example to a closed row's closing claim.
`work/fix/verb-and-dimension-render-through-debug`'s *What landed*
section says *"Re-swept: no `Dimension` reaches any user surface through
`Debug` anywhere in the tree"* — that re-sweep found the four viewer
labels and missed this door, which has been in `editor-core` throughout.
A sweep with no instrument behind it is what Gap 2 costs: there is no
guard that would have disagreed.

**`StepArg` is the same shape as the `SlotId` instance this row already
names.** `crates/editor-core/src/persist/check.rs`'s `ProgramFault`
rendered `{arg:?}` over a `StepArg`, which is fieldless and carries an
inherent `StepArg::label()` — no `Display`, exactly like `SlotId`. So a
bypass verdict keyed on `Display` would miss it too, which is the second
data point for the question this row leaves open ("whether the verdict
keys on `Display` or on 'a prose renderer, however spelled'").

**The interaction worth knowing before either row is taken.** That
`{arg:?}` site was NOT silently passed — it sat in `UNDECIDED`, because
its binding is introduced by a nested pattern
(`work/fix/census-cannot-type-a-nested-pattern-binding`). Repairing that
row alone would have resolved the binding to `StepArg`, and the site
would have moved from a named blind spot to a silent `Prose` pass. A
census that gets better at typing bindings gets *quieter* about this
class until Gap 2 is closed, so the two rows want ordering, or one
lane.
