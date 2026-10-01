# AUTH-8 — an assertion row shows its verdict

**Row**: `work/author/an-assertion-row-shows-no-verdict` (P0, M), which
AUTH-7's lane filed. Read it in full, and read AUTH-7's PR #3528,
because this unit extends what that PR built.

**Branch** `author/assertion-verdict`. **Never merge; I merge.**

## Why

A document can record "this web is at least 0.5 mm" (E10), violate it,
and still show a tree row the GUI can't tell apart from one that holds.
An assertion row reads `Assertion` and nothing else.

## What is verified (2026-09-30)

- `AssertionVerdict<T>` (`crates/editor-core/src/measure.rs` ~:753) is
  `Holds { measured, bound }`, `Violated { measured, bound }` or
  `Unevaluated { reason }`. The numbers are bare `T` with no dimension.
- `UnevaluatedReason` (~:810) has **more arms than the row lists**:
  `Indeterminate` (the margin sits in the tolerance's sliver band),
  `MeasureUnavailable(MeasureUnavailableAt)`, and a carrier-endpoint arm
  (M10-6). It has a `Display`.
- `Node::Assertion { measure: RecipeNodeId, bound: Expr, … }`
  (`crates/editor-core/src/node.rs` ~:2627) names its measure by id.
  The bound must type-check against **that measure's dimension**. A
  failed or poisoned measure poisons its assertions (F2).
- `tree.rs` now has `Measured` (AUTH-7 renamed it from `Reading`; the
  row's text is stale, so fix that line) and `measured_of`, which
  answers `None` for `ValuePayload::Assertion`.

## The unit

An assertion row shows its verdict, meaning which state it is in and,
where there are numbers, the measured value against the bound. The
unavailable and indeterminate states say why, as the kernel says it.

## Three design calls: decide and say

1. **Where the numbers get their dimension.** My reading is that
   whenever a verdict carries numbers, its measure evaluated to a
   `ValuePayload::Measure { dim, .. }`, because an unavailable or failed
   measure yields `Unevaluated` or poisons the assertion. So the honest
   source is the measure node's landed payload, reached through
   `Node::Assertion::measure`, and the numbers are spelled through
   `props::computed_text` like AUTH-7's. **Check that reading.** If a
   verdict can carry numbers when the measure's payload has no `dim`,
   say so and don't guess one.
2. **How loud a `Violated` row is.** E10 makes an assertion
   report-only: no op consumes a verdict. Today the tree's one loud
   row is a failed node. There are two readings. Report-only says
   nothing about tone, and a violated recorded requirement is exactly
   what an author has to act on, so it should be `Actionable`. Or loud
   means "the run failed", and a violated assertion evaluated fine, so
   it should stay advisory. Pick one and say why. I lean toward
   `Actionable`, but it's your call, and the reviewers will test it. If
   it's genuinely close, I'll put it to Ev.
3. **`Unevaluated(MeasureUnavailable)` must not draw the kernel's
   sentence a second time.** AUTH-7 draws `MeasureUnavailableAt` under
   the measure's own row, so drawing it again one row away is two homes
   for one sentence. Point at the measure's row instead, using the
   chrome's one spelling of a node (`tree::node_number` /
   `repair_wording`), or decide otherwise and say why. The other arms
   (`Indeterminate`, the carrier-endpoint arm) use the kernel's
   `Display`. **Never re-spell kernel prose.** AUTH-6 shipped a copied
   kernel sentence that had already drifted.

Also decide whether the row shows the comparison's direction (≥ or ≤).
If the kernel has a word for the side, use it; otherwise leave it out
and say so. Don't mint one.

## Build on AUTH-7, don't beside it

Extend `Measured`, `measured_of` and `feature_row_ui` (which now owns
the row's whole layout, click handling included) and the pane's
`row_result` / `lines_under`. **Every AUTHOR unit so far, seven of
seven, has minted a fresh duplication while closing one**, usually a
number spelling or a copied sentence. Check your own diff before you
push. The reviewers will be hunting for it.

## If I'm wrong

If anything above is wrong about the tree, say so and don't implement
it.

## Scope, verification, deliverable

In: `tree.rs`, `pane/features.rs`, `props.rs`/`readout.rs` (read-mostly).
Out: `crates/editor-core`. The PROPS row
`measure-assertion-offers-an-unvalued-tighten-and-drops-its-margin` is
related context, not this unit.

- Drive the real row (`feature_row_ui` through `crate::pane::headless`)
  and assert what is PAINTED for each case: Holds, Violated,
  Unevaluated-indeterminate, Unevaluated-measure-unavailable, and an
  assertion poisoned by a failed measure.
- Every stated behaviour gets a row that goes red if it stops being
  true, with the mutation named. Restore mutations from a byte copy
  (never `git checkout --`) and **`touch`** the restored file.
- Local checks: fmt; clippy `--workspace --all-targets --features
  viewer/app -D warnings`; `doc-gate.sh`; every `scripts/gates/*.sh`;
  `--lib` and `--test all` **as separate runs**; `work.py lint`;
  `work.py territory` output in the PR. Seam notes on
  `work/chrome/log.md` / `work/vnews/log.md` if you change
  `pane/features.rs` beyond the additive.
- `CARGO_TARGET_DIR=/root/auth-8-target` on every invocation, including
  excluded roots. Scratch in `/root/auth-8-scratch/`. Run `cargo` via
  `local-scripts/with-build-slot.sh -- <cmd>`.
- CI: confirm the run's head SHA equals your branch head, read `gate
  ok`, and say what `change filter` selected.

Open a PR titled `AUTH-8: an assertion row shows its verdict` that
covers the three calls, the territory output, mutations and §6 rows.
Report to me; don't merge.
