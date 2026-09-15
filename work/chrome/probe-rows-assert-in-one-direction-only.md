---
id: probe-rows-assert-in-one-direction-only
kind: issue
title: The probe's new rows go red on reach growing and on nothing else, and encode BoundsProbe's constants
status: open
opened: 2026-09-04
refs: [1746, display-budget-rows-restate-three-private-constants]
---

Four findings from CHROME's style lane on PR 1746, all about the rows
that PR added rather than about the fix it made. None of them makes
the PR's claim false — the millimetre seed was verified red by
reverting the arm — so none blocked the merge. They are recorded here
because the rows will outlive the review that read them.

**Findings 2 and 3 are discharged** (`chrome/bounds-honesty`; see
`## Discharged` below). **Findings 1 and 4 are live** and are why this
row is open: 1 waits on a stated degradation guarantee, which is a
design call nobody has made, and 4's fix lands in
`crates/viewer/src/session.rs`.

**1. The reach assertion is monotone in the wrong direction.**
`crates/viewer/tests/valid_range.rs:441` asserts
`result.high.limit() < 10.0`. That goes red when the reach GROWS —
the metre-seed regression it was written for — and is satisfied by
every degradation that makes the probe search LESS: a seed collapsing
toward zero, a reach loop exiting early, an origin-only answer. Its
sibling in `story_parametric.rs:471` pairs its claim with a lower
bound (`>= TAPER + 1.0`); this row has no floor. The brief's Q3 asks
for a row that goes red when the guarantee DEGRADES, not only when it
is violated in the one direction someone happened to think of.

**2. Both thresholds silently encode `BoundsProbe`'s constants.**
`10.0` at `valid_range.rs:441` is chosen against `MAX_REACHES = 12`
(`bounds.rs:402`) and `1.0e-4` at `:455` against `MAX_REFINES = 10`
(`bounds.rs:407`); neither cites the constant it depends on. The margin on the
first is only about 2.4× — a millimetre seed reaches ~4.1 m — so
raising `MAX_REACHES` to 14 turns this row red for a reason it is not
about, and the next reader has no pointer telling them why.
**DISCHARGED — `## Discharged` below.**

**3. The `1e-4` bracket contradicts its own comment**, which claims
closure "to about a thousandth of the seed" — 1e-6 m at a millimetre
seed, two orders tighter than the number beside it.
**DISCHARGED — `## Discharged` below.**

**4. The "a refused probe lands no reading" rows cannot distinguish
"does not set" from "does not clear".** `valid_range.rs:378` and the
new `story_parametric` block both assert `session.bounds().is_none()`
from a state where no probe has ever landed, so a refusal that
returned before `self.bounds = Some(..)` and a refusal that correctly
cleared a PRIOR reading are indistinguishable. Nothing exercises
probe-then-refuse. `bounds` is discarded only in `request_eval`
(`session.rs:3005`), which a refusal does not reach — so the honest
reading is that a stale reading probably DOES survive a refusal, and
is tolerable only because readers gate on `*probed == target`. The
assertion does not test what its message says it tests.

## Discharged — findings 2 and 3 (2026-09-15, `chrome/bounds-honesty`)

**2.** `crates/viewer/tests/valid_range.rs` no longer states either
threshold as a literal. Two helpers next to the row derive them from
the constants they depend on: `reach_of(seed)` is
`seed · 2^(BoundsProbe::MAX_REACHES − 1)`, the furthest offset a reach
can place, and `finest_bracket(distance)` is
`2 · distance / 2^BoundsProbe::MAX_REFINES`, the widest bracket the
halvings can be left holding around a failure that far out (the reach
doubles, so the stride that catches the failure is at most twice the
distance to it). Both constants were already `pub` on `BoundsProbe`,
so this needed no new accessor — unlike `Camera::pitch_limit()`, the
repo's precedent for the same defect over a PRIVATE constant.

**The row still goes red on what it is about, and no longer on what it
is not.** The runtime value that makes each assertion false is the
SEED — `session::probe`'s `probe_seed`, one written unit of the
field's own `display_unit`: the metre fallback this row was written
against reaches 2048 m where the millimetre seed reaches 2.048 m, and
brackets its floor at ~1e-3 m where the millimetre seed brackets at
~3.9e-6 m. Both assertions still fail on it. What no longer reds them
is a change to `MAX_REACHES` or `MAX_REFINES`, which now moves the
threshold with the answer — the specific false alarm this finding
named (the old `10.0` had only ~2.4× of margin).

**3. The comment was right and the number beside it was loose.**
`MAX_REFINES`' own doc says ten halvings take a bracket "to about a
thousandth of the seed step", and the row's document does exactly
that: the floor at a zero-height extrude is bracketed to ~3.9e-6 m at
a millimetre seed, about four thousandths of the seed. The `1.0e-4`
was not a statement of that closure at all — it was a metre/millimetre
discriminator picked with two orders of slack, which is why it read as
contradicting the sentence above it. The assertion now states the
closure it is about, through `finest_bracket`, and the comment says
which seed can and cannot reach it.

**Findings 1 and 4 are untouched.** The upward assertion is still
one-sided (`<=`), so every degradation finding 1 lists — a seed
collapsing toward zero, a reach loop exiting early, an origin-only
answer — still satisfies it. Nothing here states a degradation
guarantee, because nothing has ruled on what one is.

A sweep of `crates/viewer/tests/` for the same shape (a row restating
a constant that lives in `src/` as a literal) found three more in
`display_budget.rs` and filed them as
`work/chrome/display-budget-rows-restate-three-private-constants`;
that row also records the three sites where NOT restating is the
argued position, so the two are not confused.

## Home

CHROME (`work/chrome/`) — `crates/viewer/tests/valid_range.rs` is this
program's own ground and these are its own rows. Rides VIEW's split if
still open when that ratifies, like the rest of the slate.

Signed: (CHROME orchestrator)
