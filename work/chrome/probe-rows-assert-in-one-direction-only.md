---
id: probe-rows-assert-in-one-direction-only
kind: issue
title: The probe's new rows go red on reach growing and on nothing else, and encode BoundsProbe's constants
status: open
opened: 2026-09-04
refs: [1746]
---

Four findings from CHROME's style lane on PR 1746, all about the rows
that PR added rather than about the fix it made. None of them makes
the PR's claim false — the millimetre seed was verified red by
reverting the arm — so none blocked the merge. They are recorded here
because the rows will outlive the review that read them.

**1. The reach assertion is monotone in the wrong direction.**
`crates/viewer/tests/valid_range.rs:405` asserts
`result.high.limit() < 10.0`. That goes red when the reach GROWS —
the metre-seed regression it was written for — and is satisfied by
every degradation that makes the probe search LESS: a seed collapsing
toward zero, a reach loop exiting early, an origin-only answer. Its
sibling in `story_parametric.rs:471` pairs its claim with a lower
bound (`>= TAPER + 1.0`); this row has no floor. The brief's Q3 asks
for a row that goes red when the guarantee DEGRADES, not only when it
is violated in the one direction someone happened to think of.

**2. Both thresholds silently encode `BoundsProbe`'s constants.**
`10.0` at `valid_range.rs:405` is chosen against `MAX_REACHES = 12`
(`bounds.rs:380`) and `1.0e-4` at `:418` against `MAX_REFINES = 10`
(`:385`); neither cites the constant it depends on. The margin on the
first is only about 2.4× — a millimetre seed reaches ~4.1 m — so
raising `MAX_REACHES` to 14 turns this row red for a reason it is not
about, and the next reader has no pointer telling them why.

**3. The `1e-4` bracket contradicts its own comment**, which claims
closure "to about a thousandth of the seed" — 1e-6 m at a millimetre
seed, two orders tighter than the number beside it.

**4. The "a refused probe lands no reading" rows cannot distinguish
"does not set" from "does not clear".** `valid_range.rs:342` and the
new `story_parametric` block both assert `session.bounds().is_none()`
from a state where no probe has ever landed, so a refusal that
returned before `self.bounds = Some(..)` and a refusal that correctly
cleared a PRIOR reading are indistinguishable. Nothing exercises
probe-then-refuse. `bounds` is discarded only in `request_eval`
(`session.rs:3005`), which a refusal does not reach — so the honest
reading is that a stale reading probably DOES survive a refusal, and
is tolerable only because readers gate on `*probed == target`. The
assertion does not test what its message says it tests.

## Home

CHROME (`work/chrome/`) — `crates/viewer/tests/valid_range.rs` is this
program's own ground and these are its own rows. Rides VIEW's split if
still open when that ratifies, like the rest of the slate.

Signed: (CHROME orchestrator)
