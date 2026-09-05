---
id: sweep-blind-spots-the-precheck-sweep-could-not-see
kind: issue
title: "What the layer-3 pre-check sweep could not see: three named blind spots"
status: open
opened: 2026-09-04
refs: [set-param-prechecks-what-the-door-refuses, self-boolean-precheck-duplicates-the-doors-duplicate-input, 1846]
---


`set-param-prechecks-what-the-door-refuses` (#1846) swept for *a
layer-3 pre-check of a condition a `DocEdit` refuses typed* and found
two instances. Its blind spots were disclosed in the PR body and
nowhere else; this file is them, so that what the sweep could not see
survives the deletion of VIEW's directory. **The sweep is accurate as
of #1846's merge base and nothing re-runs it.**

## The two patterns that ran

1. Every `OpOutcome::refused(...)` preceding a `self.commit(…)` /
   `self.commit_action(…)` in the same function. All 41 remaining
   `OpOutcome::refused` sites live in `crates/viewer/src/session.rs`;
   16 functions matched.
2. Every helper returning `Result<_, Refusal>`
   (`grep -rn "Result<[^>]*Refusal>" crates/viewer/src/`), then each
   caller — for pre-checks raised through `?` rather than an early
   return. Five helpers: `driver_of` (`session.rs:833`),
   `guard_driven` (`:842`), `require_kind` (`:1334`),
   `part_catalogue` (`:779`), and `probe::probe_bounds` with its seed
   reader in `session/probe.rs`.

## Blind spot 1 — CLOSED

*A pre-check separated from its commit by a helper call, where the
helper does not itself name `Refusal`.* Discharged for `session.rs` by
the #1846 style review, which walked every document read that precedes
a commit and classified each as either data the edit needs, a genuine
lookup, the cascade order, or one of the four gates #1846
dispositioned. Nothing was left over. **Closed for `session.rs`; not
run for any other file.**

## Blind spot 2 — OPEN

*A pre-check that never mentions `Refusal` at all*, because it returns
`OpOutcome::default()` and silently declines instead of refusing. Both
patterns keyed on a refusal being constructed, so a door that does
nothing quietly is invisible to them. Not searched. The shape to look
for is an early `return OpOutcome::default()` guarded by a condition
about the document.

## Blind spot 3 — OPEN, and the largest

**Both patterns keyed on the type `Refusal`.** The rule they enforce —
layer 3 must not restate what a lower layer refuses typed — is about a
LAYERING, not about one enum, and `crates/viewer/src/` holds 21 other
typed fault enums (`grep -rn "pub enum .*Error\|pub enum .*Fault"
crates/viewer/src/`). Two carry the same shape one layer down and were
found by the #1846 style review, not by either pattern:

- **`BlendError::NoEdges`** (`crates/viewer/src/blend.rs:170-180`).
  Its own doc-comment names the lower rule it duplicates:
  *"`NodeErrorKind::BlendSelectionEmpty` refuses an empty selection at
  evaluation, so a hand-written recipe gets the same answer as an
  authored one."*
- **`MateToolError::SamePick`** (`crates/viewer/src/matetool.rs:216-223`).
  Likewise: *"the tool refuses here rather than authoring the edit the
  solve would refuse as a self-mate."*

Both differ from #1846's defect in one way that matters and must not be
skipped over: the lower refusal arrives at **evaluation or solve time**,
not at the edit door, so the pre-check buys the user an answer *instead
of* a landed-then-broken node rather than *ahead of* an identical
answer. That is the same trade `add_profile`'s doc-comment states and
#1846 dispositioned as correct for `require_kind`. Both also STATE the
duplication rather than hiding it, which is the discipline working.

So this is not a claim that either is a defect. It is the record that
**the sweep never looked**, that the rule generalises past `Refusal`,
and that `crates/viewer/README.md`'s delegation clause is written about
`Refusal` alone while the other 21 enums have no such clause anywhere.

## The one gate the sweep found and left standing

`driver_of`'s `Refusal::NoSuchSlot` (`session.rs:837`) IS a condition
`apply` refuses typed (`EditError::UnknownNode` / `UnknownSlot`,
`crates/editor-core/src/edit.rs:1942-1947`), and it is deliberately
kept: `driver_of` must run anyway to get the driver for
`DrivenByExpression`, and it reads `props::slot_rows` — the panel's
projection — not the node's slot vocabulary, so the two conditions are
not the same set. Recorded here because "a lookup, not a pre-check" is
the load-bearing distinction and the case that most nearly fails it.

## Blind spot 2 — RUN, and empty (VIEW orchestrator, 2026-09-04)

The shape this item said to look for was *"an early
`return OpOutcome::default()` guarded by a condition about the
document"* — a door that declines silently instead of refusing, and so
is invisible to both of #1846's patterns because neither fires without
a `Refusal` being constructed.

Run against the tree at `8604dfb3`:

- `return OpOutcome::default()` across `crates/viewer/src/` — **one
  hit**, `session.rs:1088`, inside `commit_gesture`. It is **not an
  instance.** It is the ratified no-move rule (*a gesture that never
  previewed commits nothing*), it is argued for six lines above itself,
  and the condition it tests is about the GESTURE, not about the
  document — there is no lower door that refuses it, because there is
  no edit.
- `OpOutcome::default()` in any position — 19 hits, all tail-position
  successes.
- No guarded (`if` / `let Some` / `let Ok` / `match`-arm) construction
  precedes any of them in `session.rs`.

**What this sweep could not match**, stated because the item's own
argument is that an undisclosed blind spot is an unverified claim:

- a door that declines by returning a *non-default* outcome that
  nonetheless does nothing (an empty `previewed`, a `superseded` with
  no entries) — the pattern keys on the constructor, not on the
  emptiness;
- a decline expressed as an `if` with no `else` around the commit,
  where the function falls through to a shared tail — the commonest
  spelling of "quietly do nothing" and the one a `return` grep cannot
  see;
- anything outside `crates/viewer/src/`.

**Blind spot 2 is CLOSED for the `return`-shaped spelling and stays
open for the fall-through spelling**, which nobody has looked for.
Blind spot 3 (the rule generalises past `Refusal` to the other 21
typed fault enums in the crate) is untouched and is the large one.
