---
id: patherror-display-renders-float-noise
kind: issue
title: `PathError`'s Display arms render scalars with `{:?}`, so refusal sentences carry round-tripped float noise
status: open
opened: 2026-08-30
github: 1282
refs: [1267, num-relative-tolerance-collides-above-a-decimetre]
---

## From GitHub issue 1282

Opened 2026-08-30; 0 comments.

A **class**, split out of the BLEND-7 review (PR #1267) rather than swept in there.

`Real` carries `Debug` and no `Display`, so every arm of `impl Display for PathError<T>` reaches its scalar payloads through `{:?}`. For `f64` that is the shortest round-tripping form, which is exactly right for a diagnostic dump and wrong in a sentence a person reads: an 8 mm radius that arithmetic produced renders as

> …tangent setback 0.008000000000000002 m exceeds the 0.0034999999999999996 m the anchor pins…

The same applies to `ProfileError` and to the other doors' error types that carry scalar payloads.

## What PR #1267 did

Only the arm it added (`FilletEnclosesLegCarrier`) renders through a small private helper, `path::num`, which prints the shortest decimal that still names the same number to a relative 1e-9 and passes non-`f64` `Debug` forms (intervals, duals) through untouched. It is a display choice only — the payload keeps the exact scalar, and nothing branches on the string.

## What is open

Whether to apply the same treatment across the existing arms, and where the helper belongs if so (several crates have the same shape, so `geom-core` beside `Real` is the obvious home). Worth deciding once: a per-arm trickle would leave the two spellings side by side indefinitely, which is its own smell.

— Filed by the BLEND-7 implementer lane, adjudicating both blinded reviews.

## Home

`work/code-quality/` — this is a structural finding about two spellings of one job (`{:?}` scalars versus `path::num`) living side by side across crates, which is the register's stated subject.

## Re-homed to DOOR (2026-09-11, the cut in `docs/WORK-TRACKS-2026-09.md` addendum 3)

DOOR collects the rows whose fix is already written in the row — one PR
each, no design question left open. This row is here because a lane can
take it and land it without deciding anything first.

Its class at the cut was **M** — decide helper's home once, then
mechanical sweep of Display arms. The class is a dispatch estimate made
by reading the row against the tree on 2026-09-11, not a verdict on the
finding, and a lane that finds it wrong says so in its PR. The id, the
`track:` letter where the row carries one, and the body above are
unchanged by the move.

## Narrowed (2026-09-11, the DOOR orchestrator) — the `profile` half is done

Read against the tree before dispatch. **The row's motivating example
already renders correctly**, and most of what it asks for landed while
it sat in the pile.

Executed over the current `num`'s exact body:
`0.008000000000000002` renders `0.008`, `0.0034999999999999996`
renders `0.0035`. FIX's closed row
`path-error-numbers-below-1e-9-render-as-zero` (PR #2366) records that
`num` already backs **38 call sites across three `Display` impls** in
that file — `CornerRefusal`, `CornerReason` and `PathError` — so *"apply
the same treatment across the existing arms"* is answered for
`crates/profile/src/path.rs`: every arm goes through the helper.

**What is left of this row is its second half only**: `ProfileError` and
the other crates' error types that carry scalar payloads, and where the
helper lives if it is to serve them. That half is untouched and is
still `M`. The helper is still private to `path.rs`, so a second
consumer is what forces the home question, and `geom-core` beside
`Real` remains the obvious answer.

**Two things this row must NOT do, both learned from FIX's two closed
rows on this helper:**

- **Do not reintroduce an absolute floor at the small end.** `num` used
  to read `tol = 1e-9 * x.abs().max(1.0)`, and that `.max(1.0)` pinned
  the tolerance absolute for `|x| <= 1`, rendering every sub-nanometre
  margin as `0` — the margins these messages exist to report, since a
  junction is tangent precisely when its margin is below threshold.
  #2366 removed it. The purely relative form is the repair.
- **Do not propagate the constant without reading it.** The relative
  1e-9 is correct below a decimetre and coarser than ε above one; that
  is a separate defect, filed on FIX's slate as
  `num-relative-tolerance-collides-above-a-decimetre` rather than
  carried here, because the helper is FIX's ground. A lane widening
  `num` to a second crate should land after it, or carry it.
