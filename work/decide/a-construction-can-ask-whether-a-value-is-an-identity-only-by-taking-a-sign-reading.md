---
id: a-construction-can-ask-whether-a-value-is-an-identity-only-by-taking-a-sign-reading
kind: issue
title: A construction can ask whether a value is an identity of the parameters only by taking a sign reading, which counts as a decision and leaves a deferred entry
status: open
opened: 2026-11-15
priority: P2
cost: M
---


Filed by the BOXFINISH orchestrator, from the repair of PR 6963's
direction mint (`work/boxfinish/main-red-boxfinishs-merges-left-slow-rows-red-through-the-gates-slow-set-gap.md`).
A design question on the decision door, not a defect: nothing is wrong
today.

**What a construction can ask.** `profile::seg::axis_or` mints a line's
unit director. Where one component of the chord is zero AS AN IDENTITY
of the parameters (a hop `W − W`), the director is the constant axis
`(0, ±1)`, with no width over a box. Where the component is zero only
at this valuation (two different corner variables that agree at the
nominal), the director must be the quotient, or the same document
re-valued stores something else (E12; five `revalue_corpus` rows caught
this on main).

So the mint needs to know whether a value is an identity, and the only
ways to ask are sign readings:
- `decide(..)` and then `MarginDiag::proven()`: stamped at decision
  time outside a deferring session;
- inside a deferring session (`with_replay_session`) every Zero is
  stamped `Deferred`, so the caller must also call
  `sym::deferred::discharge(margin)`.

**What that costs,** measured by the repair's lane:
- each reading counts in `SymCounts`' symbolic or numeric column,
  whether or not its answer is used; `sym::aside` + `Aside::discard`
  cuts the shape report and the re-valuation recorder back, and by its
  own doc does not cut the counts. On `sym11`'s four documents that is
  +4, +1, +8 and +16 readings;
- a discarded Zero in a deferring session leaves a `Pending::Zero`
  entry that nobody is obliged to discharge.

**The question.** Whether `Decide` (or the symbolic tier) should offer
a query that is not a decision, "is this value an identity of the
parameters", answering by the tier's identity test and by nothing at
`f64` and `Interval`, with no count and no pending entry. The lane's
sketch: `fn proves_zero(self) -> bool`, default `false`, `Sym`
answering `discharge_retried` (memoised per session).

**What to weigh, not weighed here.**
- A branch on proof with no record is a new kind of door; what stops
  it being used where a decision belongs.
- `geom_core::is_exact_zero` already reads "exactly zero" at every
  scalar, and at `Sym` it reads the value channel. Whether its callers
  at `Sym` have the same value-coincidence exposure, and whether the
  two queries are one.
- At plain `Interval` there is no re-valuation, so a degenerate
  `[0, 0]` component could mint its axis; under the repair it takes
  the quotient and the director's `±1` carries rounding width.

**Not blocking.** The repair mints the axis through the existing parts
and re-baselines the counts. This row is for whoever next needs to ask
the question at a site where the counts or the pending entries matter.

**Weighed as fork 272 (2026-11-15)**, with the mint's own row
`work/boxfinish/the-axis-mint-asks-for-proof-only-where-the-value-reads-zero.md`,
which holds the recommendation: `is_exact_zero` itself becomes the
structural question, answered from the expression at `Sym`. The
"not blocking" line above no longer holds: PR 6990's review showed the
sign-reading spelling takes two branches for one document.

## Evidence (2026-11-15, PR 6990's reviews): "was this Zero proven" has three spellings

PR 6990 gave the profile's axis mint one spelling,
`geom_core::sym::deferred::discharge(margin)` (true for a margin proven
eagerly, and for one a deferring session discharges). Two more stand in
production code, each asking the same question of a `Decided`:

- `topo` `coincidence.rs`, `Discharge::of`: `MarginDiag::proven()`
  alone, which does not see a deferred Zero's discharge;
- `editor-core` `coincide/replay.rs`: `discharge_or_why`, at three call
  sites (the deferring replay lane).

Three presentation readers of `proven()` in `geom-core`'s
`predicate.rs` only report. Whatever answers this row gives the
question one home; until then a fourth site has three examples to copy.
(BOXFINISH orchestrator)

## The question is answered (2026-11-24, PR 7488)

The query that is not a decision exists: `Expression::proves_zero`, a
method of a scalar's expression (`Real::Expression`), which a numeric
scalar does not have. It counts nothing, defers nothing and reads no
value; the profile's axis mint asks it. So the `design` flag is cleared.

What this row still holds is the evidence section above: "was this Zero
proven" keeps three spellings among the readers of a decided margin
(`topo`'s `Discharge::of`, `editor-core`'s `discharge_or_why`, and
`sym::deferred::discharge`), none of which stores a spelling on its
answer. (BOXFINISH orchestrator)
