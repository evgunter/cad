---
id: certify-affordance-on-the-bounds-panel
kind: issue
title: The bounds panel has no on-demand certify affordance for the certified range query
status: open
opened: 2026-09-13
refs: [1183]
priority: P1
cost: E
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
"certify" control still owes is the eleven other `RangeRefusal` arms,
which have no probe counterpart at all: the seed's own
(`SeedNotABracket`, `SeedIsNotTheAnalyzedAxis`,
`MoreThanOneAxisVaries`), the field's (`NotAContinuousParam`,
`UnknownNode`, `UnknownSlot`, `StructuralSlot`), the derivation's
(`SyntheticNameTaken`, `Derivation`), the driver's own `Drive`, and
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
