---
id: certify-affordance-on-the-bounds-panel
kind: unit
title: The bounds panel has no on-demand certify affordance for the certified range query
status: closed
opened: 2026-09-13
refs: [1183]
priority: P1
cost: H
branch: offer/range-is-the-certified-search
pr: 7548
closed: 2026-11-23
---

Filed by DOCM-9, which built the kernel side and was forbidden by its
own fence from building either consumer door. DOCM closed on
2026-09-14 (`docs/DOC-LEDGER.md`), so there is nobody to hand this
back to: the query exists and the consumer door is CHROME's to build.

`editor_core::range::certified_range` answers, on demand, the question
`viewer::bounds` answers by sampling — over which interval around a
field's current value is the document PROVABLY the same build. Ev's
2026-09-13 ruling on pacing is that the sampling probe STAYS the
interactive answer and the certified range is an on-demand query whose
result arrives later and REPLACES the probe's reading. Its record is
DOCM-9's spec, deleted at that unit's merge and recoverable at
`git show ec7f7770b569fb3c3aa3c87e086f66170166712c:docs/DOCM-9-SPEC.md`
(`docs/DOC-LEDGER.md`, the per-merge deletion entry) — the paths this
row used to cite, `docs/DOCM-9-SPEC.md` and
`work/docm/certify-locally-valid-range-instead-of-sampling`, both left
the tree with DOCM.

**Where the tree stands today.** `crates/viewer/src/bounds.rs`'s
module header states the query, its three discharged kernel doors and
the cost that keeps the probe interactive, so a reader of the module
is no longer told the certified answer is blocked on kernel work. The
PANEL is still sampling-only: the reading comes from
`Session::bounds()` through `crate::bounds::probe`, and nothing in
`crates/viewer` calls `certified_range` on a user's behalf. The one
reach to it in this crate is the row
`crates/viewer/tests/docm9_range_vs_probe.rs`, which measures the two
answers against each other on one document and is behind the
`interval` feature, so a default-feature build of this crate does not
touch the query at all.

What is missing is the affordance, in `crates/viewer`:

- a "certify" control on the bounds panel, off the interaction path
  (the query is an interval drive — seconds, not a frame);
- seeded by the probe's own bracket, since the query does not choose a
  seed (`RangeSeed { lo, hi }` are OFFSETS from the field's current value,
  so a probe bracket converts without a subtraction at the axis);
- the reading replaced when it returns, with the two answers never
  silently merged: a certified range is a SUBSET of every
  locally-valid range, so a panel shows both or names which one it is
  showing.

The signature, verbatim:

```rust
pub fn certified_range(
    doc: &Doc<ProfileProgram>,
    field: &RangeField,          // Param(ParamName) | Slot { node, slot }
    seed: RangeSeed,                  // offsets, lo <= 0 <= hi
    config: &DriveConfig,        // the caller's budget
    tol: Tol,
) -> Result<CertifiedRange, RangeRefusal>;
```

`CertifiedRange` carries `lo()` and `hi()`, each a `RangeSide` with
four arms: `Certified { to }` (the seed's edge — never "unbounded"),
`NewFailure`/`DecisionFlip` (a bracket `within` around a boundary,
with the driver's evidence), and `Indeterminate` (the driver could not
decide — **not a bound, and never to be rendered as one**). The panel
that renders `Indeterminate` as an edge is the specific mistake this
row exists to prevent.

**The two doors already agree on what they will not answer**, so the
affordance does not have to invent an admission rule: an
expression-driven slot is refused by name on both sides —
`Session::guard_driven` raises `Refusal::DrivenByExpression` before
the probe seeds anything, and `certified_range` raises
`RangeRefusal::SlotIsNotALiteral`, whose message says to certify the
parameters that expression reads instead. (The probe half of that was
`work/chrome/probe-bounds-lacks-driven-slot-guard`, closed 2026-09-04
at PR 1746; this row used to call it adjacent and open.) What a
"certify" control still owes is the ten other `RangeRefusal` arms,
which have no probe counterpart at all: the seed's own
(`SeedNotABracket`, `SeedIsNotTheAnalyzedAxis`,
`MoreThanOneAxisVaries`), the field's (`NotAContinuousParam`,
`UnknownNode`, `UnknownSlot`, `StructuralSlot`), the derivation's
(`Derivation`), the driver's own `Drive`, and
`LeavesAreNotAPartition`, which is the fail-loud door on a driver
change rather than a case today's driver reaches.

## Three shapes the panel has to render correctly

1. **`within` is not a valid interval.** It is a bracket around a
   boundary whose interior the driver did not decide, and today it
   routinely CONTAINS values at which the document does not build (a
   leaf whose node definitely fails is priced `Budget` rather than
   named — `work/props/coincidence-zone-priced-budget-at-the-floor`).
   Shading it as "still fine" is the specific wrong reading.
2. **Both sides can report ONE span crossing the current value.** The
   leaf holding the nominal belongs to both walks, so when it is the
   leaf reported, `lo` and `hi` carry the same straddling span and
   `certified_to` is zero on both: nothing either side was proven. A
   panel drawing two half-ranges from that draws a range that is not
   there.
3. **`Indeterminate` with `Budget` may be a failure at the floor.**
   Nothing at the type tells an exhausted budget from a boundary the
   driver cannot name, so "try a bigger budget" is honest advice only
   above the floor; at the shipped depth, or on `Budget(Resolution)`,
   more budget buys nothing.

## What it costs, so the affordance is designed for the real number

Measured at `docm/9-certified-range`: ~3.4 s PER LEAF on the corpus
plate's `hole_r` and ~17 s per leaf on the die's cube slot, certifying
nothing at either. So the control is a long-running query with a
progress and a cancel, not a button that returns; the budget belongs
in the panel (a wall-clock target the panel converts to `max_leaves`),
and `DriveConfig::default()` must not be what it sends — its 65,536
leaves are days.

## Home

CHROME owns `crates/viewer`.

## Re-priced E → H (2026-09-21, orchestrator)

`cost: E` was wrong and the row's own body is the evidence against it.
What it asks for is a long-running query with a progress indicator and
a cancel, a panel-side budget converted from a wall-clock target to
`max_leaves` (because `DriveConfig::default()`'s 65,536 leaves are
days), a reading that REPLACES the probe's without ever silently
merging the two, three enumerated shapes the panel must render
correctly, and eleven `RangeRefusal` arms with no probe counterpart at
all. Measured cost of one query on the corpus is 3.4 s to 17 s per
leaf, certifying nothing.

That is a design pass and a unit, not a lane brief. Nothing about the
finding changes; only the price does, so that the board stops offering
it as cheap work.

## Design (2026-11-22)

What follows was weighed by two designers and a further designer (`work/offer/log.md`).

**The kernel query changes first.** `editor_core::range` gains a resumable search that refines outward from the field's current value and keeps every finished step:
- It splits at the current value first. On each side it replays the interval whole, and where the result is undecided it halves and replays both halves, nearest the current value first. Because the far half is replayed too, the search looks past undecided ground and can find a proven difference beyond it.
- Its answer after n steps is a pure function of the inputs and n, so stopping keeps the answer. Ground it has not reached is priced as budget (E6).
- It takes the session's part resolver (`work/scalar/the-box-driver-carries-no-part-resolver` is the same gap in `drive`).
- `certified_range` becomes a wrapper that runs the search to completion. `drive` is unchanged.

**The answer type** states three things and nothing shaped like a second interval:
- a proven interval around the current value, possibly empty, derived from the two sides;
- per side, either "reached the seed's end" or "proven different at a point", carrying the driver's evidence, with its kind derived from that evidence;
- per side, "not decided past x", carrying the typed reason. "Can more proving help" is derived from the reason.

The `within` pair and the straddling span go.

**The viewer control.**
- **Prove** sits on a sampled reading. The seed is the range the reading states, as exact offsets carried on the reading: on each side, the furthest value the probe found valid, or as far as it looked where nothing failed.
- One kernel admission door, which runs no geometry, greys the button before the click.
- The search runs on its own seam with a stop, and its busy state is separate from `Session::busy()`. An edit, a new probe or a value gesture cancels it and drops its result.
- Every number is rendered through `props::shown_text`.
- The undecided ground between a proven end and a point proven different is never called valid. Outside the sampled range the proof says nothing.

**When the proof replaces the sampled reading:** when both sides reach the seed's ends, the line reads "proven c…d". Otherwise it reads "proven a…b, of the sampled c…d", then says what stopped each side.

## Decided (Ev, 2026-11-22, on PR #7340)

The sampling probe goes. `range?` was never meant to sample, and nothing is to be kept solely to support it. `range?` becomes the certified query, rebuilt as the kernel search described in `## Design` above. The `## Design` section's kernel half stands as written. Its viewer half is replaced by what follows.

- **The search.** `range?` on a slot or variable starts the certified search on its own seam. It runs off the frame, and its busy state is separate from `Session::busy()`.
  - While it runs, the line reads "proving… proven a…b so far · stop".
  - Stop keeps what has been proven. An edit cancels the run and drops its result.
- **The seed** is how far the search looks. No sampled reading exists to supply one. It comes from the field's own admissible domain where the kernel states one, such as a length's lower bound. Otherwise it is a fixed span relative to the current value, chosen in the unit and stated on the reading ("checked to …").
  - The directed search spends steps near the current value first, so a wide seed costs little.
  - The reading never says "unbounded".
- **The readings, and their wording.** Every number goes through `props::shown_text`.
  - "proven a…b";
  - per side, one of:
    - "at y it builds, differently";
    - "node stops building at y";
    - "not decided past x", followed by "more proving may go further" or "more time will not help";
  - "nothing proven around v";
  - "holding names at their current values".
- **Admission.** One kernel admission door, which runs no geometry, greys `range?` with the refusal's own words.
- **What is deleted.** The sampling probe: `viewer::bounds`'s probe and `BoundsProbe`, `Session::probe_bounds`, `SessionOp::ProbeBounds` as a sampler, and `crates/viewer/tests/docm9_range_vs_probe.rs`. `work/offer/the-bounds-probe-runs-inline-on-the-frame` therefore has nothing left to fix.

## What the kernel unit hands this one (2026-11-22)

From `the-certified-range-refines-away-from-the-current-value`; the surface is `editor_core::range`, and its module docs are the reference.

- **The search**: `RangeSearch::new(doc, field, seed, budget, resolver, stop, tol)`, `step(&Stop) -> Step`, `answer()`. One step is one replay; `Step::Stopped` leaves the answer where it stood and a later step continues it, so a cancel token on the step's `Stop` is the stop control.
- **Admission**: `admit(doc, field, seed, built)` runs no geometry. Handed the session's own finished `f64` build as `built`, it also refuses a document that does not build now, in the witness's words, and a build of another document (`EvaluationOfAnotherDocument`).
- **The seed**: `RangeSeed::of(doc, field)` is `±|current value|`, and `±1` at zero. No field of the kernel states an admissible domain, so there is no domain to read; the answer states the seed (`CertifiedRange::seed`) for "checked to …".
- **The budget is `RangeBudget`, the search's own; pass `RangeBudget::for_seed(seed, tol, usize::MAX)` and stop the search with the step's `Stop`.**
  - `max_depth` is halvings of one side. It bounds a side only until that side's first proof: after it, an end of the answer stops being refined `STALL_LEVELS` (4) halvings past the width that last moved it, whatever the depth. A deeper bound costs two replays a level on a side that never proves anything.
  - So the depth is sized to reach the width a curved field certifies at, near `ε/4`: `for_seed` reaches `ε/16` on the wider side, about 32 halvings for the plate's `hole_r` under `RangeSeed::of`. At the old default of 24 that field proves nothing.
  - The `f64` grid is no bound near the current value, so a depth of 64 is not free on a side that never proves.
  - `max_replays` is a cap, and the search is stepped, so the panel's stop is the real budget: a field that certifies only near `ε` grows its proof by about that much every two replays and never finishes a wide seed. `Step::Finished` means the answer can no longer move.
- **The readings**: `RangeSide::proven_to`, `reached_seed_end`, `different()` (`at`, `kind()`: `StandingChange { node }` or `BuildsDifferently`), `not_decided()` (`past`, `reason()`: `Unproven::Unfinished { replays }`, `Stalled { width }` or the driver's `Floor(reason)`, and `more_time_helps()`, true only for `Unfinished`); `CertifiedRange::nothing_proven`, `pinned`, and `proven_interval`, whose absolute ends are rounded toward the nominal. Render the ends from `proven_interval`, not from `absolute` of the offsets.
- **What a replay costs** is in `range.rs`'s module docs and on `work/scalar/a-replay-that-nearly-certifies-costs-fifty-times-one-that-stops-early`: seconds while the interval is wide, 20 s to 2 min near a width that certifies. The first proof on the plate arrives after about 47 replays from a seed of `±1e-6`, and later from the chosen one.
- `crates/viewer/tests/docm9_range_vs_probe.rs` and `viewer::bounds`'s module docs were kept compiling and true on the new answer type, nothing more.
