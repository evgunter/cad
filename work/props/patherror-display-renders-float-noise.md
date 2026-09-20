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

## Re-homed to PROPS, 2026-09-20

(DOOR orchestrator) Ev, in chat, 2026-09-20: *"can you kick all the design decisions back to
the track they actually belong to, leaving fix design-free?"* — asked of
FIX and applied to DOOR in the same sitting. **DOOR claims no paths**, so
unlike FIX it can never be the owning track for any decision: there is no
row here whose surface this program owns. A row needing a decision
therefore always leaves. That also retires the charter clause admitting
*"a small design call (where a shared helper's home goes, what a door
looks like)"* — DOOR's own rule already said *"a row that grows a design
question stops being this program's"*, and the two clauses contradicted
each other.

**The decision this row is blocked on:** where the `num` helper lives once a second crate consumes it —
`crates/geom-core/src` beside `Real` is the row's own obvious answer — and
whether the remaining error types get the same treatment.

**Why PROPS.** The helper's candidate home, `crates/geom-core/src/real.rs`, is **owned by
props**, and the row's remaining half is *`ProfileError` and the other
crates' error types that carry scalar payloads*. The `profile` half is
already done and is not PROPS's: all 38 arms across three `Display` impls in
`crates/profile/src/path.rs` (PATHS's) go through the helper.

**A correction that matters more than the routing, and it is why this note
is long.** `work/door/plan.md` carried a long *Review posture* section
prescribing the rounding point this row should adopt — an absolute cap at
`DEFAULT_EPS / 10` met with a relative arm, the finer grid winning, spelled
`tol = (DEFAULT_EPS * 0.1).min(x.abs() * RELATIVE)`, with `RELATIVE` left for
the lane to pick and argue. **That is not an open choice: it is the code in
the tree.** `crates/profile/src/path.rs`'s `num` reads
`let tol = (DEFAULT_EPS * 0.1).min(x.abs() * 1e-9);` today, landed by FIX's
PR 2399 on 2026-09-12 closing
`num-relative-tolerance-collides-above-a-decimetre`, together with the
reasoning now in the helper's own doc comment (why the cap is compile-time
`DEFAULT_EPS` and never the run's live `Tolerance::eps()`). The plan was
written the day before that landed and was never re-read against the tree.
The section is lifted here rather than deleted with the plan, as a RECORD of
what was decided and where it landed — not as an instruction to a lane.

**So the blocker this row names has fired.** Its body says a lane widening
`num` to a second crate *"should land after it, or carry it"*, naming the
relative-tolerance row; that row closed 2026-09-12.

**The trap in the vocabulary, which cost a reader once already.** The plan
calls `DEFAULT_EPS / 10` *"an absolute floor"* while this row's body says
*"do not reintroduce an absolute floor at the small end"* — they mean
opposite things. `min` CAPS the tolerance, making the grid finer at large
magnitudes; the defect PR 2366 removed was `1e-9 * x.abs().max(1.0)`, which
FLOORED the tolerance and rendered every sub-nanometre margin as `0`. The
helper's doc now states this at the site: *"a FLOOR under the tolerance is
the mirror defect and is precisely what a `min` cannot become"*. Read the
doc, not either summary.

**What is actually left**, therefore: the home question above, and the sweep
of the other crates' error `Display` arms. FIX kept the one PATHS-side
instance of the same class as a written fix
(`fillet-leg-carrier-renders-raw-float-noise`,
`crates/profile/src/validate.rs`) — worth reading beside this row, since a
second consumer of `num` outside `path.rs` is exactly what forces the home
question this row asks.

Nothing about the finding is changed by the move: same id, same
evidence, still `open`, and no part of its question is answered for you.
