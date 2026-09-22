---
id: probe-rows-assert-in-one-direction-only
kind: issue
title: The probe's new rows go red on reach growing and on nothing else, and encode BoundsProbe's constants
status: open
opened: 2026-09-04
refs: [1746, display-budget-rows-restate-three-private-constants, a-negative-extrude-distance-probes-as-valid]
priority: P3
cost: E
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

**Everything below was measured on the fixture, not reasoned about.**
A first version of this note reasoned, got the downward search wrong,
and said so confidently; the corrections are marked where they land,
because a discharge note is exactly where an unverified causal story
gets enshrined.

**2. Neither threshold is a literal any more, and neither is restated
in the test.** `BoundsProbe::furthest_reach(seed)` is the furthest
offset a reach can place, and `Sweep`'s reach ladder reads the same
private `reach_offset` it does, so the test no longer computes
anything the probe computes. This is the `Camera::pitch_limit` /
`datums::patch_cover` shape: the point is that the DERIVATION has one
home, not that a constant is readable.

**The two halves discriminate different things, and only the first is
about the seed.**

- **Reach (upward).** Threshold is `origin + furthest_reach(one
  written millimetre)` = 2.056 m; measured answer 2.056 m, so the
  `<=` holds exactly. Forcing `probe_seed` to 1.0 gives
  `high: Open { probed: 2048.008 }` — red by three orders. Falsified
  by: the seed, which is the regression this row was written for.
  Not falsified by `MAX_REACHES` moving, which was finding 2's
  complaint.
- **Bracket (downward).** Falsified by the REFINEMENT, not the seed.
  It asserts a COUNT — that the refinement spent all `MAX_REFINES`
  halvings on the bracket it entered (`origin/2` wide) — and not a
  width. Measured: 10.000000000000158 halvings at the default ε,
  9.999999999999519 at `ε = 1e-6`. Planted red at BOTH those ε rows:
  settling one halving early reads 9.00 and the row reds naming the
  count.

  **Why a count and not a width — this is the CI failure of PR
  2683.** The first version compared `valid - invalid` to an ideal
  `origin/2 / 2^MAX_REFINES`. `Bound::Edge` reports ABSOLUTE field
  values, each formed as `origin ± offset`, so their difference
  inherits the ULP of the ORIGIN (1.734e-18 at 0.008) rather than of
  the bracket (~8.5e-22). At the default ε the floor sits at exactly
  0, the low end of the pair is exactly 0, the subtraction is exact
  and the comparison passed **by luck**; at `ε = 1e-6` both ends are
  ordinary non-zero floats and the same rule measured
  3.9062500000013e-6 against 3.90625e-6 — over by 1.304e-18, which is
  0.75 of one ULP of the origin. Two CI jobs failed on it with
  byte-identical numbers (`test (eps = 1e-6, 2/2)` and
  `test (interval, eps = 1e-6, 1/2)`), which is itself the evidence
  that the lane does not reach this path and ε alone does. A halving
  is a factor of two, so the nearest wrong answer is a whole one away
  and no rounding dust reaches it.

  `BoundsProbe::refined_width`, minted in the first fix pass to hold
  that ideal width, is DELETED rather than left public with no
  caller: nothing should ship a door whose only use was an assertion
  that could not be made robust. The law it held is stated on
  `MAX_REFINES`, together with why a check counts halvings instead.

**CORRECTION — the first version of this note was wrong about the
downward search, in the comment, in the helper's doc and here.** It
said a metre seed "brackets its floor to a millimetre" and that "both
assertions still fail on it". Measured, with `probe_seed` forced:

| seed | `result.low` |
| --- | --- |
| 1 mm | `Edge { valid: 3.9e-6, invalid: 0.0 }` |
| 0.8 mm | `Open { probed: -1.6304 }` |
| 1 m | `Open { probed: -2047.992 }` |

A metre seed produces **no bracket at all**, so the bracket assertion
is not reached, let alone failed — execution stops at the `let else`.
And the 0.8 mm row is the one that matters: an ordinary seed, no
bracket. The reason is that **the floor is the single value `0`** — a
negative thickness still builds — so a direction brackets it only
when a doubling lands exactly on it, which a millimetre ladder does
because the floor is 8 seeds out and 8 is a power of two. The old
`finest_bracket` helper stated a universal upper bound (`2·distance /
2^MAX_REFINES`) that the code does not hold in that case, and used it
as a discriminator. All three spellings are gone. The measurement is
filed as its own row,
`work/chrome/a-negative-extrude-distance-probes-as-valid.md`, because
it is a finding about the kernel's reading of a length field and not
about these rows.

**PREMISE CORRECTION — the floor's LOCATION moves with ε, and two
earlier notes here stated it flatly.** Measured on this fixture at
three ε rows, failing nodes by thickness:

| thickness | default ε | `1e-6` | `1e-12` |
| --- | --- | --- | --- |
| -8 mm, -1 mm | builds | builds | builds |
| 0 | FAILS | FAILS | FAILS |
| 1e-6, 5e-6 | builds | **FAILS** | builds |
| 1e-5 and up | builds | builds | builds |

So "the floor is the single value `0`" is true at the default ε and
at `1e-12` and FALSE at `1e-6`, where everything below about `1e-5`
fails — one rule (an extrusion the tolerance cannot tell from zero)
whose width is ε's to set. What does NOT move is that a negative
thickness builds at every ε, which is the finding
`work/chrome/a-negative-extrude-distance-probes-as-valid.md` carries.
The row's prose and that row both say this now; neither may assume a
point.

**3. The stale premise was in `bounds.rs`, and it is now fixed rather
than quoted.** This is a CORRECTION of the first note, which quoted
`MAX_REFINES`' own doc approvingly as "the comment was right". It was
not right. `MAX_REFINES` said "ten halvings take a bracket to about a
thousandth of the SEED STEP" and `BoundsProbe::new` said "the finest
bracket is about a thousandth of one [seed]". Both state the wrong
law: the halvings divide **the bracket the refinement entered**, which
is the reach stride that caught the failure. Measured on this fixture:
a 1e-3 seed closes to 3.9e-6, four times looser than a thousandth of
the seed, because the stride that caught the floor was four seeds
wide. Both sentences are rewritten to the law the code holds, and
`MAX_REFINES` now also carries why a CHECK on that law counts halvings
rather than measuring the reported pair. With the premise gone from
every spelling, finding 3 is discharged on the merits rather than by
adjudication.

**Findings 1 and 4 are untouched.** The reach assertion is still
one-sided (`<=`), so every degradation finding 1 lists still satisfies
it. Nothing here states a degradation guarantee, because nothing has
ruled on what one is.

**Residue, disclosed and NOT scheduled here.** `probe_seed`
(`crates/viewer/src/session/probe.rs`) says it is "the one place that
arithmetic is spelled", and the reach row still states one written
millimetre for itself. That restatement is deliberate and argued in
the comment — the rule under test cannot be imported from the code
under test, the argument `tests/edge_pick.rs` makes for its own
occlusion band — but if a later lane decides the probe should expose
its seed as a contract the way `BoundsProbe` now exposes its reach,
that is a change in `session/probe.rs`, which this lane was fenced
out of.

A sweep of `crates/viewer/tests/` for the same shape (a row restating
a constant that lives in `src/` as a literal) found three more in
`display_budget.rs` and filed them as
`work/chrome/display-budget-rows-restate-three-private-constants.md`;
that row also records the sites where NOT restating is the argued
position, and what the sweep could not see.

## Home

CHROME (`work/chrome/`) — `crates/viewer/tests/valid_range.rs` is this
program's own ground and these are its own rows. Rides VIEW's split if
still open when that ratifies, like the rest of the slate.

Signed: (CHROME orchestrator)
