---
id: certify-affordance-on-the-bounds-panel
kind: issue
title: The bounds panel has no on-demand certify affordance for the certified range query
status: open
opened: 2026-09-13
refs: [1183]
---

Filed by DOCM-9, which built the kernel side and is forbidden by its
own fence from building either consumer door.

`editor_core::range::certified_range` answers, on demand, the question
`viewer::bounds` answers by sampling — over which interval around a
field's current value is the document PROVABLY the same build. Ev's
2026-09-13 ruling on pacing (`docs/DOCM-9-SPEC.md`, and the closed
`work/docm/certify-locally-valid-range-instead-of-sampling`) is that
the sampling probe STAYS the interactive answer and the certified
range is an on-demand query whose result arrives later and REPLACES
the probe's reading.

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

`work/chrome/probe-bounds-lacks-driven-slot-guard` is adjacent: the
query refuses an expression-driven slot typed
(`RangeRefusal::SlotIsNotALiteral`), which is the admission rule that
row asks the probe for.

## Home

CHROME owns `crates/viewer`.
